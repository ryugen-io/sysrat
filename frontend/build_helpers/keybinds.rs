use std::path::PathBuf;

/// Load keybinds configuration with XDG compliance support.
///
/// Order of precedence:
/// 1. User-specified keybinds file (USER_KEYBINDS_FILE env var)
/// 2. Default frontend/keybinds.toml
///
/// The selected file path is set as KEYBINDS_FILE env var for
/// embedding into the WASM binary. Path must be relative to
/// frontend/src/keybinds/mod.rs (where include_str! is called).
pub fn load_keybinds_config() {
    // Path relative to src/keybinds/mod.rs where include_str! is called
    let default_path = "../../keybinds.toml";

    // Try user-specified keybinds file first
    if let Ok(user_keybinds) = std::env::var("USER_KEYBINDS_FILE") {
        let expanded_path = expand_tilde(&user_keybinds);

        if expanded_path.exists() {
            println!(
                "cargo:warning=Using user keybinds: {}",
                expanded_path.display()
            );
            println!("cargo:rustc-env=KEYBINDS_FILE={}", expanded_path.display());
            println!("cargo:rerun-if-changed={}", expanded_path.display());
            return;
        }
    }

    // Fall back to default keybinds.toml
    println!("cargo:warning=Using default keybinds: keybinds.toml");
    println!("cargo:rustc-env=KEYBINDS_FILE={}", default_path);
    println!("cargo:rerun-if-changed=keybinds.toml");
}

/// Expand tilde (~/) in path to HOME directory.
fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/")
        && let Ok(home) = std::env::var("HOME")
    {
        return PathBuf::from(home).join(stripped);
    }
    PathBuf::from(path)
}
