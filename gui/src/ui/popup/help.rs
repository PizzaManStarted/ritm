use egui::{
    Align, AtomExt, Button, CentralPanel, Color32, Context, Frame, Id, Image, ImageButton, Label,
    Layout, Margin, Modal, RichText, Separator, SidePanel, Stroke, Ui, Vec2, include_image,
    style::WidgetVisuals, vec2,
};
use egui_flex::{Flex, FlexAlignContent, item};

use crate::{
    App,
    ui::{font::Font, popup::RitmPopup, theme::Theme},
};

pub fn show(app: &mut App, ctx: &Context) {
    Modal::new(Id::new("help"))
        .frame(Frame {
            fill: app.theme.white,
            stroke: Stroke::new(2.0, app.theme.gray),
            inner_margin: Margin::same(10),
            corner_radius: 10.into(),
            ..Default::default()
        })
        .show(ctx, |ui: &mut Ui| {
            ui.set_max_size(ui.ctx().screen_rect().size() * 0.8);
            ui.set_min_size(ui.ctx().screen_rect().size() * 0.8);

            ui.scope(|ui| {
                Theme::set_widget(
                    ui,
                    WidgetVisuals {
                        bg_stroke: Stroke::new(1.0, app.theme.gray),
                        corner_radius: 5.into(),
                        expansion: 3.0,
                        ..app.theme.default_widget()
                    },
                );

                // Popup header
                Flex::horizontal()
                    .w_full()
                    .align_content(FlexAlignContent::SpaceBetween)
                    .show(ui, |flex| {
                        flex.add(
                            item(),
                            Label::new(RichText::new("Help").font(Font::default_big())),
                        );

                        flex.grow();

                        let img = Image::new(include_image!("../../../assets/icon/back.svg"))
                            .fit_to_exact_size(Vec2::splat(25.0))
                            .tint(app.theme.gray)
                            .atom_size(Vec2::splat(25.0));

                        if flex
                            .add(
                                item(),
                                Button::new((img, RichText::new("Back").font(Font::default_big())))
                                    .frame(false),
                            )
                            .clicked()
                        {
                            app.popup = RitmPopup::None;
                        }
                    });

                ui.add(Separator::default().horizontal().grow(10.0));

                SidePanel::left("previous")
                    .exact_width(50.0)
                    .frame(Frame::new().fill(Color32::TRANSPARENT))
                    .show_separator_line(false)
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        Frame::new()
                            .inner_margin(0)
                            .outer_margin(0)
                            .stroke(Stroke::new(1.0, app.theme.gray))
                            .corner_radius(10)
                            .show(ui, |ui| {
                                if ui
                                    .add_sized(
                                        ui.available_size(),
                                        ImageButton::new(
                                            Image::new(include_image!(
                                                "../../../assets/icon/left.svg"
                                            ))
                                            .fit_to_exact_size(Vec2::splat(ui.available_width())),
                                        )
                                        .tint(app.theme.gray)
                                        .frame(false),
                                    )
                                    .clicked()
                                {
                                    if app.help_slide_index > 0 {
                                        app.help_slide_index -= 1;
                                    }
                                }
                            });
                    });

                SidePanel::right("next")
                    .exact_width(50.0)
                    .frame(Frame::new().fill(Color32::TRANSPARENT))
                    .show_separator_line(false)
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        Frame::new()
                            .inner_margin(0)
                            .outer_margin(0)
                            .stroke(Stroke::new(1.0, app.theme.gray))
                            .corner_radius(10)
                            .show(ui, |ui| {
                                if ui
                                    .add_sized(
                                        ui.available_size(),
                                        ImageButton::new(
                                            Image::new(include_image!(
                                                "../../../assets/icon/right.svg"
                                            ))
                                            .fit_to_exact_size(Vec2::splat(ui.available_width())),
                                        )
                                        .tint(app.theme.gray)
                                        .frame(false),
                                    )
                                    .clicked()
                                {
                                    if app.help_slide_index < 6 {
                                        app.help_slide_index += 1;
                                    }
                                }
                            });
                    });

                CentralPanel::default()
                    .frame(
                        Frame::new()
                            .fill(Color32::WHITE)
                            .inner_margin(Margin::symmetric(5, 5)),
                    )
                    .show_inside(ui, |ui| {
                        ui.allocate_ui_with_layout(
                            ui.available_size(),
                            Layout::top_down(Align::Center),
                            |ui| {
                                ui.spacing_mut().item_spacing = vec2(0.0, 20.0);

                                match app.help_slide_index {
                                    0 => welcome(app, ui),
                                    1 => code(app, ui),
                                    2 => settings(app, ui),
                                    3 => ribbons(app, ui),
                                    4 => controls(app, ui),
                                    5 => graph(app, ui),
                                    6 => editing(app, ui),
                                    _ => welcome(app, ui),
                                }
                                
                            },
                        );
                    });
            });

            if app.event.close_popup {
                app.event.close_popup = false;
                app.popup = RitmPopup::None;
            }
        });
}

