use chrono::{DateTime, Local, TimeDelta};
use eframe::egui::{Button, IntoAtoms, KeyboardShortcut, Response, Ui};

/**
 * check for a keybind being pressed
 */
pub fn check_bind(ui: &mut Ui, shortcut: &KeyboardShortcut) -> bool {
    ui.input_mut(|inp| inp.consume_shortcut(shortcut))
}

/**
 * draws a button with an associated keybind; does not merge shortcut state with output response
 */
pub fn draw_bind_button<'a>(ui: &mut Ui, label: impl IntoAtoms<'a>, bind: &KeyboardShortcut) -> Response {
    ui.add(
        Button::new(label).shortcut_text(ui.ctx().format_shortcut(bind))
    )
}

/**
 * combination of sign/absolute for an int,
 * for separating the sign and magnitude of a number
 */
fn sigabs(v: i64) -> (i64, i64) {
    (v.signum(), v.abs())
}

/** Alias for `chrono::offset::Local::now()`. Collects current local time. */
pub fn timenow() -> DateTime<Local> { chrono::offset::Local::now() }

/** Collects TimeDelta from specified time to time now. */
pub fn delta_now(from : DateTime<Local>) -> TimeDelta {
    from - timenow()
}

/**
 * Formats a `chrono::TimeDelta` in t[+|-]hh:mm:ss format.
 */
pub fn format_delta_time(delta : TimeDelta) -> String {
    let diff = delta;
    let hour = (diff.num_hours()).abs();
    let mins = (diff.num_minutes() % 60).abs();
    let (sign, secs) = sigabs((diff.num_seconds() % 60) % 60);
    format!(
        "t{}{:02}:{:02}:{:02}",
        if sign < 0 { "+" } else { "-" },
        hour,
        mins,
        secs
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    /** test basic use of sigabs  */
    fn test_sigabs() {
        const SAMPLES : &[(i64, i64, i64)] = &[
            (-12, -1, 12),
            (100, 1, 100),
            (0, 0, 0),
            (i64::MAX, 1, i64::MAX),
        ];

        for v in SAMPLES {
            let result = sigabs(v.0);
            assert_eq!(result.0, v.1);
            assert_eq!(result.1, v.2);
        }
    }

    #[test]
    #[should_panic]
    /** because of the usage of `i64::abs()`, `sigabs()` is expected to fail when evaluating `i64::MIN. */
    fn test_sigabs_overflow() {
        let v = sigabs(i64::MIN);
        if v.0 == i64::MIN { panic!("Sigabs result is signed!") }
    }
}