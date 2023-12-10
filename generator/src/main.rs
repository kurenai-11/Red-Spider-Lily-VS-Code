use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
struct EditorColors {
    theme_type: String,
    primary: String,
    foreground_main: String,
    background_main: String,
}

#[derive(Deserialize)]
struct TokenColors {
    function: String,
    type_like: String,
    strings: String,
}

#[derive(Deserialize)]
struct ColorsConfig {
    name: String,
    editor_colors: EditorColors,
    token_colors: TokenColors,
}

fn read_color_files(root_folder: &Path) -> Result<Vec<ColorsConfig>, Box<dyn std::error::Error>> {
    let configs_folder = root_folder.join("colors");
    let mut result = vec![];
    for file in fs::read_dir(configs_folder)? {
        let path = file?.path();
        let contents = fs::read_to_string(path)?;
        result.push(toml::from_str(&contents)?);
    }
    Ok(result)
}

fn main() {
    let dir = std::env::current_dir().expect("should be ableto get the working directory");
    let project_root = dir
        .parent()
        .expect("should be able to get the parent directory");
    let themes_folder = project_root.join("themes-test");
    let color_theme_configs = read_color_files(project_root);
    match color_theme_configs {
        Ok(val) => println!("{}", val[0].token_colors.function),
        Err(err) => println!("{err}"),
    }
}
