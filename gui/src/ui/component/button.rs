use egui::{Atoms, Button, Color32, IntoAtoms, Stroke, TextWrapMode, Widget, vec2};

use crate::ui::theme::LIGHT_THEME;

pub struct RitmButton<'a> {
    atoms: Atoms<'a>,
}

impl<'a> RitmButton<'a> {
    pub fn new(atoms: impl IntoAtoms<'a>) -> Self {
        Self {
            atoms: atoms.into_atoms(),
        }
    }
}

impl<'a> Widget for RitmButton<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.spacing_mut().button_padding = vec2(20.0, 5.0);
        ui.add(
            Button::new(self.atoms)
                .stroke(Stroke::new(1.0, LIGHT_THEME.border))
                .fill(Color32::TRANSPARENT)
                .wrap_mode(TextWrapMode::Extend)
                .gap(10.0)
                .min_size(vec2(ui.available_width(), 0.0))
                .corner_radius(10.0)
                .image_tint_follows_text_color(true),
        )
    }
}