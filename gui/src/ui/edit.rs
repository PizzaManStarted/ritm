use egui::{
    Align, Button, Color32, Frame, Id, Image, ImageSource, LayerId, Layout, Margin, Pos2, Rect, Response, Stroke, Ui, UiBuilder, Vec2, include_image, vec2,
};

use crate::{
    App,
    error::RitmError,
    ui::{
        popup::RitmPopupEnum, theme::LIGHT_THEME,
    },
};

#[derive(Default)]
pub struct Edit {
    pub is_adding_state: bool,
    pub is_adding_transition: bool,
}

/// Control of the graph
pub fn show(app: &mut App, ui: &mut Ui) -> Result<(), RitmError> {
    let icon_size = app.settings.edit_button_size + 10.0;

    let ui_rect = ui.available_rect_before_wrap();
    // Floating control absolute position
    ui.scope_builder(
        UiBuilder::new()
            .max_rect(Rect::from_min_max(
                Pos2::new(ui_rect.right() - icon_size, ui_rect.top()),
                Pos2::new(ui_rect.right(), ui_rect.bottom()),
            ))
            .layer_id(LayerId::new(egui::Order::Debug, Id::new("test")))
            .layout(Layout::bottom_up(Align::Center).with_cross_align(Align::Center)),
        |ui| {

        

            // TODO: replace with flags from bitflags crate
            let state_selected =
                app.ui.graph.selected_state().is_some() && app.ui.graph.selected_transitions().is_none();
            let _transition_selected =
                app.ui.graph.selected_transitions().is_some() && app.ui.graph.selected_state().is_none();
            let _both_selected =
                app.ui.graph.selected_state().is_some() && app.ui.graph.selected_transitions().is_some();
            let either_selected =
                app.ui.graph.selected_state().is_some() || app.ui.graph.selected_transitions().is_some();
            let none_selected =
                app.ui.graph.selected_state().is_none() && app.ui.graph.selected_transitions().is_none();

            // Vertical alignment, bottom to up
            let _edit = ui
                .allocate_ui_with_layout(
                    vec2(icon_size, ui.available_height()),
                    Layout::bottom_up(Align::Center).with_cross_align(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing = vec2(0.0, 5.0);

                        // State
                        // Only possible to add a state if nothing is selected
                        // IDEA : maybe permit it for state selected, and create a transition directly
                        if none_selected {
                            let state = button(
                                ui,
                                app,
                                include_image!("../../assets/icon/stateplus.svg"),
                                app.ui.edit.is_adding_state,
                            );
                            if state.clicked() {
                                app.ui.edit.is_adding_state ^= true;
                            }
                        }

                        // Transition
                        // Only possible to create transition if a state is selected
                        if state_selected {
                            let button = button(
                                ui,
                                app,
                                include_image!("../../assets/icon/transition.svg"),
                                app.ui.edit.is_adding_transition,
                            );
                            if button.clicked() {
                                app.ui.edit.is_adding_transition ^= true;
                            }
                        }

                        // Delete
                        // If a state or transition is selected, then display the delete button
                        if either_selected && app.ui.graph.selected_state().is_some_and(|s| s > 1) {
                            let delete = button(
                                ui,
                                app,
                                include_image!("../../assets/icon/delete.svg"),
                                false,
                            );
                            if delete.clicked() {
                                if let Some(state_selected) = app.ui.graph.selected_state() {
                                    app.turing.remove_state(state_selected)?;
                                }

                                if let Some(transition_selected) = app.ui.graph.selected_transitions()
                                {
                                    app.turing.remove_transitions(
                                        transition_selected.source_id,
                                        transition_selected.target_id,
                                    )?;
                                }
                            }
                            // app.ui.tutorial.add_boxe(
                            //     "delete",
                            //     TutorialBox::new(delete.rect.expand(6.0))
                            //         .with_align(Align2::LEFT_TOP),
                            // );
                        }

                        // Edit the selected transitions or state
                        if either_selected {
                            let edit = button(
                                ui,
                                app,
                                include_image!("../../assets/icon/edit.svg"),
                                false,
                            );
                            if edit.clicked() {
                                if let Some(state_selected) = app.ui.graph.selected_state() {
                                    app.edit_state(state_selected)?
                                }

                                if let Some(transition_selected) = app.ui.graph.selected_transitions()
                                {
                                    app.ui.popup.open(RitmPopupEnum::TransitionEdit(
                                        (
                                            transition_selected.source_id,
                                            transition_selected.target_id,
                                        )
                                            .into(),
                                    ));

                                    app.turing.prepare_transition_edit(
                                        transition_selected.source_id,
                                        transition_selected.target_id,
                                    )?;
                                }
                            }
                        }

                        // Recenter the graph
                        let recenter = button(
                            ui,
                            app,
                            include_image!("../../assets/icon/recenter.svg"),
                            false,
                        );
                        if recenter.clicked() {
                            app.ui.graph.recenter();
                        }

                        // Unpin every state of the graph
                        let unpin = button(
                            ui,
                            app,
                            include_image!("../../assets/icon/unpin.svg"),
                            false,
                        );
                        if unpin.clicked() {
                            app.turing.unpin_all();
                        }

                        Ok::<(), RitmError>(())
                    },
                )
                .response;
        },
    );

    Ok(())
}

fn button(ui: &mut Ui, app: &mut App, icon: ImageSource, selected: bool) -> Response {
    let margin = 5;
    Frame::new()
        .stroke(Stroke::new(1.0, LIGHT_THEME.border))
        .corner_radius(app.settings.edit_button_size / 2.0)
        .fill(LIGHT_THEME.surface)
        .inner_margin(Margin::same(margin))
        .show(ui, |ui| {
            ui.add(
                Button::image(
                    Image::new(icon)
                        .fit_to_exact_size(Vec2::splat(app.settings.edit_button_size))
                        .tint(if selected {
                            LIGHT_THEME.active
                        } else {
                            LIGHT_THEME.icon
                        }),
                )
                .frame(false)
                .corner_radius(app.settings.edit_button_size / 2.0),
            )
        })
        .inner
}