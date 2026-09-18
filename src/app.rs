use std::time::Duration;

use chrono::{DateTime, Days, Local, TimeDelta, Timelike};
use eframe::egui::{
    self, Color32, Frame, Key, KeyboardShortcut, Margin, Modifiers, Theme, Ui,
};
use serde::{Deserialize, Serialize};

use crate::app::action::AppAction;
use crate::app::managed_focus::{FocusManager, ManagedFocus};
use crate::app::state::AppState;

mod action;
mod menubar;
mod state;
mod lib;
mod managed_focus;
mod widgets;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct App {
    state: AppState,
    #[serde(skip)]
    action: AppAction,
    #[serde(skip)]
    managed_focus: FocusManager,
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
     * Shorthand for self.set_action(AppAction::None)
     */
    pub fn reset_action(&mut self) {
        self.action = AppAction::None;
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


    /**
     * parser for relative timestamps, automatically calculating based on a difference from current time.
     * 
     * timestamps are in the format [+|-][[hh:]mm:]ss
     */
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
                        if let Some(v) = Self::draw_big_clock(ui, self.state.timestamp_format_string()) {
                            self.set_action(v);
                        }

                        // add delta time if available;
                        if let Some(timestamp) = self.state.get_timestamp() {
                            Self::draw_delta_time(ui, self.state.timestamp_format_string(), timestamp);
                        }
                    });
                });
            });

        // draw modals for actions
        if let Some(v) = match &mut self.action {
            AppAction::TimestampAbsRequest(newts) => {
                Self::draw_absolute_timestamp_modal(ui, newts, &mut self.managed_focus)
            }
            AppAction::TimestampRelRequest(newts) => {
                Self::draw_relative_timestamp_modal(ui, newts, &mut self.managed_focus)
            }
            _ => {
                // if no modal is present, reset the focus management
                self.managed_focus.reset_focus();
                None
            },
        }
        
        {
            self.action = v
        };

        // apply "instant" actions;
        match self.action {
            AppAction::Exit => {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            AppAction::SetTimestamp(v) => {
                self.state.set_timestamp(v);
                self.reset_action();
            }
            AppAction::ResTimestamp => {
                // reset the timestamp
                self.state.reset_timestamp();
                self.reset_action();
            }
            AppAction::Toggle12Hour => {
                self.state.toggle_12hr();
                self.reset_action();
            }
            AppAction::RefreshBorderless => {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Decorations(!self.state.get_borderless()));
                self.reset_action();
            }
            _ => {}
        }

        // apply focus...
        self.managed_focus.apply_focus(ui);

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
