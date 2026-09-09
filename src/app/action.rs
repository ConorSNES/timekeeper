
/** Represents deferred, memory safe execution of one action. */
#[derive(Debug, Clone)]
pub enum AppAction {
    None,
    SetTimestampAbs(String),
    SetTimestampRel(String),
    ResTimestamp,
    RefreshBorderless,
    Exit
}

impl Default for AppAction {
    fn default() -> Self {
        Self::None
    }
}