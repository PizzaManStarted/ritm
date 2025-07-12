use egui::{FontFamily, FontId};

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

    pub fn default_with_size(size: f32) -> FontId {
        FontId {
            family: FontFamily::Name("RobotoMono-regular".into()),
            size: size
        }
    }
}