use std::sync::Arc;

use eframe::egui::{Context, FontData, FontDefinitions};

pub const OPENSANS_MEDIUM: &str = "opensans";
pub const OPENSANS_LIGHT: &str = "opensans-l";
pub const OPENSANS_BOLD: &str = "opensans-b";

pub const FONTFAM_MED: &str = "medium";
pub const FONTFAM_HEV: &str = "heavy";

// inject custom fonts to program
pub fn inject_fonts(ctx: &Context) {
    let mut fontset = FontDefinitions::default();

    fontset.font_data.insert(
        OPENSANS_MEDIUM.into(),
        Arc::new(FontData::from_static(include_bytes!(
            "./OpenSans-Medium.ttf"
        ))),
    );

    fontset.font_data.insert(
        OPENSANS_LIGHT.into(),
        Arc::new(FontData::from_static(include_bytes!(
            "./OpenSans-Light.ttf"
        ))),
    );

    fontset.font_data.insert(
        OPENSANS_BOLD.into(),
        Arc::new(FontData::from_static(include_bytes!(
            "./OpenSans-Bold.ttf"
        ))),
    );

    // inject opensans light as main font
    fontset
        .families
        .entry(eframe::egui::FontFamily::Proportional)
        .or_default()
        .insert(0, OPENSANS_LIGHT.into());

    // add two sets;
    // medium weight
    fontset
        .families
        .insert(eframe::egui::FontFamily::Name(FONTFAM_MED.into()), vec![
            OPENSANS_MEDIUM.into()
        ]);

    // bold weight
    fontset
        .families
        .insert(eframe::egui::FontFamily::Name(FONTFAM_HEV.into()), vec![
            OPENSANS_BOLD.into()
        ]);


    ctx.set_fonts(fontset);
}