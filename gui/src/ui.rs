use egui::{
    Align, CentralPanel, CornerRadius, Frame, Layout, Margin, Panel, Sense, Ui, UiBuilder, vec2,
};

pub mod code;
pub mod component;
pub mod constant;
pub mod control;
pub mod edit;
pub mod font;
pub mod graph;
pub mod menu;
pub mod popup;
pub mod tape;
pub mod theme;
pub mod tutorial;
pub mod utils;

use crate::{App, error::RitmError};

/// The main Ui layout
/// The principal block are set here
pub fn show(app: &mut App, ui: &mut Ui) -> Result<(), RitmError> {
    // The root Panel
    CentralPanel::default()
        .frame(Frame {
            outer_margin: Margin::same(0),
            inner_margin: Margin::same(0),
            fill: app.theme.background,
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            ui.scope_builder(UiBuilder::new().sense(Sense::empty()), |ui| {
                ui.spacing_mut().indent = 10.0;
                // ui.style_mut().override_font_id = Some(Font::default_medium()); // TODO check if there is not a better way to do that

                // Display code if enabled
                if app.code.is_closed() {
                    closed_code(ui, app)?
                } else {
                    open_code(ui, app)?
                }

                // Tape and Graph
                CentralPanel::default()
                    .frame(Frame {
                        outer_margin: Margin::same(0),
                        inner_margin: Margin::same(0),
                        ..Default::default()
                    })
                    .show_inside(ui, |ui| {
                        // Tape and execution control
                        Panel::top("ribbon&control")
                            .frame(Frame {
                                outer_margin: Margin {
                                    bottom: 10,
                                    ..Default::default()
                                },
                                inner_margin: Margin::same(10),
                                corner_radius: CornerRadius {
                                    sw: 5,
                                    ..Default::default()
                                },
                                fill: app.theme.primary,
                                ..Default::default()
                            })
                            .resizable(false)
                            .show_separator_line(false)
                            .show_inside(ui, |ui| {
                                ui.allocate_ui_with_layout(
                                    vec2(ui.available_width(), ui.ctx().content_rect().height()),
                                    Layout::top_down(Align::Min),
                                    |ui| {
                                        // Tape
                                        tape::show(app, ui);

                                        // Control
                                        control::show(app, ui)?;
                                        Ok::<(), RitmError>(())
                                    },
                                )
                                .inner
                            })
                            .inner?;

                        // Graph visual and edition
                        CentralPanel::default()
                            .frame(Frame {
                                outer_margin: Margin::same(0),
                                inner_margin: Margin::same(0),
                                fill: app.theme.secondary,
                                corner_radius: CornerRadius {
                                    nw: 5,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .show_inside(ui, |ui| {
                                graph::show(app, ui)?;
                                Ok::<(), RitmError>(())
                            });
                        Ok::<(), RitmError>(())
                    });
                Ok::<(), RitmError>(())
            });
        });

    // Display the current popup/modal
    popup::show(ui, app)?;

    Ok::<(), RitmError>(())
}

/// Show only the settings, without the code section
fn closed_code(ui: &mut Ui, app: &mut App) -> Result<(), RitmError> {
    // Menu/Settings
    Panel::left("settings")
        .frame(Frame {
            inner_margin: 5.into(),
            ..Default::default()
        })
        .resizable(false)
        .exact_size(45.0)
        .show_inside(ui, |ui| menu::show(app, ui))
        .inner?;

    Ok::<(), RitmError>(())
}

/// Show the code and the settings
fn open_code(ui: &mut Ui, app: &mut App) -> Result<(), RitmError> {
    // Code and file loading
    Panel::left("code")
        .frame(Frame {
            outer_margin: Margin {
                right: 10,
                ..Default::default()
            },
            fill: app.theme.background,
            ..Default::default()
        })
        .resizable(false)
        .show_separator_line(false)
        .max_size(ui.available_width() / 3.0)
        .min_size(ui.available_width() / 3.0)
        .show_inside(ui, |ui| {
            Panel::top("settings")
                .frame(Frame {
                    fill: app.theme.background,
                    ..Default::default()
                })
                .resizable(false)
                .show_separator_line(false)
                .show_inside(ui, |ui| menu::show(app, ui))
                .inner?;

            CentralPanel::default()
                .frame(Frame {
                    outer_margin: Margin::same(0),
                    inner_margin: Margin::same(0),
                    fill: app.theme.code_background,
                    corner_radius: CornerRadius {
                        ne: 5,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .show_inside(ui, |ui| code::show(app, ui))
                .inner
        });

    Ok(())
}
