pub fn set_build_date() {
    let (year, month, day) = calculate_build_date();
    println!(
        "cargo:rustc-env=BUILD_DATE={:04}-{:02}-{:02}",
        year, month, day
    );
}

fn calculate_build_date() -> (u64, u64, u64) {
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    let secs = duration.as_secs();

    let days_since_epoch = secs / 86400;
    let years_since_1970 = days_since_epoch / 365;
    let year = 1970 + years_since_1970;

    // Approximate month and day (good enough for build date display)
    let days_in_year = days_since_epoch % 365;
    let month = (days_in_year / 30) + 1;
    let day = (days_in_year % 30) + 1;

    (year, month, day)
}
