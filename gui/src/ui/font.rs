use egui::{text::LayoutJob, vec2, Color32, FontFamily, FontId, Ui, Vec2};

/// Access to the different font used in the application
pub struct Font;

/// Font are accessed by function
///
/// To add one, first load it in the app.rs
impl Font {
    pub const BIG_SIZE: f32 = 20.0;
    pub const MEDIUM_SIZE: f32 = 16.0;
    pub const SMALL_SIZE: f32 = 12.0;

    /// default font used in the application
    pub fn default(ui: &Ui) -> FontId {
        let size = ui.ctx().screen_rect().size();
        FontId {
            family: FontFamily::Name("RobotoMono-regular".into()),
            size: if size.x < 1100.0 || size.y < 700.0 {
                Self::SMALL_SIZE
            } else if size.x > 1800.0 || size.y > 1200.0 {
                Self::BIG_SIZE
            } else {
                Self::MEDIUM_SIZE
            },
        }
    }

    /// bold version of the default font used in the application
    pub fn bold() -> FontId {
        FontId {
            family: FontFamily::Name("RobotoMono-Bold".into()),
            size: Font::MEDIUM_SIZE
        }
    }

    /// Compute the size of a text with a certain font
    pub fn text_size(ui: &Ui, fond_id: FontId, text: &str) -> Vec2 {
        let rect = ui.fonts(|f| {
            f.layout_job(LayoutJob::simple_singleline(
                text.to_string(),
                fond_id,
                Color32::PLACEHOLDER,
            ))
            .rect
        });

        vec2(rect.width(), rect.height())
    }

    pub fn get_heigth(ui: &Ui, fond_id: &FontId) -> f32 {
        ui.fonts(|f| f.row_height(fond_id))
    }

    pub fn get_width(ui: &Ui, fond_id: &FontId) -> f32 {
        ui.fonts(|f| f.glyph_width(fond_id, 'M'))
    }
}
