use egui::{Context, Frame, Id, Margin, Modal, Stroke, Ui};

use crate::{ui::popup::Popup, App};

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


            if app.event.close_popup {
                app.event.close_popup = false;
                app.popup = Popup::None;
            }
        });
}