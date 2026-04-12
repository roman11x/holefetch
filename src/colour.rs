use std::process::Command;
use crate::desktop::DesktopEnvironment;
use image::ImageReader;
use colored::Colorize;

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
    if let Ok(reader) = ImageReader::open(wallpaper) {
        if let Ok(reader) = reader.with_guessed_format() {
            if let Ok(wp) = reader.decode() {
                let pixels = wp.to_rgb8().into_raw();
                if let Ok(result) = color_thief::get_palette(&pixels, color_thief::ColorFormat::Rgb, 5, 4) {
                    return result;
                }
            }
        }
    }
    vec![]
}
/*
color-thief extracts dominant colors from a wallpaper.
“Dominant” means most frequently occurring or most visually significant — not “most readable on a terminal.”
A dark wallpaper like a moonlit scene or a dark anime panel will have dominant colors with very low luminance — dark blues, dark grays, near-blacks.
If you use those colors for your label text (OS:, CPU:, etc.) on a standard dark terminal background, the text will be nearly invisible.

The luminance check allows us to detect this situation and correct for it before the colors reach the terminal.
 */
pub fn correct_brightness(palette: &[color_thief::Color], brightness: f32) -> Vec<color_thief::Color> {
    palette.into_iter()
        .map(|color| {
            let  r = color.r as f32;
            let  g = color.g as f32;
            let  b = color.b as f32;

            let luminance = 0.299 * r  + 0.587 * g  + 0.114 * b ; // The result is between 0 (black, no brightness) and 255 (white, full brightness).

            if luminance < 100.0 && luminance > 0.0 {
                let luminance_factor = (100.0 / luminance) * brightness;

               color_thief::Color { // color is not bright enough, correct it. As color is not mutable, we return a new color
                   r: (r * luminance_factor).min(255.0) as u8,
                   g: (g * luminance_factor).min (255.0) as u8,
                   b: (b * luminance_factor).min(255.0) as u8,
               }
            }
            else {
                //return a copy of the original color multiplied by the brightness factor
                let r = (color.r as f32 * brightness).min(255.0) as u8;
                let g = (color.g as f32 * brightness).min(255.0) as u8;
                let b = (color.b as f32 * brightness).min(255.0) as u8;
                color_thief::Color {r, g, b}
            }

        } )
        .collect()
}
// substitute the placeholders in the logo with the dominant colors of the wallpaper
// the placeholders are in the form ${n} where n is a number between 1 and 9
// ${1} will be replaced with the dominant index 0 of the dominant color palette and so on
pub fn substitute_placeholders(logo: &str, palette: &[color_thief::Color]) -> String {
    let mut result = String::new();
    let mut chars = logo.chars().peekable();

    if let Some(color) = palette.first() {  // This sets palette[0] as the default color from the very beginning,
                                                    // so lines without any $N placeholder still get colored.
        result.push_str(&format!("\x1b[38;2;{};{};{}m", color.r, color.g, color.b));
    }

    while let Some(c) = chars.next() {
        if c!='$' { // if the current character is not a $, it will not be followed by a digit, so just append it to the string
            result.push(c);
            continue;
        }
        match chars.peek() { // check if the next character is a digit
            Some(&'$') => {
                result.push('$');
                chars.next();
            },
            Some(&digit) if digit.is_ascii_digit() => { // if the next character is a digit in the range 1-9
                    let digit_char = chars.next().unwrap(); //it is guaranteed that the next character exists
                    let n = digit_char.to_digit(10).unwrap() as usize; // it is guaranteed that the digit is in the range 1-9
                    if n > 0 && n <= palette.len() {
                        let color = palette[n-1];
                        result.push_str(&format!("\x1b[38;2;{};{};{}m", color.r, color.g, color.b));
                    }
                }
            _ => {result.push('$');} // if the next character is not a digit, just append a $ to the string
            }

        }
    result.push_str("\x1b[0m"); //append a color reset to the end of the string
    result
    }

pub fn colorize_label(label: &str, palette: &[color_thief::Color]) -> String {
    if palette.len() >= 3 {
        let color = palette[2];
        let r = color.r;
        let g = color.g;
        let b = color.b;
        let padded = format!("{:<15}", label); // pad the label
        let text = padded.truecolor(r,g,b).to_string();
        return text
    }
    label.to_string()
}

pub fn colorize_value(value: &str, palette: &[color_thief::Color]) -> String {
    if palette.len() >= 4 {
        let color = palette[3];
        let r = color.r;
        let g = color.g;
        let b = color.b;
        let text = value.truecolor(r,g,b).to_string();
        return text
    }
    value.to_string()
}






