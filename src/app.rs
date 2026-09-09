use std::time::Duration;

use chrono::{DateTime, Days, Local, TimeDelta, Timelike};
use eframe::egui::{
    self, Color32, FontFamily, Frame, Key, KeyboardShortcut, Margin, Modal, ModalResponse, Modifiers, RichText, TextEdit, Theme, Ui,
};
use serde::{Deserialize, Serialize};

use crate::app::action::AppAction;
use crate::app::lib::check_bind;
use crate::app::state::AppState;
use crate::font::{FONTFAM_HEV, FONTFAM_MED};

mod action;
mod menubar;
mod state;
mod lib;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct App {
    state: AppState,
    #[serde(skip)]
    action: AppAction,
}

impl App {
    const PERSISTENCE_KEY: &str = "timekeeper_state";

    pub const WINDOW_MIN_X : f32 = 256.0;
    pub const WINDOW_MIN_Y : f32 = 148.0;

    const PKG_NAME: &str = env!("CARGO_PKG_NAME");
    const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

    const COL_LIGHT: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);
    const COL_LIGHTGREY: Color32 = Color32::from_rgb(0xF8, 0xF8, 0xF8);
    const COL_DARKGREY: Color32 = Color32::from_rgb(0x30, 0x30, 0x30);
    const COL_DARK: Color32 = Color32::from_rgb(0x1B, 0x1B, 0x1B);

    const KEYBIND_SUBMIT_GENERIC: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::Enter);
    const KEYBIND_DISMISS_GENERIC: KeyboardShortcut = KeyboardShortcut::new(Modifiers::NONE, Key::Escape);

    /**
     * compose new App based on persistence data 
     */ 
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let o = match cc.storage {
            None => Self::default(),
            Some(storage) => match eframe::get_value(storage, Self::PERSISTENCE_KEY) {
                None => Self::default(),
                Some(v) => v,
            },
        };
        // apply borderless-ness from app state we've just loaded...
        cc.egui_ctx
            .send_viewport_cmd(egui::ViewportCommand::Decorations(
                !o.state.get_borderless(),
            ));
        o
    }

    /**
     * replace the current action with a new one
     */
    pub fn set_action(&mut self, newact: AppAction) {
        self.action = newact;
    }

    /**
     * collect bg and bg-highlight colours for the current theme
     * 
     * note: theme state is independent of app state while also being persistent
     */
    fn get_theme_cols(ui: &Ui) -> (Color32, Color32) {
        match ui.theme() {
            Theme::Dark => (Self::COL_DARK, Self::COL_DARKGREY),
            Theme::Light => (Self::COL_LIGHT, Self::COL_LIGHTGREY),
        }
    }

    /**
     * modal with custom style for app
     */
    fn draw_modal<T>(ui: &mut Ui, inner: impl FnOnce(&mut Ui) -> T) -> ModalResponse<T> {
        let modalframe = Frame::popup(ui.style()).fill(Self::get_theme_cols(ui).1);

        Modal::new("action".into())
            .frame(modalframe)
            .show(ui, inner)
    }

    /**
     * combination of sign/absolute for a uint,
     * for separating the sign and magnitude of a number
     */
    fn sigabs(v: i64) -> (i64, i64) {
        (v.signum(), v.abs())
    }

    /**
     * custom parser for absolute user timestamps
     * 
     * timestamps are in the format [[hh:]mm:]ss
     */
    fn parse_user_timestamp_abs(value: String) -> DateTime<Local> {
        let timenow = chrono::offset::Local::now();
        let mut indat = value.split(":").collect::<Vec<&str>>().into_iter().rev();
        let secs = indat
            .next()
            .map_or(0, |v| v.parse::<u32>().unwrap_or(0))
            .clamp(0, 59);
        let mins = indat
            .next()
            .map_or(0, |v| v.parse::<u32>().unwrap_or(0))
            .clamp(0, 59);
        let hours = indat
            .next()
            .map_or(0, |v| v.parse::<u32>().unwrap_or(0))
            .clamp(0, 23);
        let mut timestamp = timenow.clone();

        timestamp = timestamp
            .with_hour(hours)
            .unwrap()
            .with_minute(mins)
            .unwrap()
            .with_second(secs)
            .unwrap();

        // increment timestamp by one day if this time is earlier than the current time
        if timestamp.time() < timenow.time() {
            timestamp = timestamp.checked_add_days(Days::new(1)).unwrap()
        }

        timestamp
    }


    fn parse_user_timestamp_rel(value: String) -> DateTime<Local> {
        let timenow = chrono::offset::Local::now();

        // collect sign first, if available
        let sign = {
            let firstchar = value.chars().next().unwrap_or('0');
            match firstchar {
                '-' => -1,
                _ => 1
            }
        };

        let mut indat = value.split(":").collect::<Vec<&str>>().into_iter().rev();
        let secs = indat
            .next()
            .map_or(0, |v| v.parse::<i64>().unwrap_or(0))
            .abs()
            .clamp(0, 59);
        let mins = indat
            .next()
            .map_or(0, |v| v.parse::<i64>().unwrap_or(0))
            .abs()
            .clamp(0, 59);
        let hours = indat
            .next()
            .map_or(0, |v| v.parse::<i64>().unwrap_or(0))
            .abs()
            .clamp(0, 23);
        let delta = TimeDelta::seconds(
            secs +
            (mins * 60) +
            (hours * 60 * 60)
        ) * sign;

        // final result is the current time with the delta added
        timenow.clone() + delta        
    }

    /**
     * function for formatting user timestamp into readable format (relative to current time)
     */
    fn format_user_timestamp(ts: DateTime<Local>) -> String {
        let timenow = chrono::offset::Local::now();
        let diff = ts - timenow;
        let hour = (diff.num_hours()).abs();
        let mins = (diff.num_minutes() % 60).abs();
        let (sign, secs) = Self::sigabs((diff.num_seconds() % 60) % 60);
        format!(
            "t{}{:02}:{:02}:{:02}",
            if sign < 0 { "+" } else { "-" },
            hour,
            mins,
            secs
        )
    }

    /**
     * returns formatted timestamp if one is present in state;
     */
    fn _try_get_timestamp_formatted(&self) -> Option<String> {
        self.state.get_timestamp().map(|v| Self::format_user_timestamp(v))
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        #[allow(unused)]
        let (col_bg, col_bg_highlight) = Self::get_theme_cols(ui);

        ui.set_min_height(App::WINDOW_MIN_Y);
        ui.set_min_width(App::WINDOW_MIN_X);

        // firstly draw navbar
        egui::Panel::top("navigation").show(ui, |ui| {
            ui.horizontal_centered(|ui| {
                // Wrap these elements in a panel for improved layout
                Frame::new().inner_margin(2.0).show(ui, |ui| {
                    Self::draw_menubar(self, ui);
                })
            })
        });

        // parse hotkeys on navbar
        Self::check_menubar_binds(self, ui);

        egui::CentralPanel::default()
            .frame(
                Frame::new()
                    .fill(col_bg)
                    .inner_margin(Margin::symmetric(16, 4)),
            )
            .show(ui, |ui| {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        // collect system time NOW
                        let timenow = chrono::offset::Local::now();

                        ui.label(
                            RichText::new(timenow.format("%H:%M:%S").to_string())
                                .size(48.0)
                                .family(FontFamily::Name(FONTFAM_HEV.into()))
                                .strong(),
                        );

                        // add delta time if available;
                        if let Some(timestamp) = self.state.get_timestamp() {
                            let timestamp_text = Self::format_user_timestamp(timestamp);
                            let timestamp_tooltip = timestamp.format("%H:%M:%S").to_string();
                            ui.label(
                                RichText::new(timestamp_text)
                                    .size(24.0)
                                    .family(FontFamily::Name(FONTFAM_MED.into()))
                                    .weak(),
                            ).on_hover_text(timestamp_tooltip);
                        }
                    });
                });
            });

        // draw modals for actions
        let mut transition = None;
        match &mut self.action {
            AppAction::SetTimestampAbs(newts) => {
                Self::draw_modal(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                    ui.label("Create an absolute timestamp;\nFormat is in [[hh:]mm:]ss\nAll timestamps assumed to be in future");
                    ui.add(
                        TextEdit::singleline(newts).hint_text("00:00:00")
                    );
                    ui.horizontal(|ui| {
                        if ui.button(RichText::from("Apply").strong()).clicked() 
                            || check_bind(ui, &Self::KEYBIND_SUBMIT_GENERIC) 
                        {
                            // parse and apply
                            let timestamp = Self::parse_user_timestamp_abs(newts.to_string());
                            self.state.set_timestamp(timestamp);

                            // return to standard position
                            transition = Some(AppAction::None)
                        }
                        if ui.button("Exit").clicked() 
                            || check_bind(ui, &Self::KEYBIND_DISMISS_GENERIC) 
                        {
                            transition = Some(AppAction::None)
                        }
                    })
                })
                });
            }
            AppAction::SetTimestampRel(newts) => {
                Self::draw_modal(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.label("Create a relative timestamp;\nFormat is in [+|-][[hh:]mm:]ss");
                        ui.add(
                            TextEdit::singleline(newts).hint_text("±00:00:00")
                        );
                        ui.horizontal(|ui| {
                            if ui.button(RichText::from("Apply").strong()).clicked() 
                                || check_bind(ui, &Self::KEYBIND_SUBMIT_GENERIC) 
                                {
                                // parse and apply
                                let new_timestamp = Self::parse_user_timestamp_rel(newts.to_string());
                                self.state.set_timestamp(new_timestamp);

                                // return to standard position
                                transition = Some(AppAction::None)
                            }
                            if ui.button("Exit").clicked() 
                                || check_bind(ui, &Self::KEYBIND_DISMISS_GENERIC) 
                            {
                                transition = Some(AppAction::None)
                            }
                        })
                    })
                });
            }
            _ => (),
        }
        if let Some(v) = transition {
            self.action = v
        };

        // apply lingering "instant" actions;
        match self.action {
            AppAction::Exit => {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            AppAction::ResTimestamp => {
                // reset the timestamp
                self.state.reset_timestamp();
                self.set_action(AppAction::None);
            }
            AppAction::RefreshBorderless => {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Decorations(!self.state.get_borderless()));
            }
            _ => {}
        }

        // force repaint (egui will get lazy!)
        ui.request_repaint_after(Duration::from_secs(1));
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, Self::PERSISTENCE_KEY, &self);
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        Duration::new(5, 0)
    }
}
