use std::{fs, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct EditorColors {
    pub theme_type: String,
    pub primary: String,
    pub foreground_main: String,
    pub background_main: String,
}

#[derive(Deserialize)]
pub struct TokenColors {
    pub function: String,
    pub type_like: String,
    pub strings: String,
}

#[derive(Deserialize)]
pub struct ColorsConfig {
    pub name: String,
    pub editor_colors: EditorColors,
    pub token_colors: TokenColors,
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
