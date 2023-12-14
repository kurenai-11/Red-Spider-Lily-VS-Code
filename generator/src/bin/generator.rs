use std::{fs, path::Path};
use utils::parsing::{read_color_files, ColorsConfig};

fn generate_color_theme() {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::current_dir().expect("should be able to get the working directory");
    let project_root = dir
        .parent()
        .expect("should be able to get the parent directory");
    let themes_folder = project_root.join("themes-test");
    let color_theme_configs = read_color_files(project_root)?;

    Ok(())
}
