use std::process::Command;
use color_thief::ColorFormat;
use crate::desktop::DesktopEnvironment;
use image::GenericImageView;


// Detect the wallpaper path of the current desktop environment
pub fn detect_wallpaper(de: &str) -> Option<String> {

    let trim_string_gnome = |schema:&str , key: &str| -> String {
        DesktopEnvironment::get_gsettings(schema, key).strip_prefix("file://").unwrap_or_default().to_string()
    };

    let read_kde_wallpaper = || -> Option<String> {
        let home = std::env::var("HOME").ok()?;
        let plasma5 = format!("{}/.config/plasma-org.kde.plasma.desktop-appletsrc", home);
        let plasma6 = format!("{}/.config/kde.org/plasma-org.kde.plasma.desktop-appletsrc", home);

        let mut result= None;

        let parse_output = |output: String| -> Option<String> {
            output.lines()
                .find(|line| line.starts_with("Image="))?
                .split("=").nth(1)
                .map(|s| s.trim().strip_prefix("file://").unwrap_or(s.trim()).to_string())
        };

        if let Ok(output) = std::fs::read_to_string(plasma5) {
            result = parse_output(output);
        }
        else if let Ok(output) = std::fs::read_to_string(plasma6) {
            result = parse_output(output);
        }

        result

    };

    match de {
        "GNOME" => {
            let scheme = DesktopEnvironment::get_gsettings("org.gnome.desktop.interface", "color-scheme");
            if scheme.contains("dark") {
                let result = trim_string_gnome("org.gnome.desktop.background", "picture-uri-dark");
                Some(result)
            }
            else {
                let result = trim_string_gnome("org.gnome.desktop.background", "picture-uri");
                Some(result)
            }
        },
        "Cinnamon" => Some(trim_string_gnome("org.cinnamon.desktop.background", "picture-uri")),
        "Mate" => Some(DesktopEnvironment::get_gsettings("org.mate.background", "picture-filename")),
        "KDE" => read_kde_wallpaper(),
        "XFCE" => {
            if let Ok(output) = Command::new("xfconf-query").args(["-c", "xfce4-desktop", "-lv"]).output() {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                return stdout.lines()
                    .find(|line| line.contains("last-image"))
                    .and_then(|line| line.split_whitespace().last())
                    .map(|s| s.trim().to_string());
            }
            None
        }
        _ => None


    }
}
// Extract the palette of the dominant color of the wallpaper
pub fn extract_palette(wallpaper: &str) -> Vec<color_thief::Color> {
    if let Ok(wp) = image::open(wallpaper) {
       let pixels = wp.to_rgb8().into_raw(); //convert the image to RGB8 raw pixels
        if let Ok(result) = color_thief::get_palette(&pixels, color_thief::ColorFormat::Rgb, 5, 4) {
            return result;
        }
    }
    vec![]
}

pub fn correct_brightness(palette: &[color_thief::Color]) -> Vec<color_thief::Color> {
    palette.into_iter()
        .map(|color| {
            let  r = color.r as f32;
            let  g = color.g as f32;
            let  b = color.b as f32;

            let luminance = 0.299 * r  + 0.587 * g  + 0.114 * b ;

            if luminance < 60.0 && luminance > 0.0 {
                let luminance_factor = 60.0 / luminance;

               color_thief::Color { // color is not bright enough, correct it. As color is not mutable, we return a new color
                   r: (r * luminance_factor).min(255.0) as u8,
                   g: (g * luminance_factor).min (255.0) as u8,
                   b: (b * luminance_factor).min(255.0) as u8,
               }
            }
            else {
                *color //return a copy of the original color
            }

        } )
        .collect()
}


