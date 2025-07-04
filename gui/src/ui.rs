use egui::{panel::Side, CentralPanel, Color32, Frame, Margin, SidePanel, TopBottomPanel};


pub mod ribbon;
pub mod palette;
use crate::App;

pub fn show(app: &mut App, ctx: &egui::Context) {


    CentralPanel::default()
    .frame(Frame {
        outer_margin: Margin::same(0),
        inner_margin: Margin::same(10),
        fill: Color32::RED,
        ..Default::default()
    })
    .show(ctx, |ui| {

        // Code and file loading
        SidePanel::left("code")
        .frame(Frame {
            outer_margin: Margin::same(0),
            inner_margin: Margin::same(10),
            fill: Color32::CYAN,
            ..Default::default()
        })
        .resizable(false)
        .min_width(500.0)
        .show_inside(ui, |ui| {
        });


        // Ribbon and Graph
        CentralPanel::default()
        .frame(Frame {
            outer_margin: Margin::same(0),
            inner_margin: Margin::same(0),
            fill: Color32::YELLOW,
            ..Default::default()
        })
        .show_inside(ui, |ui| {

            // Ribbon and execution control
            TopBottomPanel::top("ribbon")
            .frame(Frame {
                outer_margin: Margin::same(0),
                inner_margin: Margin::same(10),
                fill: Color32::LIGHT_GREEN,
                ..Default::default()
            })
            .show_inside(ui, |ui| {

                ribbon::show(app, ui);
            });
        });
    });
}