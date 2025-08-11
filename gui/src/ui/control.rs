
use egui::{
    include_image, vec2, Align, Align2, Button, Frame, Image, ImageButton, Label, Layout, RichText, Sense, TextEdit, Ui, Vec2
};
use egui_flex::{Flex, FlexAlign, FlexAlignContent, item};

use crate::{
    App,
    ui::{component::grid::Grid, constant::Constant, font::Font},
};

pub fn show(app: &mut App, ui: &mut Ui) {
    Frame::new().show(ui, |ui| {
        ui.set_height(70.0);
        let grid = Grid::new(ui, 2, 3);

        grid.place(ui, 1, 1, |ui| input(app, ui));

        grid.place(ui, 1, 2, |ui| control(app, ui));
        grid.place(ui, 2, 2, |ui| speed_control(app, ui));

        grid.place(ui, 1, 3, |ui| step(app, ui));
        grid.place(ui, 2, 3, |ui| state(app, ui));

    });
}

// #[cfg(not(target_arch = "wasm32"))]
// fn interval(is_next: Arc<AtomicBool>, ctx: Context, duration: Duration) {
//     thread::spawn(move || {
//         thread::sleep(duration);
//         is_next.store(true, Ordering::Relaxed);
//         ctx.request_repaint();
//     });
// }

// #[cfg(target_arch = "wasm32")]
// fn interval(is_next: Arc<AtomicBool>, ctx: Context, duration: Duration) {

//     use wasm_thread as thread;

//     thread::spawn(move || {
//         thread::sleep(duration);
//         is_next.store(true, Ordering::Relaxed);
//         ctx.request_repaint();
//     });
// }


fn input(app: &mut App, ui: &mut Ui) {
    ui.allocate_ui_with_layout(
        ui.available_size(),
        Layout::right_to_left(Align::Center),
        |ui| {
            if ui.add(Button::new("Submit")).clicked() {
                let res = app.turing.reset_word(&app.input);
                match res {
                    Ok(_) => app.next(),
                    Err(e) => println!("{:?}", e),
                }
            }

            ui.add_sized(
                vec2(ui.available_width(), 4.0 + Font::get_heigth(ui, &Font::default())), // 4.0 is 2 times the hardcoded default vertical margin of textedit
                TextEdit::singleline(&mut app.input)
                    .font(Font::default())
                    .hint_text("Input..."),
            );
        },
    );
}

/// Control the iteration of the application, automatic or manual
fn control(app: &mut App, ui: &mut Ui) {
    let icon_size = Vec2::splat(Constant::CONTROL_ICON_SIZE);
    let finished = app.event.is_accepted.is_some();
    let started = finished || app.step.get_nb_iterations() != 0;

    Flex::horizontal()
        .align_items(FlexAlign::Center)
        .align_content(FlexAlignContent::Center)
        .align_items_content(Align2::CENTER_CENTER)
        .h_full()
        .w_full()
        .show(ui, |flex| {
            flex.grow();
            // If playing
            if app.event.is_running {
                // Display pause button
                if flex
                    .add(
                        item(),
                        ImageButton::new(
                            Image::new(include_image!("../../assets/icon/pause.svg"))
                                .fit_to_exact_size(icon_size)
                                .tint(if finished {
                                    app.theme.gray
                                } else {
                                    app.theme.white
                                }),
                        )
                        .frame(false)
                        .sense(if finished {
                            Sense::empty()
                        } else {
                            Sense::click()
                        }),
                    )
                    .clicked()
                {
                    app.event.is_running = false;
                }

            } else {
                // Else display play button
                if flex
                    .add(
                        item(),
                        ImageButton::new(
                            Image::new(include_image!("../../assets/icon/play.svg"))
                                .fit_to_exact_size(icon_size)
                                .tint(if finished {
                                    app.theme.gray
                                } else {
                                    app.theme.white
                                }),
                        )
                        .frame(false)
                        .sense(if finished {
                            Sense::empty()
                        } else {
                            Sense::click()
                        }),
                    )
                    .clicked()
                {
                    app.event.is_running = true;
                }
            }

            // Next button
            if flex
                .add(
                    item(),
                    ImageButton::new(
                        Image::new(include_image!("../../assets/icon/next.svg"))
                            .fit_to_exact_size(icon_size)
                            .tint(if finished {
                                app.theme.gray
                            } else {
                                app.theme.white
                            }),
                    )
                    .frame(false)
                    .sense(if finished {
                        Sense::empty()
                    } else {
                        Sense::click()
                    }),
                )
                .clicked()
            {
                app.next();
            }

            // Reset button
            if flex
                .add(
                    item(),
                    ImageButton::new(
                        Image::new(include_image!("../../assets/icon/reset.svg"))
                            .fit_to_exact_size(icon_size)
                            .tint(if started {
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
}

/// Control the speed of the automatic iteration
fn speed_control(app: &mut App, ui: &mut Ui) {
    let min = app.interval >= 3;
    let max = app.interval <= -5;

    Flex::horizontal()
        .align_content(FlexAlignContent::Center)
        .align_items_content(Align2::CENTER_CENTER)
        .align_items(FlexAlign::Center)
        .w_full()
        .h_full()
        .show(ui, |flex| {
            flex.grow();
            if flex
                .add(
                    item(),
                    ImageButton::new(
                        Image::new(include_image!("../../assets/icon/less.svg"))
                            .fit_to_exact_size(vec2(25.0, 25.0))
                            .tint(if min { app.theme.gray } else { app.theme.white }),
                    )
                    .frame(false),
                )
                .clicked()
            {
                if app.interval < 3 {
                    app.interval += 1;
                }
            }

            flex.add(
                item(),
                Label::new(
                    RichText::new(format!("{}X", 2.0_f32.powi(app.interval as i32 * -1)))
                        .font(Font::default())
                        .color(app.theme.gray),
                ),
            );

            if flex
                .add(
                    item(),
                    ImageButton::new(
                        Image::new(include_image!("../../assets/icon/add.svg"))
                            .fit_to_exact_size(vec2(25.0, 25.0))
                            .tint(if max { app.theme.gray } else { app.theme.white }),
                    )
                    .frame(false),
                )
                .clicked()
            {
                if app.interval > -5 {
                    app.interval -= 1;
                }
            }

            flex.grow()
        });
}

fn step(app: &mut App, ui: &mut Ui) {
    Flex::horizontal()
        .align_content(FlexAlignContent::Center)
        .align_items_content(Align2::CENTER_CENTER)
        .align_items(FlexAlign::Center)
        .w_full()
        .h_full()
        .show(ui, |flex| {
            flex.grow();
            flex.add(
                item(),
                Label::new(format!("Steps : {}", app.step.get_nb_iterations())),
            );
            flex.grow();
        });
}

fn state(app: &mut App, ui: &mut Ui) {
    Flex::horizontal()
        .align_content(FlexAlignContent::Center)
        .align_items_content(Align2::CENTER_CENTER)
        .align_items(FlexAlign::Center)
        .w_full()
        .h_full()
        .show(ui, |flex| {
            flex.grow();
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

            flex.add(item(), Label::new(RichText::new(text).color(color)));
            flex.grow();
        });
}
