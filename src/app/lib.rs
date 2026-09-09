use eframe::egui::{KeyboardShortcut, Ui};

/**
 * check for a keybind being pressed
 */
pub fn check_bind(ui: &mut Ui, shortcut: &KeyboardShortcut) -> bool {
    ui.input_mut(|inp| inp.consume_shortcut(shortcut))
}