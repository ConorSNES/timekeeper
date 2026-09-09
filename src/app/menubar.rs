use eframe::egui::{Button, Key, KeyboardShortcut, Modifiers, Response, Ui};

use crate::app::{App, action::AppAction, lib::check_bind};

impl App {
    const KEYBIND_NEW_RELATIVE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::N);
    const KEYBIND_NEW_ABSOLUTE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), Key::N);
    const KEYBIND_RESET_TIME: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::W);
    const KEYBIND_EXIT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::F4);
    // todo: there is no message in program for the borderless keybind.
    const KEYBIND_BORDERLESS: KeyboardShortcut = KeyboardShortcut::new(Modifiers::ALT, Key::B);

    /**
     * draws a button with an associated keybind; does not merge shortcut state with output response
     */
    fn draw_bind_button(ui: &mut Ui, label: &str, bind: &KeyboardShortcut) -> Response {
        ui.add(
            Button::new(label).shortcut_text(ui.ctx().format_shortcut(bind))
        )
    }

    // Creates the borderless config checkbox.
    fn draw_borderless_config(app: &mut App, ui: &mut Ui) {
        if ui.checkbox(app.state.get_borderless_mut(), "Borderless").clicked() {
            app.set_action(AppAction::RefreshBorderless);
        }
    }

    // Creates the theme config menu.
    fn draw_themeconfig(ui: &mut Ui) {
        ui.menu_button("Theme", |ui| {
            let mut theme = ui.theme();
            ui.radio_value(
                &mut theme,
                ui.system_theme().unwrap_or(eframe::egui::Theme::Dark),
                "Match System",
            );
            ui.radio_value(&mut theme, eframe::egui::Theme::Light, "Light");
            ui.radio_value(&mut theme, eframe::egui::Theme::Dark, "Dark");
            ui.set_theme(theme);
        });
    }

    pub fn draw_menubar(app: &mut App, ui: &mut Ui) {
        // Add navigation buttons.
        ui.style_mut().visuals.button_frame = false;
        {
            ui.menu_button("File", |ui| {
                Self::draw_borderless_config(app, ui);
                Self::draw_themeconfig(ui);
                if Self::draw_bind_button(ui, "Exit", &Self::KEYBIND_EXIT).clicked() {
                    app.set_action(AppAction::Exit);
                }
            });
            ui.menu_button("Timestamp", |ui| {
                if Self::draw_bind_button(ui, "Set (relative)", &Self::KEYBIND_NEW_RELATIVE).clicked() 
                {
                    app.set_action(AppAction::SetTimestampRel("".to_owned()));
                }
                if Self::draw_bind_button(ui, "Set (absolute)", &Self::KEYBIND_NEW_ABSOLUTE).clicked()
                {
                    app.set_action(AppAction::SetTimestampAbs("".to_owned()));
                }
                if Self::draw_bind_button(ui, "Reset", &Self::KEYBIND_RESET_TIME).clicked() {
                    app.set_action(AppAction::ResTimestamp);
                }
            });
            ui.menu_button("About", |ui| {
                ui.label(format!("{}, version {}", Self::PKG_NAME, Self::PKG_VERSION));
                ui.label("Authored by Conor SS 2026");
            });
        }
    }

    /**
     * Check for keybinds visually defined within draw_menubar.
     * 
     * Execution must occur outside of `draw_menubar` as per immediate-mode rendering; 
     * the action of the binds should be of greater scope than the buttons.
     */
    pub fn check_menubar_binds(app: &mut App, ui: &mut Ui) {
        // replace this with some kind of hash table at some point (and get mutable access only once!)

        if check_bind(ui, &Self::KEYBIND_EXIT) {
            app.set_action(AppAction::Exit);
        }
        else if check_bind(ui, &Self::KEYBIND_NEW_ABSOLUTE) {
            app.set_action(AppAction::SetTimestampAbs("".to_owned()));
        }
        else if check_bind(ui, &Self::KEYBIND_NEW_RELATIVE) {
            app.set_action(AppAction::SetTimestampRel("".to_owned()));
        }
        else if check_bind(ui, &Self::KEYBIND_RESET_TIME) {
            app.set_action(AppAction::ResTimestamp);
        }
        else if check_bind(ui, &Self::KEYBIND_BORDERLESS) {
            // slightly more going on here
            app.state.toggle_borderless();
            app.set_action(AppAction::RefreshBorderless);
        }
    }
}