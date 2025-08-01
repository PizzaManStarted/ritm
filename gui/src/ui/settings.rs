use egui::{include_image, vec2, Image, ImageButton, Layout, Ui, Vec2};

use crate::{ui::{constant::Constant, theme::Theme}, App};

pub fn show(app: &mut App, ui: &mut Ui) {
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 0.0),
        Layout::left_to_right(egui::Align::Center),
        |ui| {
            if ui
                .add(
                    ImageButton::new(
                        Image::new(include_image!("../../assets/icon/setting.svg"))
                            .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
                            .tint(Theme::constrast_color(app.theme.background)),
                    )
                    .frame(false),
                )
                .clicked()
            {
                app.event.need_recenter = true;
            }
        },
    );
}
