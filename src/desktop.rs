use std::process::Command;

// Group all the relevant fields of the DE into a struct
#[derive(Debug)]
pub struct DesktopEnvironment {
    pub name: String,
    pub display_server: String,
    pub wm: String,
    pub theme: String,
    pub icon_theme: String,
    pub cursor_theme: String,
    pub cursor_size: String,
    pub font: String,
    pub wm_theme: String,
}


impl DesktopEnvironment {
    pub fn new() -> DesktopEnvironment {
        match Self::read_desktop_environment().as_str() {
            "GNOME" => DesktopEnvironment {
                name: "GNOME".to_string(),
                display_server: Self::read_display_server(),
                wm: "Mutter".to_string(),
                theme: Self::get_gsettings("org.gnome.desktop.interface", "gtk-theme"),
                icon_theme: Self::get_gsettings("org.gnome.desktop.interface", "icon-theme"),
                cursor_theme: Self::get_gsettings("org.gnome.desktop.interface", "cursor-theme"),
                cursor_size: Self::get_gsettings("org.gnome.desktop.interface", "cursor-size"),
                font: Self::get_gsettings("org.gnome.desktop.interface", "font-name"),
                wm_theme: Self::get_gsettings("org.gnome.desktop.wm.preferences", "theme"),
            },
            "KDE" => DesktopEnvironment {
                name: "KDE".to_string(),
                display_server: Self::read_display_server(),
                wm: "KWin".to_string(),
                theme: Self::kde_config_get("kdeglobals", "General", "ColorScheme"),
                icon_theme: Self::kde_config_get("kdeglobals", "Icons", "Theme"),
                cursor_theme: Self::kde_config_get("kcminputrc", "Mouse", "cursorTheme"),
                cursor_size: Self::kde_config_get("kcminputrc", "Mouse", "cursorSize"),
                font: Self::kde_config_get("kdeglobals", "General", "font"),
                wm_theme: Self::kde_config_get("kwinrc", "org.kde.kdecoration2", "theme"),
            },
            "XFCE" => DesktopEnvironment {
                name: "XFCE".to_string(),
                display_server: Self::read_display_server(),
                wm: "Xfmw4".to_string(),
                theme: Self::xfconf_query_get("xsettings", "/Net/ThemeName"),
                icon_theme: Self::xfconf_query_get("xsettings", "/Net/IconThemeName"),
                cursor_theme: Self::xfconf_query_get("xsettings", "/Gtk/CursorThemeName"),
                cursor_size: Self::xfconf_query_get("xsettings", "/Gtk/CursorThemeSize"),
                font: Self::xfconf_query_get("xsettings", "/Gtk/FontName"),
                wm_theme: Self::xfconf_query_get("xfwm4", "/general/theme")
            },
            "X-Cinnamon" => DesktopEnvironment {
                name: "Cinnamon".to_string(),
                display_server: Self::read_display_server(),
                wm: "Muffin".to_string(),
                theme: Self::get_gsettings("org.cinnamon.desktop.interface", "gtk-theme"),
                icon_theme: Self::get_gsettings("org.cinnamon.desktop.interface", "icon-theme"),
                cursor_theme: Self::get_gsettings("org.cinnamon.desktop.interface", "cursor-theme"),
                cursor_size: Self::get_gsettings("org.cinnamon.desktop.interface", "cursor-size"),
                font: Self::get_gsettings("org.cinnamon.desktop.interface", "font-name"),
                wm_theme: Self::get_gsettings("org.cinnamon.desktop.wm.preferences", "theme"),
            },
            "MATE" => DesktopEnvironment {
                name: "Mate".to_string(),
                display_server: Self::read_display_server(),
                wm: "Marco".to_string(),
                theme: Self::get_gsettings("org.mate.interface", "gtk-theme"),
                icon_theme: Self::get_gsettings("org.mate.interface", "icon-theme"),
                cursor_theme: Self::get_gsettings("org.mate.peripherals-mouse", "cursor-theme"),
                cursor_size: Self::get_gsettings("org.mate.peripherals-mouse", "cursor-size"),
                font: Self::get_gsettings("org.mate.interface", "font-name"),
                wm_theme: Self::get_gsettings("org.mate.marco.general", "theme"),
            },
            _ => DesktopEnvironment {
                name: "Unknown".to_string(),
                display_server: Self::read_display_server(),
                wm: "Unknown".to_string(),
                theme: "Unknown".to_string(),
                icon_theme: "Unknown".to_string(),
                cursor_theme: "Unknown".to_string(),
                cursor_size: "Unknown".to_string(),
                font: "Unknown".to_string(),
                wm_theme: "Unknown".to_string(),
            }
        }


    }

    pub fn to_lines(&self) -> Vec<String> {
        vec![
            format!("DE: {} ({})", self.name, self.display_server),
            format!("WM: {} ({})", self.wm, self.display_server),
            format!("WM Theme: {}", self.wm_theme),
            format!("Theme: {}", self.theme),
            format!("Icons: {}", self.icon_theme),
            format!("Cursor Theme: {} ({}px)", self.cursor_theme, self.cursor_size),
            format!("Font: {}", self.font),
        ]
    }

    // Returns the desktop environment used and the display server
    pub fn read_desktop_environment() -> String {


        let desktop = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_|std::env::var("DESKTOP_SESSION") //if it's not set, check for DESKTOP_SESSION
                .unwrap_or_else(|_| "Unknown".to_string()));

        desktop.to_string()
    }

    // Returns the display server used
    pub fn read_display_server() -> String {

        // This project focuses on traditional desktop environments,
        // so the only possible display servers are Wayland and X11

        let display = std::env::var("WAYLAND_DISPLAY")
            .map(|_| "Wayland").unwrap_or_else(|_| "X11"); //if it's not set, it's X11

        display.to_string()
    }

    // helper function to get the value of a gsettings key
    pub fn get_gsettings(schema: &str, key: &str) -> String {
        let  result;
        if let Ok(output) = Command::new("gsettings").args(["get", schema, key]).output() {
            result = String::from_utf8_lossy(&output.stdout).trim().
                trim_matches(|c| c == '\'' || c == '\"').to_string();
        }
        else {
            return "Unknown".to_string();
        }

        if result.is_empty() {
            return "None".to_string();
        }
        result
    }
    // helper function to get the value of a xfconf key (for XFCE)
   pub fn xfconf_query_get(channel: &str, property: &str) -> String {
        let result;
        if let Ok(output) = Command::new("xfconf-query").args(["-c", channel, "-p", property]).output() {
            result = String::from_utf8_lossy(&output.stdout).trim().to_string();
        }
        else {
            return "Unknown".to_string();
        }

        if result.is_empty() {
            return "None".to_string();
        }

        result
    }
    // helper function to get the value of a kde config key (for KDE)
   pub fn kde_config_get(file: &str, section: &str,key: &str) -> String {
        let mut result;
        for cmd in &["kreadconfig6", "kreadconfig5"] {
            if let Ok(output) = Command::new(cmd).args(["--file", file, "--group", section, "--key", key]).output() {
                result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !result.is_empty() {
                    return result;
                }
            }
        }
        return "None".to_string();
    }

}