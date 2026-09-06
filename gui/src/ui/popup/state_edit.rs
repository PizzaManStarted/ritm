use egui::{
    Align, AtomExt, Button, Color32, Image, Layout, RichText, Stroke, TextEdit, Ui, Vec2,
    include_image, vec2,
};

use crate::{
    App,
    error::RitmError,
    turing::StateEdit,
    ui::{popup::RitmPopupEnum, theme::{LIGHT_THEME, Theme}},
    utils::font::Font,
};

pub fn show(ui: &mut Ui, app: &mut App) -> Result<(), RitmError> {
    ui.with_layout(Layout::top_down(Align::Min), |ui| {
        ui.style_mut().spacing.item_spacing = vec2(10.0, 10.0);

        ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
            ui.add(
                Image::new(include_image!("../../../assets/icon/edit.svg"))
                    .fit_to_exact_size(Vec2::splat(
                        Font::get_heigth(ui, &Font::default_big()) + 4.0,
                    ))
                    .tint(LIGHT_THEME.surface),
            );

            if app.turing.state_edit.is_none()
                && let Some(RitmPopupEnum::StateEdit(selected)) = app.ui.popup.current()
            {
                app.turing.state_edit = if let Ok(state) = app.turing.get_state(*selected) {
                    Some(StateEdit::from(state))
                } else {
                    Some(StateEdit::empty(app.turing.next_id()))
                }
            }

            let Some(state) = &mut app.turing.state_edit else {
                app.ui.popup.close();
                return;
            };

            let edit = TextEdit::singleline(&mut state.get_edit().name)
                .font(Font::default_big())
                .background_color(Color32::from_black_alpha(20))
                .char_limit(10);

            ui.add(edit);
        });

        ui.columns(2, |ui| {
            let text = RichText::new("Cancel")
                .color(Theme::constrast_color(LIGHT_THEME.error))
                .font(Font::default_medium())
                .atom_grow(true);

            if ui[1]
                .add(
                    Button::new(text)
                        .stroke(Stroke::new(2.0, LIGHT_THEME.border))
                        .fill(LIGHT_THEME.error)
                        .corner_radius(10.0),
                )
                .clicked()
            {
                app.ui.popup.close();
            }

            let Some(state) = &app.turing.state_edit else {
                app.ui.popup.close();
                return Ok(());
            };

            let state_name = state.to().name.clone();

            if (ui[0]
                .add(
                    Button::new(
                        RichText::new("Save")
                            .color(Theme::constrast_color(LIGHT_THEME.success))
                            .font(Font::default_medium())
                            .atom_grow(true),
                    )
                    .stroke(Stroke::new(2.0, LIGHT_THEME.border))
                    .fill(if state_name.is_empty() {
                        LIGHT_THEME.border
                    } else {
                        LIGHT_THEME.success
                    })
                    .corner_radius(10.0),
                )
                .clicked())
                && !state_name.is_empty()
            {
                let state_id = app.turing.apply_state_change();

                match state_id {
                    Ok(state_id) => {
                        app.turing.state_edit = None;
                        app.ui.graph.select_state(state_id);
                        app.ui.popup.close();
                    }
                    Err(_error) => {
                        // app.ui.error.push_back(error);
                    }
                }
            };
            Ok::<(), RitmError>(())
        })
    })
    .inner
}