use egui::{CentralPanel, CornerRadius, Frame, Margin, SidePanel, TopBottomPanel};


pub mod ribbon;
pub mod graph;
pub mod edit;
pub mod constant;
pub mod theme;
pub mod utils;
pub mod font;
pub mod control;
pub mod component;

use crate::{ui::font::Font, App};

pub fn show(app: &mut App, ctx: &egui::Context) {


    CentralPanel::default()
    .frame(Frame {
        outer_margin: Margin::same(0),
        inner_margin: Margin::same(0),
        fill: app.theme.background,
        ..Default::default()
    })
    .show(ctx, |ui| {
        ui.spacing_mut().indent = 10.0;

        ui.style_mut().override_font_id = Some(Font::default(ui)); // TODO check if there is not a better way to do that

        // Code and file loading
        SidePanel::left("code")
        .frame(Frame {
            outer_margin: Margin { right: 10, ..Default::default()},
            fill: app.theme.code,
            ..Default::default()
        })
        .resizable(false)
        .show_separator_line(false)
        .min_width(300.0)
        .default_width(300.0)
        .show_inside(ui, |ui| {
            ui.label("lol");
        });


        // Ribbon and Graph
        CentralPanel::default()
        .frame(Frame {
            outer_margin: Margin::same(0),
            inner_margin: Margin::same(0),
            ..Default::default()
        })
        .show_inside(ui, |ui| {

            // Ribbon and execution control
            TopBottomPanel::top("ribbon")
            .frame(Frame {
                outer_margin: Margin { bottom: 10, ..Default::default() },
                inner_margin: Margin::same(10),
                corner_radius: CornerRadius::same(5),
                fill: app.theme.ribbon,
                ..Default::default()
            })
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                ribbon::show(app, ui);
                control::show(app, ui);
            });


            // Graph visual and edition
            CentralPanel::default()
            .frame(Frame {
                outer_margin: Margin::same(0),
                inner_margin: Margin::same(0),
                fill: app.theme.graph,
                corner_radius: CornerRadius::same(5),
                ..Default::default()
            })
            .show_inside(ui, |ui| {
                graph::show(app, ui);
            });
        });
    });
}