use egui::{
    Align, Color32, Frame, Id, Image, ImageButton, LayerId, Layout, Margin, Rect, Shadow, Stroke,
    Ui, UiBuilder, include_image, vec2,
};

use crate::App;

/// Control of the graph
pub fn show(app: &mut App, ui: &mut Ui, rect: Rect) {

    // The parent ui paint on the background layer, so we need to change it to a higher layer
    let layer = LayerId::new(egui::Order::Middle, Id::new("edit"));
    ui.scope_builder(UiBuilder::new().layer_id(layer), |ui| {

        // Floating control absolute position
        ui.allocate_new_ui(
            UiBuilder::new()
                .max_rect(Rect::from_center_size(
                    rect.center_bottom() + vec2(0.0, -100.0),
                    (rect.width(), 1.0).into(),
                ))
                .layout(Layout::top_down(Align::Center).with_cross_align(Align::Center)),
            // .layer_id(layer),
            |ui| {
                // Used for background
                Frame::new()
                    .fill(app.theme.color2)
                    .inner_margin(Margin::symmetric(20, 10))
                    .corner_radius(15)
                    .stroke(Stroke::new(1.0, Color32::from_gray(190)))
                    .shadow(Shadow {
                        offset: [0, 4],
                        blur: 4,
                        spread: 0,
                        color: Color32::from_black_alpha(25),
                    })
                    .show(ui, |ui| {

                        // Need to compute the width of the menu to center it
                        let mut width = 0.0;
                        let mut count = 0;
                        if app.selected_state.is_none() && app.selected_transition.is_none() {
                            count += 1;
                        }
                        if app.selected_state.is_some() {
                            count += 1;
                        }
                        if app.selected_state.is_some() || app.selected_transition.is_some() {
                            count += 1;
                        }
                        width += 35.0 * count as f32 + 20.0 * (count - 1) as f32;

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
                                                    Color32::BLUE
                                                } else {
                                                    Color32::BLACK
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
                                                    "../../assets/icon/path.svg"
                                                ))
                                                .fit_to_exact_size(vec2(35.0, 35.0))
                                                .tint(if app.event.is_adding_transition {
                                                    Color32::BLUE
                                                } else {
                                                    Color32::BLACK
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
                                if app.selected_state.is_some() || app.selected_transition.is_some()
                                {
                                    if ui
                                        .add(
                                            ImageButton::new(
                                                Image::new(include_image!(
                                                    "../../assets/icon/delete.svg"
                                                ))
                                                .fit_to_exact_size(vec2(35.0, 35.0))
                                                .tint(Color32::BLACK),
                                            )
                                            .frame(false),
                                        )
                                        .clicked()
                                    {

                                        // Delete the selected state/transition
                                    }
                                }
                            },
                        );
                    });
            },
        );
    });
}
