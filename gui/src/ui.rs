use egui::{CentralPanel, CornerRadius, Frame, Margin, Panel, Shape, Stroke, Ui, Vec2, pos2, vec2};

pub mod code;
pub mod component;
pub mod control;
pub mod edit;
pub mod graph;
pub mod menu;
pub mod popup;
pub mod tape;
pub mod theme;
pub mod tutorial;

use crate::{
    App,
    error::RitmError,
    ui::{
        code::Code,
        control::Control,
        edit::Edit,
        graph::Graph,
        menu::Menu,
        popup::{RitmPopup, save::Save},
        theme::LIGHT_THEME,
    },
};

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct RitmUi {
    #[serde(skip)]
    pub edit: Edit,

    #[serde(skip)]
    pub graph: Graph,

    #[serde(skip)]
    pub control: Control,

    #[serde(skip)]
    pub menu: Menu,

    #[serde(skip)]
    pub popup: RitmPopup,

    pub code: Code,

    #[serde(skip)]
    pub save: Save,
}

/// The main Ui layout
/// The principal block are set here
pub fn show(app: &mut App, ui: &mut Ui) -> Result<(), RitmError> {
    if ui.content_rect().width() <= 800.0 {
        mobile_show(app, ui);
    } else {
        desktop_show(app, ui);
    }

    Ok::<(), RitmError>(())
}

/// Desktop display
///
/// This version feature a topbar with every action the user can do, which result
/// in a more intuitive UI since there is enough room to put the icon AND the text
pub fn desktop_show(app: &mut App, ui: &mut Ui) {
    // The root/background frame
    CentralPanel::default()
        .frame(Frame::new().fill(LIGHT_THEME.background))
        .show_inside(ui, |ui| {
            // The topbar action
            Panel::top("Menu")
                .frame(Frame::new().inner_margin(Margin::same(5)))
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    Menu::desktop_show(app, ui);
                });
            // The rest of the application
            CentralPanel::default()
                .frame(Frame::new().fill(LIGHT_THEME.background))
                .show_inside(ui, |ui| {
                    // The code section on the left, 1/3 of the available width
                    Panel::left("Code")
                        .show_separator_line(false)
                        .resizable(false)
                        .size_range(0.0..=ui.content_rect().width() / 3.0)
                        .frame(
                            Frame::new()
                                .outer_margin(Margin {
                                    right: 10,
                                    ..Default::default()
                                })
                                .fill(LIGHT_THEME.code_background)
                                .corner_radius(CornerRadius {
                                    ne: 10,
                                    ..Default::default()
                                }),
                        )
                        .show_inside(ui, |ui| {
                            code::show(app, ui);
                        });

                    // The graph and controls section on the right
                    CentralPanel::default()
                        .frame(Frame::NONE)
                        .show_inside(ui, |ui| {
                            ui.spacing_mut().item_spacing = Vec2::ZERO;

                            // The tapes and controls on top of the graph
                            Panel::top("Tapes&Control")
                                .show_separator_line(false)
                                .frame(
                                    Frame::new()
                                        .fill(LIGHT_THEME.secondary)
                                        .corner_radius(CornerRadius {
                                            sw: 10,
                                            nw: 10,
                                            ..Default::default()
                                        })
                                        .stroke(Stroke::new(1.0, LIGHT_THEME.border))
                                        .inner_margin(Margin::same(10)),
                                )
                                .show_inside(ui, |ui| {
                                    ui.vertical(|ui| {
                                        ui.spacing_mut().item_spacing.y = 10.0;
                                        tape::show(app, ui);
                                        control::show(app, ui);
                                    })
                                });

                            // This allow to have a small right "border" on the small gap between
                            // the controls and the graph. It look ugly without.
                            let rect = ui.allocate_space(vec2(ui.available_width(), 10.0)).1;
                            ui.painter_at(rect).add(Shape::LineSegment {
                                points: [
                                    pos2(rect.right() - 1.0, rect.top()),
                                    pos2(rect.right() - 1.0, rect.top() + 10.0),
                                ],
                                stroke: Stroke::new(1.0, LIGHT_THEME.border),
                            });

                            // The graph where the user can edit graphically the turing machines
                            CentralPanel::default()
                                .frame(
                                    Frame::new()
                                        .fill(LIGHT_THEME.scene)
                                        .corner_radius(CornerRadius {
                                            nw: 10,
                                            ..Default::default()
                                        })
                                        .stroke(Stroke::new(1.0, LIGHT_THEME.border)),
                                )
                                .show_inside(ui, |ui| {
                                    let _ = graph::show(app, ui);
                                })
                        });
                });

            // Show the popups on top of everyting else
            let _ = popup::show(ui, app);
        });
}

/// Mobile display
///
/// This version disable the whole code part of the application since there is not enough room for it.
/// A icon-only bottombar allow the user to use the limited action in a more compact way.
///
/// This version is only for showcase, i.e show existing creations or how the application feel,
/// since editing is harder on small devices.
pub fn mobile_show(app: &mut App, ui: &mut Ui) {
    // Root
    CentralPanel::default()
        .frame(Frame::new().fill(LIGHT_THEME.background))
        .show_inside(ui, |ui| {
            // Tapes and Controlss
            Panel::top("Tapes&Controls")
                .frame(Frame::new().fill(LIGHT_THEME.secondary))
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui.vertical(|ui| {
                        tape::show(app, ui);
                        let _ = control::show(app, ui);
                    })
                });

            Panel::bottom("Menu")
                .frame(Frame::NONE)
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    Menu::mobile_show(app, ui);
                });
            // Graph
            CentralPanel::default()
                .frame(Frame::new().fill(LIGHT_THEME.scene))
                .show_inside(ui, |ui| {
                    let _ = graph::show(app, ui);
                });
        });
}