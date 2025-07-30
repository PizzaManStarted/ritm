use std::process::exit;

use egui::{
    Align, AtomExt, Button, Color32, Frame, Id, Image, ImageButton, Label, Layout, Margin, Modal,
    RichText, ScrollArea, Stroke, TextEdit, Ui, include_image, style::WidgetVisuals, vec2,
};
use ritm_core::turing_state::TuringDirection;

use crate::{
    App,
    turing::{State, TuringTransitionString},
    ui::{component::combobox::ComboBox, font::Font, theme::Theme},
};

pub fn show(app: &mut App, ui: &mut Ui) {
    Modal::new(Id::new("transition_edit"))
        .frame(Frame {
            fill: Color32::WHITE,
            stroke: Stroke::new(2.0, Color32::GRAY),
            inner_margin: Margin::same(10),
            ..Default::default()
        })
        .show(ui.ctx(), |ui: &mut Ui| {
            let selected_transition = app.selected_transition.unwrap();

            // Main layout
            ui.vertical_centered(|ui| {
                ui.style_mut().spacing.item_spacing = vec2(0.0, 10.0);

                // The transition "name" : From state To state
                ui.label(
                    RichText::new(format!(
                        "{} -> {}",
                        State::get(app, selected_transition.0).name,
                        State::get(app, selected_transition.1).name
                    ))
                    .font(Font::default()),
                );

                // List of the rule
                let width = ui
                    .vertical_centered(|ui| {
                        ui.style_mut().spacing.item_spacing = vec2(0.0, 10.0);

                        // Copy the transition (Modify without change)
                        if app.rules_edit.is_empty() {
                            app.rules_edit = app
                                .turing
                                .graph_ref()
                                .get_state(selected_transition.0)
                                .unwrap()
                                .transitions
                                .iter()
                                .map(|transition| TuringTransitionString::from(transition))
                                .collect::<Vec<TuringTransitionString>>();
                        }
                        ScrollArea::vertical()
                            .max_height(ui.ctx().input(|i| i.screen_rect()).height()/2.0)
                            .show(ui, |ui| {
                                // Create a row with the rule of each transition
                                let count = app.rules_edit.len();
                                let mut marked_to_delete: Vec<usize> = vec![];
                                for transition_index in 0..count {
                                    // Skip the transition not relevant
                                    if app.rules_edit[transition_index].index_to_state.unwrap()
                                        != selected_transition.1
                                    {
                                        continue;
                                    }

                                    // Right to left to allow the text edit to take the remaining space
                                    // To remove later with a system based on the number of ribbons
                                    ui.allocate_ui_with_layout(
                                        vec2(250.0, 10.0),
                                        Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            // Delete
                                            if ui
                                                .add(
                                                    ImageButton::new(
                                                        Image::new(include_image!(
                                                            "../../../assets/icon/delete.svg"
                                                        ))
                                                        .fit_to_exact_size(vec2(35.0, 35.0))
                                                        .tint(Color32::RED),
                                                    )
                                                    .frame(false),
                                                )
                                                .clicked()
                                            {
                                                marked_to_delete.push(transition_index);
                                            }

                                            // Undo change
                                            if ui
                                                .add(
                                                    ImageButton::new(
                                                        Image::new(include_image!(
                                                            "../../../assets/icon/stateplus.svg"
                                                        ))
                                                        .fit_to_exact_size(vec2(35.0, 35.0))
                                                        .tint(Color32::GREEN),
                                                    )
                                                    .frame(false),
                                                )
                                                .clicked()
                                            {
                                                todo!("Implement undo change")
                                            }

                                            // Margin for textedit
                                            let margin = vec2(3.0, 2.0);

                                            // Combobox use global variable
                                            ui.spacing_mut().button_padding = margin;
                                            ui.visuals_mut().widgets.inactive.weak_bg_fill =
                                                Color32::LIGHT_GRAY;
                                            ui.visuals_mut().widgets.active.weak_bg_fill =
                                                ui.visuals_mut().widgets.inactive.weak_bg_fill;
                                            ui.visuals_mut().widgets.hovered.weak_bg_fill =
                                                ui.visuals_mut().widgets.inactive.weak_bg_fill;
                                            ui.visuals_mut().widgets.open.weak_bg_fill =
                                                ui.visuals_mut().widgets.inactive.weak_bg_fill;

                                            // Make access easier
                                            let transition = &mut app.rules_edit[transition_index];

                                            // Layout single character TextEdit
                                            ui.with_layout(
                                                Layout::left_to_right(Align::Center),
                                                |ui| {
                                                    ui.spacing_mut().item_spacing = vec2(5.0, 0.0);
                                                    ui.set_height(
                                                        Font::get_heigth(ui, &Font::default())
                                                            + margin.y * 2.0,
                                                    );

                                                    // Textedit for each reading character
                                                    for i in 0..transition.chars_read.len() {
                                                        // TextEdit don't accept char, so must use a String
                                                        ui.scope(|ui| {
                                                            ui.visuals_mut().selection.stroke =
                                                                Stroke::NONE;
                                                            if transition.chars_read[i].is_empty() {
                                                                Theme::set_widget(
                                                                    ui,
                                                                    WidgetVisuals {
                                                                        bg_stroke: Stroke::new(
                                                                            1.0,
                                                                            Color32::RED,
                                                                        ),
                                                                        ..app.theme.default_widget()
                                                                    },
                                                                );
                                                            }
                                                            ui.add(
                                                                TextEdit::singleline(
                                                                    &mut transition.chars_read[i],
                                                                )
                                                                .background_color(
                                                                    Color32::LIGHT_GRAY,
                                                                )
                                                                .frame(true)
                                                                .font(Font::default())
                                                                .margin(margin)
                                                                .desired_width(Font::get_width(
                                                                    ui,
                                                                    &Font::default(),
                                                                ))
                                                                .char_limit(1),
                                                            );
                                                        });

                                                        // Aesthetic purpose, add a colon between each reading char
                                                        if i != transition.chars_read.len() - 1 {
                                                            ui.add(Label::new(","));
                                                        }
                                                    }

                                                    // Aesthetic purpose, add an arrow between the "condition" and the "action"
                                                    ui.add(Label::new("->"));

                                                    // Reading ribbon moving direction
                                                    ComboBox::from_id_salt(format!(
                                                        "moveread-{}",
                                                        transition_index
                                                    ))
                                                    .selected_text(
                                                        RichText::new(match transition.move_read {
                                                            TuringDirection::Left => {
                                                                "L".to_string()
                                                            }
                                                            TuringDirection::Right => {
                                                                "R".to_string()
                                                            }
                                                            TuringDirection::None => {
                                                                "N".to_string()
                                                            }
                                                        })
                                                        .font(Font::default()),
                                                    )
                                                    .width(20.0) // TODO change and think about this value, I hardcoded it
                                                    .show_ui(ui, |ui| {
                                                        ui.selectable_value(
                                                            &mut transition.move_read,
                                                            TuringDirection::Right,
                                                            "Right",
                                                        );
                                                        ui.selectable_value(
                                                            &mut transition.move_read,
                                                            TuringDirection::Left,
                                                            "Left",
                                                        );
                                                        ui.selectable_value(
                                                            &mut transition.move_read,
                                                            TuringDirection::None,
                                                            "None",
                                                        );
                                                    });

                                                    // Again, aesthetic purpose
                                                    ui.add(Label::new(","));

                                                    // Textedit for each reading character
                                                    for i in 0..transition.chars_write.len() {
                                                        // TextEdit don't accept char, so must use a String
                                                        ui.scope(|ui| {
                                                            if transition.chars_write[i]
                                                                .0
                                                                .is_empty()
                                                            {
                                                                Theme::set_widget(
                                                                    ui,
                                                                    WidgetVisuals {
                                                                        bg_stroke: Stroke::new(
                                                                            1.0,
                                                                            Color32::RED,
                                                                        ),
                                                                        ..app.theme.default_widget()
                                                                    },
                                                                );
                                                            }
                                                            ui.add(
                                                                TextEdit::singleline(
                                                                    &mut transition.chars_write[i]
                                                                        .0,
                                                                )
                                                                .background_color(
                                                                    Color32::LIGHT_GRAY,
                                                                )
                                                                .frame(true)
                                                                .font(Font::default())
                                                                .margin(margin)
                                                                .desired_width(Font::get_width(
                                                                    ui,
                                                                    &Font::default(),
                                                                ))
                                                                .char_limit(1),
                                                            );
                                                        });

                                                        // Again, aesthetic purpose
                                                        ui.add(Label::new(","));

                                                        // Reading ribbon moving direction
                                                        ComboBox::from_id_salt(format!(
                                                            "movewrite-{}",
                                                            transition_index
                                                        ))
                                                        .selected_text(
                                                            RichText::new(
                                                                match transition.chars_write[i].1 {
                                                                    TuringDirection::Left => {
                                                                        "L".to_string()
                                                                    }
                                                                    TuringDirection::Right => {
                                                                        "R".to_string()
                                                                    }
                                                                    TuringDirection::None => {
                                                                        "N".to_string()
                                                                    }
                                                                },
                                                            )
                                                            .font(Font::default()),
                                                        )
                                                        .width(20.0) // TODO change and think about this value, I hardcoded it
                                                        .show_ui(ui, |ui| {
                                                            ui.selectable_value(
                                                                &mut transition.move_read,
                                                                TuringDirection::Right,
                                                                "Right",
                                                            );
                                                            ui.selectable_value(
                                                                &mut transition.move_read,
                                                                TuringDirection::Left,
                                                                "Left",
                                                            );
                                                            ui.selectable_value(
                                                                &mut transition.move_read,
                                                                TuringDirection::None,
                                                                "None",
                                                            );
                                                        });

                                                        // Again, aesthetic purpose
                                                        if i != transition.chars_write.len() - 1 {
                                                            ui.add(Label::new(","));
                                                        }
                                                    }
                                                },
                                            );
                                        },
                                    );
                                }
                            
                                marked_to_delete.iter().for_each(|transition| { app.rules_edit.remove(*transition); });
                            });

                        if ui
                            .add(
                                ImageButton::new(
                                    Image::new(include_image!(
                                        "../../../assets/icon/stateplus.svg"
                                    ))
                                    .fit_to_exact_size(vec2(35.0, 35.0))
                                    .tint(app.theme.gray),
                                )
                                .frame(false),
                            )
                            .clicked()
                        {
                            app.rules_edit.push(TuringTransitionString {
                                chars_read: vec!["ç".to_string(); app.turing.graph_ref().get_k() + 1],
                                move_read: TuringDirection::None,
                                chars_write: vec![
                                    ("ç".to_string(), TuringDirection::None);
                                    app.turing.graph_ref().get_k()
                                ],
                                index_to_state: Some(selected_transition.1),
                            });
                        }
                    })
                    .response
                    .rect
                    .width();

                ui.set_width(width);

                ui.spacing_mut().button_padding = vec2(0.0, 8.0);
                ui.spacing_mut().item_spacing = vec2(10.0, 0.0);
                ui.columns(2, |columns| {
                    if columns[0]
                        .add(
                            Button::new(
                                RichText::new("Save")
                                    .color(Theme::constrast_color(app.theme.valid))
                                    .font(Font::default())
                                    .atom_grow(true),
                            )
                            .stroke(Stroke::new(2.0, app.theme.gray))
                            .fill(app.theme.valid)
                            .corner_radius(10.0),
                        )
                        .clicked()
                    {
                        app.apply_transition_change();
                    };

                    if columns[1]
                        .add(
                            Button::new(
                                RichText::new("Cancel")
                                    .color(Theme::constrast_color(app.theme.invalid))
                                    .font(Font::default())
                                    .atom_grow(true),
                            )
                            .stroke(Stroke::new(2.0, app.theme.gray))
                            .fill(app.theme.invalid)
                            .corner_radius(10.0),
                        )
                        .clicked()
                    {
                        app.cancel_transition_change();
                    };
                });
            });
        });
}
