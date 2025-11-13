// Build metadata and dependency versions - extracted automatically at build time
const BUILD_DATE: &str = env!("BUILD_DATE");
const RATZILLA_VERSION: &str = env!("RATZILLA_VERSION");
const RATATUI_VERSION: &str = env!("RATATUI_VERSION");
const AXUM_VERSION: &str = env!("AXUM_VERSION");

pub fn tech_stack_info() -> String {
    format!(
        "built on: {} | Ratzilla v{} | Ratatui v{} | Axum v{}",
        BUILD_DATE, RATZILLA_VERSION, RATATUI_VERSION, AXUM_VERSION
    )
}
