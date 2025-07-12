use egui::{panel::Side, CentralPanel, Color32, CornerRadius, Frame, Margin, SidePanel, TopBottomPanel};


pub mod ribbon;
pub mod graph;
pub mod edit;
pub mod constant;
pub mod theme;
pub mod utils;
pub mod font;
use crate::App;

pub fn show(app: &mut App, ctx: &egui::Context) {


    CentralPanel::default()
    .frame(Frame {
        outer_margin: Margin::same(0),
        inner_margin: Margin::same(10),
        fill: Color32::WHITE,
        ..Default::default()
    })
    .show(ctx, |ui| {
        ui.spacing_mut().indent = 10.0;

        // Code and file loading
        SidePanel::left("code")
        .frame(Frame {
            outer_margin: Margin { right: 10, ..Default::default()},
            inner_margin: Margin::same(10),
            fill: Color32::CYAN,
            ..Default::default()
        })
        .resizable(false)
        .show_separator_line(false)
        .min_width(500.0)
        .show_inside(ui, |ui| {
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
                fill: app.theme.color3,
                ..Default::default()
            })
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                ribbon::show(app, ui);
            });


            // Graph visual and edition
            CentralPanel::default()
            .frame(Frame {
                outer_margin: Margin::same(0),
                inner_margin: Margin::same(0),
                corner_radius: CornerRadius::same(5),
                ..Default::default()
            })
            .show_inside(ui, |ui| {
                graph::show(app, ui);
            });
        });
    });
}