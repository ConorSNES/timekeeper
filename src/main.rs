use eframe::egui::{self, Vec2};

mod app;
mod font;
use app::App;

use crate::font::inject_fonts;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size(Vec2::new(App::WINDOW_MIN_X, App::WINDOW_MIN_Y))
            //.with_max_inner_size(Vec2::new(128.0, 128.0))
            //.with_resizable(false)
            .with_maximize_button(false)
            .with_app_id("timekeeper")
            //.with_icon(from_png_bytes(ICON).expect("Missing Icon File!!"))
            ,
        ..Default::default()
    };

    eframe::run_native(
        "Timekeeper",
        options,
        Box::new(|cc| {
            inject_fonts(&cc.egui_ctx);
            Ok(Box::new(App::new(cc)))
        })
    )
}
