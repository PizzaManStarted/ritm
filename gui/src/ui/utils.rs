use eframe::egui::{Pos2, Vec2};
use egui::{text::LayoutJob, vec2, Color32, FontFamily, FontId, Ui};

use crate::ui::constant::Constant;

/// Compute the distance between 2 points
pub fn distance(p1: Pos2, p2: Pos2) -> f32 {
    f32::sqrt((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2))
}

/// Compute the repulsion force of the node
pub fn rep_force(p1: Pos2, p2: Pos2) -> f32 {
    let force = Constant::CREP / distance(p1, p2).powi(2);
    f32::max(-Constant::MAX_FORCE, f32::min(Constant::MAX_FORCE, force))
}

/// Compute the attraction force of the node
pub fn attract_force(p1: Pos2, p2: Pos2) -> f32 {
    let force = Constant::CSPRING * (distance(p1, p2) / (Constant::L)).log(10.0);
    f32::max(-Constant::MAX_FORCE, f32::min(Constant::MAX_FORCE, force))
}

/// Compute the direction between 2 points
pub fn direction(p1: Pos2, p2: Pos2) -> Vec2 {
    Vec2::new(p2.x - p1.x, p2.y - p1.y).normalized()
}

/// Compute the best contrast color between white and black for any RGB color
pub fn constrast_color(color: Color32) -> Color32 {
    let luminance =
        (0.299 * color.r() as f32 + 0.587 * color.g() as f32 + 0.114 * color.b() as f32) / 255.0;

    if luminance > 0.5 {
        Color32::BLACK
    } else {
        Color32::WHITE
    }
}

/// Compute the size of a text with a certain font
pub fn text_size(ui: &Ui, fond_id: FontId, text: &str) -> Vec2 {
    let rect = ui.fonts(|f| {
        f.layout_job(LayoutJob::simple_singleline(
            text.to_string(),
            fond_id,
            Color32::BLACK,
        ))
        .rect
    });

    vec2(rect.width(), rect.height())
}

/// Access to the different font used in the application
pub struct Font;

/// Font are accessed by function
/// 
/// To add one, first load it in the app.rs
impl Font {

    const BIG_SIZE: f32 = 20.0;
    const MEDIUM_SIZE: f32 = 16.0;
    const SMALL_SIZE: f32 = 12.0;

    /// The default font
    pub fn default() -> FontId {
        FontId {
            family: FontFamily::Name("RobotoMono-regular".into()),
            size: Font::MEDIUM_SIZE
        }
    }
}
