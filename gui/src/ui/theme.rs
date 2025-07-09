use egui::Color32;

/// Theme used in the application
pub struct Theme {
    pub color1: Color32,
    pub color2: Color32,
    pub color3: Color32,
    pub color4: Color32,
    pub color5: Color32,
}

impl Theme {

    pub const DEFAULT: Theme = Theme {
        color1: Color32::from_rgb(245, 229, 211),
        color2: Color32::from_rgb(255, 255, 255),
        color3: Color32::from_rgb(151, 172, 187),
        color4: Color32::from_rgb(48, 65, 123),
        color5: Color32::from_rgb(239, 234, 215),
    };
}