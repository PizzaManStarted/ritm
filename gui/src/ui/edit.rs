use egui::{
    include_image, vec2, Align, Button, Color32, ComboBox, Frame, Id, Image, ImageButton, LayerId, Layout, Margin, Rect, Sense, Shadow, Stroke, Ui, UiBuilder
};

use crate::{turing::State, ui::{popup::Popup, theme::Theme}, App};

/// Control of the graph
pub fn show(app: &mut App, ui: &mut Ui, rect: Rect) {
    // Floating control absolute position
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(Rect::from_center_size(
                rect.center_bottom() + vec2(0.0, -100.0),
                (rect.width(), 1.0).into(),
            ))
            .sense(Sense::empty())
            .layout(Layout::top_down(Align::Center).with_cross_align(Align::Center)),
        |ui| {
            // The parent ui paint on the background layer, so we need to change it to a higher layer
            let layer = LayerId::new(egui::Order::Middle, Id::new("edit"));
            ui.scope_builder(UiBuilder::new().layer_id(layer), |ui| {
                // Used for background
                Frame::new()
                    .fill(app.theme.white)
                    .inner_margin(Margin::symmetric(20, 10))
                    .corner_radius(15)
                    .stroke(Stroke::new(1.0, app.theme.gray))
                    .shadow(Shadow {
                        offset: [0, 4],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_black_alpha(25),
                    })
                    .show(ui, |ui| {

                        let state_selected = app.selected_state.is_some();
                        let transition_selected = app.selected_transition.is_some();
                        // Need to compute the width of the menu to center it

                        // Recenter/Unpin/Pin
                        let mut count = 3;
                        if state_selected && transition_selected {
                            // Adding a state condition
                            count += 1;
                        }
                        if state_selected {
                            // Create transition from the selected state
                            count += 1;
                        }
                        if state_selected || transition_selected {
                            // Delete/Edit state/transition
                            count += 2;
                        }
                        let width = 35.0 * count as f32 + 20.0 * (count - 1) as f32;

                        ui.allocate_ui_with_layout(
                            vec2(width, 35.0),
                            Layout::left_to_right(Align::Min).with_cross_align(Align::Center),
                            |ui| {
                                ui.spacing_mut().item_spacing = vec2(20.0, 0.0);

                                // State
                                // Only possible to add a state if nothing is selected
                                // IDEA : maybe permit it for state selected, and create a transition directly
                                if app.selected_state.is_none() && app.selected_transition.is_none()
                                {
                                    if ui
                                        .add(
                                            ImageButton::new(
                                                Image::new(include_image!(
                                                    "../../assets/icon/stateplus.svg"
                                                ))
                                                .fit_to_exact_size(vec2(35.0, 35.0))
                                                .tint(if app.event.is_adding_state {
                                                    app.theme.selected
                                                } else {
                                                    Theme::constrast_color(app.theme.white)
                                                }),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        app.event.is_adding_state ^= true;
                                    }
                                }

                                // Transition
                                // Only possible to create transition if a state is selected
                                if app.selected_state.is_some() {
                                    if ui
                                        .add(
                                            ImageButton::new(
                                                Image::new(include_image!(
                                                    "../../assets/icon/transition.svg"
                                                ))
                                                .fit_to_exact_size(vec2(35.0, 35.0))
                                                .tint(if app.event.is_adding_transition {
                                                    app.theme.selected
                                                } else {
                                                    Theme::constrast_color(app.theme.white)
                                                }),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {
                                        app.event.is_adding_transition ^= true;
                                    }
                                }

                                // Delete
                                // If a state or transition is selected, then display the delete button
                                if (app.selected_state.is_some()
                                    || app.selected_transition.is_some())
                                    && ui
                                        .add(
                                            ImageButton::new(
                                                Image::new(include_image!(
                                                    "../../assets/icon/delete.svg"
                                                ))
                                                .fit_to_exact_size(vec2(35.0, 35.0))
                                                .tint(Theme::constrast_color(app.theme.white)),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                {
                                    if state_selected {
                                        app.remove_state();
                                    }
                                    
                                    if transition_selected {
                                        app.remove_transitions();
                                    }
                                }

                                if (app.selected_state.is_some()
                                    || app.selected_transition.is_some())
                                    && ui
                                        .add(
                                            ImageButton::new(
                                                Image::new(include_image!(
                                                    "../../assets/icon/edit.svg"
                                                ))
                                                .fit_to_exact_size(vec2(35.0, 35.0))
                                                .tint(Theme::constrast_color(app.theme.white)),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                {
                                    if state_selected {
                                        app.popup = Popup::StateEdit;
                                    }
                                    if transition_selected {
                                        app.popup = Popup::TransitionEdit
                                    }
                                }


                                if ui
                                        .add(
                                            ImageButton::new(
                                                Image::new(include_image!(
                                                    "../../assets/icon/recenter.svg"
                                                ))
                                                .fit_to_exact_size(vec2(35.0, 35.0))
                                                .tint(Theme::constrast_color(app.theme.white)),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                {
                                    app.event.need_recenter = true;
                                }

                                if ui
                                        .add(
                                            ImageButton::new(
                                                Image::new(include_image!(
                                                    "../../assets/icon/unpin.svg"
                                                ))
                                                .fit_to_exact_size(vec2(35.0, 35.0))
                                                .tint(Theme::constrast_color(app.theme.white)),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                {
                                    app.unpin();
                                }

                                if ui
                                        .add(
                                            ImageButton::new(
                                                Image::new(include_image!(
                                                    "../../assets/icon/pin.svg"
                                                ))
                                                .fit_to_exact_size(vec2(35.0, 35.0))
                                                .tint(Theme::constrast_color(app.theme.white)),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                {
                                    if state_selected {
                                        State::get_mut(app, app.selected_state.unwrap()).is_pinned = true;
                                    } else {
                                        app.pin();
                                    }
                                }
                            },
                        );
                    });
            });
        },
    );
}
