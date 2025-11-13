mod help_text;
mod types;

pub use types::*;

const KEYBINDS_TOML: &str = include_str!("../../keybinds.toml");

impl Keybinds {
    pub fn load() -> Self {
        toml::from_str(KEYBINDS_TOML).expect("Failed to parse keybinds.toml")
    }
}
