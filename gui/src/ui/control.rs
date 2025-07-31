use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use egui::{include_image, vec2, Context, Frame, Image, ImageButton, Label, RichText, Sense, Slider, TextEdit, Ui};
use egui_flex::{Flex, FlexAlign, FlexAlignContent, item};

use crate::{ui::{constant::Constant, font::Font}, App};

pub fn show(app: &mut App, ui: &mut Ui) {
    Frame::new().show(ui, |ui| {
        ui.columns_const(|[col1, col2, col3]| {
            col1.horizontal_centered(|ui| {
                ui.label(RichText::new("Input : ").font(Font::default(ui)).color(app.theme.gray));
                ui.text_edit_singleline(&mut app.input);

                TextEdit::singleline(&mut app.input).font(Font::default(ui))
            });

            Flex::horizontal()
                .h_full()
                .w_full()
                .align_items(FlexAlign::Center)
                .align_content(FlexAlignContent::SpaceBetween)
                .show(col2, |flex| {
                    flex.grow();
                    if app.event.is_running {
                        if flex
                            .add(
                                item(),
                                ImageButton::new(
                                    Image::new(include_image!("../../assets/icon/pause.svg"))
                                        .fit_to_exact_size(vec2(
                                            Constant::CONTROL_ICON_SIZE,
                                            Constant::CONTROL_ICON_SIZE,
                                        ))
                                        .tint(if app.event.is_accepted.is_none() {
                                            app.theme.white
                                        } else {
                                            app.theme.gray
                                        }),
                                )
                                .frame(false)
                                .sense(
                                    if app.event.is_accepted.is_none() {
                                        Sense::click()
                                    } else {
                                        Sense::empty()
                                    },
                                ),
                            )
                            .clicked()
                        {
                            app.event.is_running = false;
                        }

                        if app.event.is_next.load(Ordering::Relaxed) {
                            if app.event.is_running {
                                app.next();
                                app.event.is_next.store(false, Ordering::Relaxed);
                                interval(
                                    app.event.is_next.clone(),
                                    flex.ui().ctx().clone(),
                                    Duration::from_millis(
                                        (2.0_f32.powi(app.interval) * 1000.0) as u64,
                                    ),
                                );
                            }
                        }
                    } else {
                        if flex
                            .add(
                                item(),
                                ImageButton::new(
                                    Image::new(include_image!("../../assets/icon/play.svg"))
                                        .fit_to_exact_size(vec2(
                                            Constant::CONTROL_ICON_SIZE,
                                            Constant::CONTROL_ICON_SIZE,
                                        ))
                                        .tint(if app.event.is_accepted.is_none() {
                                            app.theme.white
                                        } else {
                                            app.theme.gray
                                        }),
                                )
                                .frame(false)
                                .sense(
                                    if app.event.is_accepted.is_none() {
                                        Sense::click()
                                    } else {
                                        Sense::empty()
                                    },
                                ),
                            )
                            .clicked()
                        {
                            app.next();
                            app.event.is_next.store(false, Ordering::Relaxed);
                            app.event.is_running = true;
                            interval(
                                app.event.is_next.clone(),
                                flex.ui().ctx().clone(),
                                Duration::from_millis((2.0_f32.powi(app.interval) * 1000.0) as u64),
                            );
                        }
                    }

                    if flex
                        .add(
                            item(),
                            ImageButton::new(
                                Image::new(include_image!("../../assets/icon/next.svg"))
                                    .fit_to_exact_size(vec2(
                                        Constant::CONTROL_ICON_SIZE,
                                        Constant::CONTROL_ICON_SIZE,
                                    ))
                                    .tint(if app.event.is_accepted.is_none() {
                                        app.theme.white
                                    } else {
                                        app.theme.gray
                                    }),
                            )
                            .frame(false)
                            .sense(
                                if app.event.is_accepted.is_none() {
                                    Sense::click()
                                } else {
                                    Sense::empty()
                                },
                            ),
                        )
                        .clicked()
                    {
                        app.next();
                    }

                    if flex
                        .add(
                            item(),
                            ImageButton::new(
                                Image::new(include_image!("../../assets/icon/reset.svg"))
                                    .fit_to_exact_size(vec2(
                                        Constant::CONTROL_ICON_SIZE,
                                        Constant::CONTROL_ICON_SIZE,
                                    ))
                                    .tint(if app.step.get_nb_iterations() != 0 {
                                        app.theme.white
                                    } else {
                                        app.theme.gray
                                    }),
                            )
                            .frame(false),
                        )
                        .clicked()
                    {
                        app.reset();
                    }
                    flex.grow();
                });

            // col3.horizontal_centered(|ui| ui.label("Accepted (no)"))
            col3.vertical_centered(|ui| {

                ui.add(
                    Slider::new(&mut app.interval, 3..=-5)
                        .custom_formatter(|n, _| format!("{}X", 2.0_f32.powi(n as i32 * -1)))
                        .custom_parser(|s| {
                            let res = s.parse::<f32>();

                            match res {
                                Ok(v) => Some(-v.log2() as f64),
                                Err(_) => None,
                            }
                        }),
                );

                ui.horizontal_centered(|ui| {
                    ui.add(Label::new(format!("Step : {}", app.step.get_nb_iterations())));

                    let (text, color) = if let Some(r) = app.event.is_accepted {
                        if r {
                        ("Accepted", app.theme.valid)

                        } else {
                        ("Rejected", app.theme.invalid)

                        }
                    } else {
                        if app.event.is_running {
                            ("Running", app.theme.gray)
                        } else {
                            ("Idle", app.theme.gray)
                        }
                    };
                    
                    ui.add(Label::new(RichText::new(text).color(color)))
                });
            })
        });
    });
}

fn interval(is_next: Arc<AtomicBool>, ctx: Context, duration: Duration) {
    thread::spawn(move || {
        thread::sleep(duration);
        is_next.store(true, Ordering::Relaxed);
        ctx.request_repaint();
    });
}
