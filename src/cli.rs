use clap::Parser;
// Command line arguments
#[derive(Parser, Debug)]
#[command(name = "holefetch", about = "A system fetch tool with wallpaper-derived theming")]
pub struct Args {
       #[arg(long)] /// List all available logos
  pub  list_logos: bool,
       #[arg(long)] /// Set the logo to a specific logo, i.e. --set-logo archlinux
  pub  set_logo: Option<String>,
       #[arg(long)] /// Preview the fetch with a specific logo, i.e. --preview-logo archlinux
  pub  preview_logo: Option<String>,
       #[arg(long)] /// Set the wallpaper to a specific path, i.e. --set-wallpaper /path/to/wallpaper.png
  pub  wallpaper_path: Option<String>,
}