fn welcome(app: &mut App, ui: &mut Ui) {

    ui.add( Label::new(
        RichText::new(
        "Welcome to RITM !
        This guide will help you understand the interface so you can build incredible machine !
        The interface is divided in 6 sections : The code (1), the settings (2), the ribbons (3),
        the controls (4), the graph (5) and the editing (6).")
            .font(Font::default_medium())
        ).halign(Align::Center)
            .wrap());

    Frame::new()
        .corner_radius(5)
        .stroke(Stroke::new(1.0, app.theme.gray))
        .show(ui, |ui| {
            let img = Image::new(include_image!("../../../assets/help/section.png"))
                .maintain_aspect_ratio(true)
                .corner_radius(5);
            let size = img.load_and_calc_size(ui, ui.available_size());
            if let Some(size) = size {
                ui.set_width(size.x);
            }
            ui.add(img);
        });
}

fn code(app: &mut App, ui: &mut Ui) {
    ui.add( Label::new(
        RichText::new(
        "This is the place where you write the code describing your turing machine.
        Use // for comment, and follow the syntax as below.
        Be aware that any machine-breaking transition will not work.")
            .font(Font::default_medium())
        ).halign(Align::Center)
            .wrap());

    Frame::new()
        .corner_radius(5)
        .stroke(Stroke::new(1.0, app.theme.gray))
        .show(ui, |ui| {
            let img = Image::new(include_image!("../../../assets/help/code.png"))
                .maintain_aspect_ratio(true)
                .corner_radius(5);
            let size = img.load_and_calc_size(ui, ui.available_size());
            if let Some(size) = size {
                ui.set_width(size.x);
            }
            ui.add(img);
        });
}

fn settings(app: &mut App, ui: &mut Ui) {
    ui.add( Label::new(
        RichText::new(
        "You can find multiple buttons above the code section.
1 - Settings : Change the differents settings of the application by clicking on it
2 - Save : Save your current code ! (Not implemented)
3 - Load : Load a default machine or a custom one using a .tm file
4 - Help : Access this guide by clicking on it
5 - Graph : Convert the current code to a graph, to visualize and edit
6 - Close : Close the code section to give more space to the graph")
            .font(Font::default_medium())
        ).halign(Align::LEFT)
            .wrap());

    Frame::new()
        .corner_radius(5)
        .stroke(Stroke::new(1.0, app.theme.gray))
        .show(ui, |ui| {
            let img = Image::new(include_image!("../../../assets/help/settings.png"))
                .maintain_aspect_ratio(true)
                .corner_radius(5);
            let size = img.load_and_calc_size(ui, ui.available_size());
            if let Some(size) = size {
                ui.set_width(size.x);
            }
            ui.add(img);
        });
}

fn ribbons(app: &mut App, ui: &mut Ui) {
    ui.add( Label::new(
        RichText::new(
        "The first ribbon is the reading ribbon, and the rest are writing ribbon.
        The number of writing ribbon is determined by the code, or by default one writing ribbon.
        You can change this value in the settings*.
        
        (*Warning : changing this value will reset the machine !)")
            .font(Font::default_medium())
        ).halign(Align::LEFT)
            .wrap());

    Frame::new()
        .corner_radius(5)
        .stroke(Stroke::new(1.0, app.theme.gray))
        .show(ui, |ui| {
            let img = Image::new(include_image!("../../../assets/help/ribbons.png"))
                .maintain_aspect_ratio(true)
                .corner_radius(5);
            let size = img.load_and_calc_size(ui, ui.available_size());
            if let Some(size) = size {
                ui.set_width(size.x);
            }
            ui.add(img);
        });
}

fn controls(app: &mut App, ui: &mut Ui) {
    ui.add( Label::new(
        RichText::new(
        "In this section you can find the input submitted to the machine on the left.
        In the middle you can run automatically or step by step and reset the machine.
        The speed of the autoplay can be changed below, the default being 1 second between steps.
        Finally you can monitor the number of steps done and the state of the machine.
        The message 'Rejected' will be displayed if the machine fail and 'Accepted' if it succeed.")
            .font(Font::default_medium())
        ).halign(Align::Center)
            .wrap());

    Frame::new()
        .corner_radius(5)
        .stroke(Stroke::new(1.0, app.theme.gray))
        .show(ui, |ui| {
            let img = Image::new(include_image!("../../../assets/help/controls.png"))
                .maintain_aspect_ratio(true)
                .corner_radius(5);
            let size = img.load_and_calc_size(ui, ui.available_size());
            if let Some(size) = size {
                ui.set_width(size.x);
            }
            ui.add(img);
        });
}

fn graph(app: &mut App, ui: &mut Ui) {
    ui.add( Label::new(
        RichText::new(
        "The most important feature : editable graph.
The code section of the application is optionnal, as turing machine can be graphically edited.
The default state (I-nitial, A-ccept and R-eject) are immuable.
The graph can be moved around, zoomed in and out and each state can move independantly. Once a state is moved, it will stay where it was put.")
            .font(Font::default_medium())
        ).halign(Align::LEFT)
            .wrap());

    Frame::new()
        .corner_radius(5)
        .stroke(Stroke::new(1.0, app.theme.gray))
        .show(ui, |ui| {
            let img = Image::new(include_image!("../../../assets/help/graph.png"))
                .maintain_aspect_ratio(true)
                .corner_radius(5);
            let size = img.load_and_calc_size(ui, ui.available_size());
            if let Some(size) = size {
                ui.set_width(size.x);
            }
            ui.add(img);
        });
}

fn editing(app: &mut App, ui: &mut Ui) {
    ui.add( Label::new(
        RichText::new(
        "At the bottom of the graph you can find the editing option.
Circled plus : Add a state on your next click on the graph
Path : Add a transition from the selected state to the next selected
Trashbin : Delete the state/transition selected
Pen & paper : Edit the transition (state is not implemented)
Arrows and dot : Recenter the graph, for better view
Crossed pin : 'Unpin' every states, reorganizing themselves")
            .font(Font::default_medium())
        ).halign(Align::LEFT)
            .wrap());

    Frame::new()
        .corner_radius(5)
        .stroke(Stroke::new(1.0, app.theme.gray))
        .show(ui, |ui| {
            let img = Image::new(include_image!("../../../assets/help/edit1.png"))
                .maintain_aspect_ratio(true)
                .corner_radius(5);
            let size = img.load_and_calc_size(ui, ui.available_size());
            if let Some(size) = size {
                ui.set_width(size.x);
            }
            ui.add(img);
        });
}

fn keybind(app: &mut App, ui: &mut Ui) {
    ui.add( Label::new(
        RichText::new(
        "You can find multiple buttons above the code section.
        1 - Settings : Change the differents settings of the application by clicking on it
        2 - Save : Save your current code ! (Not implemented)
        3 - Load : Load a default machine or a custom one using a .tm file
        4 - Help : Access this guide by clicking on it
        5 - Graph : Convert the current code to a graph, to visualize and edit
        6 - Close : Close the code section to give more space to the graph")
            .font(Font::default_medium())
        ).halign(Align::Center)
            .wrap());

    Frame::new()
        .corner_radius(5)
        .stroke(Stroke::new(1.0, app.theme.gray))
        .show(ui, |ui| {
            let img = Image::new(include_image!("../../../assets/help/settings.png"))
                .maintain_aspect_ratio(true)
                .corner_radius(5);
            let size = img.load_and_calc_size(ui, ui.available_size());
            if let Some(size) = size {
                ui.set_width(size.x);
            }
            ui.add(img);
        });
}
