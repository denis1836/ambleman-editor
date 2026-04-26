mod log;
mod metadata;
mod ui;

use crate::log::log;
use crate::ui::run_ui;

///Main function that starts the application.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    log('I', "Attempting to start...");

    if let Err(e) = run_ui() {
        log('E', &format!("Failed to start the application: {}", e));
        eprintln!("Failed to start the application: {}", e);
        return Err(e);
    }

    log('I', "Application started successfully");

    Ok(())
}
