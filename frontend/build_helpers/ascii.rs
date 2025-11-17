use std::fs;
use std::path::PathBuf;

pub fn load_ascii_art() {
    embed_ascii_arts();
}

fn embed_ascii_arts() {
    const BLUE: &str = "\x1b[38;2;137;180;250m";
    const GREEN: &str = "\x1b[38;2;166;227;161m";
    const MAUVE: &str = "\x1b[38;2;203;166;247m";
    const NC: &str = "\x1b[0m";
    const INFO_ICON: &str = "\u{f05a}";
    const CHECK_ICON: &str = "\u{f00c}";

    let mut asciis = Vec::new();
    let mut default_count = 0;
    let mut user_count = 0;

    // Scan default ASCII arts from frontend/assets/
    if let Ok(entries) = fs::read_dir("assets") {
        for entry in entries.flatten() {
            if let Some(name) = get_ascii_name(&entry.path()) {
                asciis.push((name, entry.path()));
                default_count += 1;
            }
        }
    }

    // Scan user custom ASCII arts from USER_ASCII_DIR env var
    if let Ok(user_ascii_dir) = std::env::var("USER_ASCII_DIR") {
        let expanded_path = expand_tilde(&user_ascii_dir);

        if let Ok(entries) = fs::read_dir(&expanded_path) {
            for entry in entries.flatten() {
                if let Some(name) = get_ascii_name(&entry.path()) {
                    // Don't duplicate if ASCII name already exists
                    if !asciis.iter().any(|(n, _)| n == &name) {
                        asciis.push((name, entry.path()));
                        user_count += 1;
                    }
                }
            }
            if user_count > 0 {
                eprintln!(
                    "{}{}  {}[ascii] Found {} custom ASCII art(s) in {}{}",
                    GREEN,
                    CHECK_ICON,
                    NC,
                    user_count,
                    MAUVE,
                    expanded_path.display()
                );
            }
        }
    }

    eprintln!(
        "{}{}  {}[ascii] Embedded {} ASCII art(s) total ({} default + {} custom)",
        BLUE,
        INFO_ICON,
        NC,
        asciis.len(),
        default_count,
        user_count
    );

    // Set ASCII art file paths as env vars
    for (name, path) in &asciis {
        let env_name = format!("ASCII_FILE_{}", name.to_uppercase().replace('-', "_"));
        println!("cargo:rustc-env={}={}", env_name, path.display());
    }

    // Generate ASCII art names list (for runtime iteration)
    let ascii_names: Vec<&str> = asciis.iter().map(|(n, _)| n.as_str()).collect();
    println!("cargo:rustc-env=ASCII_NAMES={}", ascii_names.join(","));
}

fn get_ascii_name(path: &std::path::Path) -> Option<String> {
    if path.extension()? != "ascii" {
        return None;
    }
    path.file_stem()?.to_str().map(String::from)
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            PathBuf::from(home).join(stripped)
        } else {
            PathBuf::from(path)
        }
    } else {
        PathBuf::from(path)
    }
}
