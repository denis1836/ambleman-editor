use chrono;
use colored::{ColoredString, Colorize};
use std::io::Write;

pub static LOG_FILE: &str = "./logs/ambleman_editor.log";

///Simple logging function that writes messages to a log file with a timestamp and message type (INFO, WARN, ERROR).
pub fn log(p_ms_type: char, p_message: &str) {
    let timestamp = chrono::Local::now().format("%d.%m.%Y %H:%M:%S");
    let ms_type: ColoredString = match p_ms_type {
        'I' => "INFO".normal(),
        'W' => "WARN".yellow(),
        'E' => "ERROR".red(),
        _ => "UNKNOWN".white(),
    };

    let log_entry = format!("[{}] [{}] {}", timestamp, ms_type, p_message);

    if let Err(e) = std::fs::create_dir_all("./logs") {
        eprintln!("Couldn't create log directory: {}", e);
        return;
    }
    if !std::fs::File::open(LOG_FILE).is_ok() {
        if let Err(e) = std::fs::File::create(LOG_FILE) {
            eprintln!("Couldn't create log file: {}", e);
            return;
        }
    }

    let mut file: std::fs::File = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .expect("Unable to open log file");

    if let Err(e) = writeln!(file, "{}", log_entry) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
