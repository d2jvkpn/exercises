use crate::models::Course;
use chrono::{Local, SecondsFormat};
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub start_at: String,
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub courses: Mutex<Vec<Course>>,
}

impl Default for AppState {
    fn default() -> Self {
        let now = Local::now();

        Self {
            start_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
            health_check_response: "I'm good. You've already asked me".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        }
    }
}
