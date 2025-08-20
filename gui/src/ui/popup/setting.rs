use egui::{
    Checkbox, Context, Frame, Grid, Id, Label, Margin, Modal, RichText,
    Separator, Stroke, Ui, style::WidgetVisuals, vec2,
};
use ritm_core::turing_machine::Mode;

use crate::{
    ui::{component::combobox::ComboBox, font::Font, popup::Popup, theme::Theme}, App
};

pub fn show(app: &mut App, ctx: &Context) {
    Modal::new(Id::new("setting"))
        .frame(Frame {
            fill: app.theme.white,
            stroke: Stroke::new(2.0, app.theme.gray),
            inner_margin: Margin::same(20),
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

                ui.add(Label::new(
                    RichText::new("Settings").font(Font::default_big()),
                ));
                ui.add(Separator::default().horizontal().grow(10.0));

                Grid::new("settings")
                    .spacing(vec2(40.0, 10.0))
                    .show(ui, |ui| {
                        ui.add(Label::new(
                            RichText::new("Turing machine mode").font(Font::default_medium()),
                        ));

                        ComboBox::from_id_salt("setting_turing_mode")
                            .selected_text(
                                RichText::new(match app.settings.turing_machine_mode {
                                    Mode::SaveAll => "Unsafe",
                                    Mode::StopAfter(_) => "Safe",
                                    Mode::StopFirstReject => "Error",
                                })
                                .font(Font::default_medium()),
                            )
                            .width(20.0) // TODO change and think about this value, I hardcoded it
                            .show_ui(ui, |ui| {
                                if app.settings.turing_machine_mode != Mode::SaveAll {
                                    ui.selectable_value(
                                        &mut app.settings.turing_machine_mode,
                                        Mode::SaveAll,
                                        "Unsafe",
                                    );
                                }
                                if let Mode::StopAfter(_) = app.settings.turing_machine_mode {} else {
                                    ui.selectable_value(
                                        &mut app.settings.turing_machine_mode,
                                        Mode::StopAfter(500),
                                        "Safe",
                                    );
                                }
                            });
                        ui.end_row();

                        ui.add(Label::new(
                            RichText::new("Toggle after action").font(Font::default_medium()),
                        ));
                        ui.add(Checkbox::without_text(
                            &mut app.settings.toggle_after_action,
                        ));
                        ui.end_row();
                    });
            });

            if app.event.close_popup {
                app.event.close_popup = false;
                app.popup = Popup::None;
            }
        });
}
