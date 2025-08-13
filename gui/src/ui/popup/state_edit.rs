use egui::{
    Align, Context, Frame, Id, Image, Layout, Margin, Modal, Stroke, TextEdit, Ui,
    Vec2, include_image, vec2,
};

use crate::{App, turing::State, ui::font::Font};

pub fn show(app: &mut App, ctx: &Context) {
    Modal::new(Id::new("state_edit"))
        .frame(Frame {
            fill: app.theme.white,
            stroke: Stroke::new(2.0, app.theme.gray),
            inner_margin: Margin::same(10),
            corner_radius: 10.into(),
            ..Default::default()
        })
        .show(ctx, |ui: &mut Ui| {
            
            ui.vertical_centered(|ui| {
                ui.style_mut().spacing.item_spacing = vec2(0.0, 10.0);

                ui.allocate_ui_with_layout(
                    vec2(ui.available_width() / 2.0, 0.0),
                    Layout::right_to_left(Align::Center),
                    |ui| {
                        ui.add(
                            Image::new(include_image!("../../../assets/icon/edit.svg"))
                                .fit_to_exact_size(Vec2::splat(20.0))
                                .tint(app.theme.gray),
                        );

                        let state = State::get_mut(app, app.selected_state.unwrap());

                        let edit = TextEdit::singleline(&mut state.name)
                            .font(Font::default_big())
                            .frame(false);
                        ui.add(edit)
                    },
                )
            });
        });
}
