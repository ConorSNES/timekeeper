use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/** Represents persistent application state, including user preferences. */
#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    timestamp: Option<DateTime<Local>>,
    borderless: bool,
    display_12hr: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            timestamp: None,
            borderless: false,
            display_12hr: false,
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

    pub fn get_12hr(&self) -> bool {
        self.display_12hr
    }

    pub fn toggle_12hr(&mut self) {
        self.display_12hr = !self.display_12hr;
    }

    pub fn timestamp_format_string(&self) -> &str {
        match self.get_12hr() {
            false => "%H:%M:%S",
            true => "%I:%M:%S %P"
        }
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
