use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use utils::{
    color_theme::{Colors, UIColors, VSCodeColorTheme},
    parsing::{read_color_files, ColorsConfig},
};

fn generate_color_theme() {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = std::env::current_dir().expect("should be able to get the working directory");
    let project_root = dir
        .parent()
        .expect("should be able to get the parent directory");
    let themes_folder = project_root.join("themes-test");
    let color_theme_configs = read_color_files(project_root)?;

    let colors = Colors::try_from(&color_theme_configs[0].editor_colors)?;

    let test_theme = VSCodeColorTheme::new("test1", colors);
    let j = serde_json::to_string(&test_theme)?;
    let mut file = File::create(themes_folder.join("test-color-theme.json"))?;
    file.write_all(j.as_bytes())?;

    Ok(())
}
