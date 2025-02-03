use log::{info, warn, error};
use chrono::Utc;

pub struct Logger;

impl Logger {
    pub fn log_info(message: &str) {
        info!("[{}] INFO: {}", Utc::now(), message);
    }

    pub fn log_warning(message: &str) {
        warn!("[{}] WARNING: {}", Utc::now(), message);
    }

    pub fn log_error(message: &str) {
        error!("[{}] ERROR: {}", Utc::now(), message);
    }
}
