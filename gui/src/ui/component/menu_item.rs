use egui::{
    Button, Color32, Image, ImageSource, IntoAtoms, RichText, Stroke, Widget
};

use crate::{ui::theme::LIGHT_THEME, utils::font::Font};

pub struct MenuItem<'a> {
    icon: ImageSource<'a>,
    text: Option<String>,
}

impl<'a> MenuItem<'a> {
    pub fn new(image: ImageSource<'a>) -> Self {
        Self {
            icon: image,
            text: None,
        }
    }

    pub fn with_text(mut self, text: impl ToString) -> Self {
        self.text = Some(text.to_string());
        self
    }
}

impl<'a> Widget for MenuItem<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::TRANSPARENT;
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::from_black_alpha(25);
        ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
        ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;

        let atoms = if let Some(text) = self.text {
            (
                Image::new(self.icon).tint(LIGHT_THEME.primary),
                RichText::new(text)
                    .color(LIGHT_THEME.primary)
                    .text_style(Font::DEFAULT_FONT)
                    .size(Font::MEDIUM),
            )
                .into_atoms()
        } else {
            Image::new(self.icon).tint(LIGHT_THEME.primary).into_atoms()
        };

        Button::new(atoms).gap(5.0).ui(ui)
    }
}