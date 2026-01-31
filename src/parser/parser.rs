use crate::settings::settings::Settings;
use serde_json::Value;

pub fn parse_settings(path: &str) -> Result<Settings, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path);
    if contents.is_err() {
        return Err(Box::new(contents.err().unwrap()));
    }
    let parsed: Value = serde_json::from_str(&contents.unwrap())?;
    let music_paths = parsed["music_paths"]
        .as_array()
        .ok_or("music_paths should be an array")?
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    let volume_level = parsed["volume_level"]
        .as_u64()
        .ok_or("volume_level should be a u64")? as u8;
    let shuffle = parsed["shuffle"]
        .as_bool()
        .ok_or("shuffle should be a bool")?;
    Ok(Settings::new(music_paths, volume_level, shuffle))
}