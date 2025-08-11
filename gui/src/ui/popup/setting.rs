use egui::{Context, Frame, Id, Margin, Modal, Stroke, Ui};

use crate::App;

pub fn show(app: &mut App, ctx: &Context) {
    Modal::new(Id::new("setting"))
        .frame(Frame {
            fill: app.theme.white,
            stroke: Stroke::new(2.0, app.theme.gray),
            inner_margin: Margin::same(10),
            corner_radius: 10.into(),
            ..Default::default()
        })
        .show(ctx, |ui: &mut Ui| {

        });
}