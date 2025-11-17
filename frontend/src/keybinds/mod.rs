mod help_text;
mod types;

pub use types::*;

impl Keybinds {
    pub fn load() -> Self {
        // Keybinds file is selected at build time from:
        // 1. User config: ~/.config/config-manager/keybinds.toml
        // 2. Default: frontend/keybinds.toml
        const KEYBINDS_TOML: &str = include_str!(env!("KEYBINDS_FILE"));
        toml::from_str(KEYBINDS_TOML).expect("Failed to parse keybinds.toml")
    }
}
