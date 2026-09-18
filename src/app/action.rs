use chrono::{DateTime, Local};


/** Represents deferred, memory safe execution of one action. */
#[derive(Debug, Clone)]
pub enum AppAction {
    None,
    TimestampAbsRequest(String),
    TimestampRelRequest(String),
    SetTimestamp(DateTime<Local>),
    ResTimestamp,
    Toggle12Hour,
    RefreshBorderless,
    Exit
}

impl Default for AppAction {
    fn default() -> Self {
        Self::None
    }
}