use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct EditorColors {
    theme_type: String,
    primary: String,
    foreground_main: String,
    background_main: String,
}

#[derive(Deserialize)]
pub struct TokenColors {
    function: String,
    type_like: String,
    strings: String,
}

#[derive(Deserialize)]
pub struct ColorsConfig {
    name: String,
    editor_colors: EditorColors,
    token_colors: TokenColors,
}

pub fn read_color_files(
    root_folder: &Path,
) -> Result<Vec<ColorsConfig>, Box<dyn std::error::Error>> {
    let configs_folder = root_folder.join("colors");
    let mut result = vec![];
    for file in fs::read_dir(configs_folder)? {
        let path = file?.path();
        let contents = fs::read_to_string(path)?;
        result.push(toml::from_str(&contents)?);
    }
    Ok(result)
}
