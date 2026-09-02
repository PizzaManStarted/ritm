use egui::{
    Atom, Frame, Image, RichText, Spinner, Stroke, TextEdit, Ui, Vec2, include_image, vec2,
};
use serde::Serialize;

use crate::{
    App,
    api::ApiHandle,
    ui::{component::button::RitmButton, popup::RitmPopupEnum, theme::LIGHT_THEME},
};

#[derive(Default)]
pub struct Save {
    upload_request: NewMachineRequest,
}

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
                    Image::new(include_image!("../../../assets/icon/cloud_upload.svg"))
                        .fit_to_exact_size(Vec2::splat(30.0))
                        .tint(LIGHT_THEME.icon),
                    RichText::new("Save online"),
                )))
                .clicked()
            {
                app.ui.popup.open(RitmPopupEnum::Upload);
            }
        });
    }

    pub fn upload(ui: &mut Ui, app: &mut App) {
        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing.y = 10.0;
            TextEdit::singleline(&mut app.ui.save.upload_request.name)
                .hint_text("Name")
                .frame(
                    Frame::new()
                        .corner_radius(10.0)
                        .inner_margin(5)
                        .stroke(Stroke::new(1.0, LIGHT_THEME.border)),
                )
                .min_size(vec2(ui.available_width(), 0.0))
                .show(ui);
            TextEdit::multiline(&mut app.ui.save.upload_request.description)
                .hint_text("Description")
                .frame(
                    Frame::new()
                        .corner_radius(10.0)
                        .inner_margin(5)
                        .stroke(Stroke::new(1.0, LIGHT_THEME.border)),
                )
                .min_size(vec2(ui.available_width(), 0.0))
                .show(ui);

            let new_machine_handle: ApiHandle<NewMachineRequest, ()> =
                ApiHandle::new("new_machine");

            new_machine_handle.on_idle(ui, app, |ui, app| {
                if ui
                    .add(RitmButton::new((
                        Atom::grow(),
                        RichText::new("Save"),
                        Atom::grow(),
                    )))
                    .clicked()
                {
                    new_machine_handle.post(
                        app,
                        "api/turingmachine".to_string(),
                        app.ui.save.upload_request.clone(),
                    );
                }
            });

            new_machine_handle.on_wait(ui, app, |ui, _| {
                ui.add(Spinner::new().size(36.0));
            });

            new_machine_handle.on_result(ui, app, |_, _, _| {});
        });
    }
}
