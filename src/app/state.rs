use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/** Represents persistent application state, including user preferences. */
#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    timestamp: Option<DateTime<Local>>,
    borderless: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            timestamp: None,
            borderless: false,
        }
    }
}

impl AppState {
    pub fn get_borderless_mut(&mut self) -> &mut bool {
        &mut self.borderless
    }

    pub fn get_borderless(&self) -> bool {
        self.borderless
    }

    pub fn toggle_borderless(&mut self) {
        self.borderless = !self.borderless
    }

    pub fn get_timestamp(&self) -> Option<DateTime<Local>> {
        self.timestamp
    }

    pub fn set_timestamp(&mut self, timestamp: DateTime<Local>) {
        self.timestamp = Some(timestamp)
    }

    pub fn reset_timestamp(&mut self) {
        self.timestamp = None
    }
}
