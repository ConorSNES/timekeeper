use chrono::{DateTime, Local};
use eframe::egui::{FontFamily, Frame, Modal, ModalResponse, RichText, TextEdit, Ui};

use crate::{app::{App, action::AppAction, lib::{check_bind, delta_now, draw_bind_button, format_delta_time}, managed_focus::{FocusManager, ManagedFocus}}, font::{FONTFAM_HEV, FONTFAM_MED}};

impl App {
    /**
     * Draw a simple clock in large print.
     */
    pub fn draw_big_clock(ui: &mut Ui) {
        // collect system time NOW
        let timenow = chrono::offset::Local::now();

        ui.label(
            RichText::new(timenow.format("%H:%M:%S").to_string())
                .size(48.0)
                .family(FontFamily::Name(FONTFAM_HEV.into()))
                .strong(),
        );
    }

    pub fn draw_delta_time(ui: &mut Ui, from: DateTime<Local>) {
        let timestamp_text = format_delta_time(delta_now(from));
        let timestamp_tooltip = from.format("%H:%M:%S").to_string();
        ui.label(
            RichText::new(timestamp_text)
                .size(24.0)
                .family(FontFamily::Name(FONTFAM_MED.into()))
                .weak(),
        ).on_hover_text(timestamp_tooltip);
    }

    /**
     * Generic recipe for drawing a modal in program style.
     */
    fn draw_modal<T>(ui: &mut Ui, inner: impl FnOnce(&mut Ui) -> T) -> ModalResponse<T> {
        let modalframe = Frame::popup(ui.style()).fill(Self::get_theme_cols(ui).1);

        Modal::new("action".into())
            .frame(modalframe)
            .show(ui, inner)
    }

    /**
     * Modal asking and managing requests to set a new timestamp from an absolute point in time.
     */
    pub fn draw_absolute_timestamp_modal(ui: &mut Ui, newts: &mut String, focusmgr: &mut FocusManager) -> Option<AppAction> {
        let mut transition = None;
        Self::draw_modal(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                    ui.label("Create an absolute timestamp;\nFormat is in [[hh:]mm:]ss\nAll timestamps assumed to be in future");
                    focusmgr.default_focus(
                        &ui.add(
                        TextEdit::singleline(newts).hint_text("00:00:00")
                        ).id
                    );
                    
                    ui.horizontal(|ui| {
                        if draw_bind_button(ui, RichText::from("Apply").strong(), &Self::KEYBIND_SUBMIT_GENERIC).clicked() 
                            || check_bind(ui, &Self::KEYBIND_SUBMIT_GENERIC) 
                        {
                            // parse and apply
                            let timestamp = Self::parse_user_timestamp_abs(newts.to_string());

                            // return to standard position
                            transition = Some(AppAction::SetTimestamp(timestamp))
                        }
                        if draw_bind_button(ui, "Exit", &Self::KEYBIND_SUBMIT_GENERIC).clicked()
                            || check_bind(ui, &Self::KEYBIND_DISMISS_GENERIC) 
                        {
                            transition = Some(AppAction::None)
                        }
                    })
                })
            });
        transition
    }

    /**
     * Modal asking and managing requests to set a new timestamp relative to the current time.
     */
    pub fn draw_relative_timestamp_modal(ui: &mut Ui, newts: &mut String, focusmgr: &mut FocusManager) -> Option<AppAction> {
        let mut transition = None;
        Self::draw_modal(ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label("Create a relative timestamp;\nFormat is in [+|-][[hh:]mm:]ss");
                focusmgr.default_focus(
                    &ui.add(
                    TextEdit::singleline(newts).hint_text("±00:00:00")
                    ).id
                );
                ui.horizontal(|ui| {
                    if draw_bind_button(ui, RichText::from("Apply").strong(), &Self::KEYBIND_SUBMIT_GENERIC).clicked()
                        || check_bind(ui, &Self::KEYBIND_SUBMIT_GENERIC) 
                        {
                        // parse and apply
                        let new_timestamp = Self::parse_user_timestamp_rel(newts.to_string());

                        // return to standard position
                        transition = Some(AppAction::SetTimestamp(new_timestamp))
                    }
                    if draw_bind_button(ui, "Exit", &Self::KEYBIND_DISMISS_GENERIC).clicked()
                        || check_bind(ui, &Self::KEYBIND_DISMISS_GENERIC) 
                    {
                        transition = Some(AppAction::None)
                    }
                })
            })
        });
        transition
    }
}