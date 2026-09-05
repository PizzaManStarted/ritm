use egui::{
    Atom, Frame, Image, RichText, Spinner, Stroke, TextEdit, Ui, Vec2, include_image, vec2,
};
use serde::Serialize;

use crate::{
    App,
    ui::{component::button::RitmButton, popup::RitmPopupEnum, theme::LIGHT_THEME},
};

#[derive(Default)]
pub struct Save {}

#[derive(Serialize, Default, Clone)]
struct NewMachineRequest {
    name: String,
    description: String,
}

impl Save {
    pub fn save(ui: &mut Ui, app: &mut App) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 10.0;
            if ui
                .add(RitmButton::new((
                    Image::new(include_image!("../../../assets/icon/save_on_disk.svg"))
                        .fit_to_exact_size(Vec2::splat(30.0))
                        .tint(LIGHT_THEME.icon),
                    RichText::new("Save on disk"),
                )))
                .clicked()
            {
                let _ = app.ui.code.save_current_tab();
                app.ui.popup.close();
            }

            if ui
                .add(RitmButton::new((
                    Image::new(include_image!("../../../assets/icon/delete.svg"))
                        .fit_to_exact_size(Vec2::splat(30.0))
                        .tint(LIGHT_THEME.icon),
                    RichText::new("Save online"),
                )))
                .clicked()
            {
                // Put in clipboard ?
            }
        });
    }
}
