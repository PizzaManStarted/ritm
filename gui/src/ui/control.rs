use egui::{
    Align, Align2, Button, FontId, Frame, Image, ImageSource, Label, Layout, Response, RichText,
    Sense, Stroke, TextEdit, Ui, Vec2, include_image, vec2,
};
use egui_flex::{Flex, FlexAlign, FlexAlignContent, FlexInstance, item};
use ritm_core::{turing_graph::TuringStateInfo, turing_machine::TuringExecutionSteps};

use crate::{
    App,
    error::RitmError,
    ui::{component::grid::Grid, theme::LIGHT_THEME},
    utils::{constant::Constant, font::Font},
};

#[derive(Default)]
pub struct Control {
    /// User input for the turing machine
    input: String,
    is_running: bool,
    /// power of 2 interval between each iteration
    interval_power: i32,
    last_step_time: f64,
}

impl Control {
    pub fn run(&mut self) {
        self.is_running = true;
    }

    pub fn pause(&mut self) {
        self.is_running = false;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn update_time(&mut self, time: f64) -> bool {
        let time_elasped = time - self.last_step_time;
        if time_elasped >= self.interval().into() {
            self.last_step_time = time
        }
        time_elasped >= self.interval().into()
    }

    pub fn interval(&self) -> f32 {
        2.0_f32.powi(self.interval_power)
    }

    pub fn speed_up(&mut self) {
        if self.interval_power < 3 {
            self.interval_power += 1
        }
    }

    pub fn speed_down(&mut self) {
        if self.interval_power > -5 {
            self.interval_power -= 1
        }
    }

    pub fn input(&self) -> &String {
        &self.input
    }
}

pub fn show(app: &mut App, ui: &mut Ui) -> Result<(), RitmError> {
    let res = Frame::new().show(ui, |ui| {
        ui.set_height(70.0);
        let grid = Grid::new(ui, 2, 3);

        let input = grid.place(ui, 1, 1, |ui| input(app, ui));
        input.inner?;

        grid.place(ui, 1, 2, |ui| control(app, ui));

        grid.place(ui, 2, 2, |ui| speed_control(app, ui));

        grid.place(ui, 1, 3, |ui| step(app, ui));

        grid.place(ui, 2, 3, |ui| state(app, ui));

        Ok::<(), RitmError>(())
    });
    res.inner
}

/// Input section of the controls
///
/// User cna enter the input of the turing machine and submit it
fn input(app: &mut App, ui: &mut Ui) -> Result<(), RitmError> {
    ui.allocate_ui_with_layout(
        ui.available_size(),
        Layout::right_to_left(Align::Center),
        |ui| {
            if ui
                .add(
                    Button::new(
                        RichText::new("Submit")
                            .font(FontId::proportional(Font::MEDIUM))
                            .color(LIGHT_THEME.surface),
                    )
                    .stroke(Stroke::new(1.0, LIGHT_THEME.border))
                    .fill(LIGHT_THEME.primary)
                    .corner_radius(5.0),
                )
                .clicked()
            {
                app.turing.set_word(&app.ui.control.input)?;
            }

            if ui
                .add_sized(
                    vec2(
                        ui.available_width(),
                        4.0 + Font::get_heigth(ui, &Font::default(20.0)),
                    ), // 4.0 is 2 times the hardcoded default vertical margin of textedit
                    TextEdit::singleline(&mut app.ui.control.input)
                        .font(Font::default(20.0))
                        .hint_text(
                            RichText::new("Input...")
                                .font(Font::default(20.0))
                                .color(LIGHT_THEME.text_secondary),
                        )
                        .background_color(LIGHT_THEME.surface)
                        .char_limit(10000),
                )
                .has_focus()
            {
                app.transient.listen_to_keybind = false;
            }
            Ok::<(), RitmError>(())
        },
    )
    .inner
}

/// Control the iteration of the application, automatic or manual
fn control(app: &mut App, ui: &mut Ui) {
    let finished = app.turing.accepted.is_some();
    let initial = app.turing.current_step.get_nb_iterations() == 0 && !finished;

    Flex::horizontal()
        .align_items(FlexAlign::Center)
        .align_content(FlexAlignContent::Center)
        .align_items_content(Align2::CENTER_CENTER)
        .gap(vec2(10.0, 0.0))
        .h_full()
        .w_full()
        .show(ui, |flex| {
            flex.grow();
            // If playing
            if app.ui.control.is_running() {
                // Display pause button
                if button(
                    flex,
                    app,
                    include_image!("../../assets/icon/pause.svg"),
                    finished,
                )
                .clicked()
                {
                    app.ui.control.pause();
                }
            } else {
                // Else display play button
                let button = button(
                    flex,
                    app,
                    include_image!("../../assets/icon/play.svg"),
                    finished,
                );
                if button.clicked() {
                    app.ui.control.run();
                }
            }

            // Next button
            let res = button(
                flex,
                app,
                include_image!("../../assets/icon/next.svg"),
                finished,
            );
            if res.clicked() {
                app.turing.next_step();
            }

            // Reset button
            let res = button(
                flex,
                app,
                include_image!("../../assets/icon/reset.svg"),
                initial,
            );
            if res.clicked() {
                app.reset();
            }
            flex.grow();
        });
}

/// Control the speed of the automatic iteration
/// TODO: make the speed width fixed so the button does not move
fn speed_control(app: &mut App, ui: &mut Ui) {
    let min = app.ui.control.interval_power >= 3;
    let max = app.ui.control.interval_power <= -5;

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
                    Button::image(
                        Image::new(include_image!("../../assets/icon/less.svg"))
                            .fit_to_exact_size(Vec2::splat(25.0))
                            .tint(if min {
                                LIGHT_THEME.border
                            } else {
                                LIGHT_THEME.icon
                            }),
                    )
                    .frame(false),
                )
                .clicked()
            {
                app.ui.control.speed_up();
            }

            flex.add(
                item(),
                Label::new(
                    RichText::new(format!("{}X", 1.0 / app.ui.control.interval()))
                        .font(Font::default(20.0))
                        .color(LIGHT_THEME.icon),
                ),
            );

            if flex
                .add(
                    item(),
                    Button::image(
                        Image::new(include_image!("../../assets/icon/add.svg"))
                            .fit_to_exact_size(Vec2::splat(25.0))
                            .tint(if max {
                                LIGHT_THEME.border
                            } else {
                                LIGHT_THEME.icon
                            }),
                    )
                    .frame(false),
                )
                .clicked()
            {
                app.ui.control.speed_down();
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
                Label::new(
                    RichText::new(t!(
                        "step",
                        "step" = app.turing.current_step.get_nb_iterations()
                    ))
                    .font(Font::default(20.0))
                    .color(LIGHT_THEME.text_primary),
                ),
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
            let (text, color) = {
                if let Some(r) = app.turing.accepted {
                    if r {
                        (t!("accepted"), LIGHT_THEME.success)
                    } else {
                        (t!("rejected"), LIGHT_THEME.error)
                    }
                } else {
                    let is_running = |from: &TuringStateInfo, to: &TuringStateInfo| {
                        (
                            t!("info", "to" = to.get_name(), "from" = from.get_name()),
                            LIGHT_THEME.text_primary,
                        )
                    };
                    match &app.turing.current_step {
                        TuringExecutionSteps::FirstIteration {
                            init_state: _,
                            init_tapes: _,
                        } => (t!("initialization"), LIGHT_THEME.text_primary),
                        TuringExecutionSteps::TransitionTaken {
                            previous_state,
                            reached_state,
                            ..
                        } => is_running(previous_state, reached_state),
                        TuringExecutionSteps::Backtracked {
                            backtracked_iteration,
                            ..
                        } => (
                            t!("backtracking", "step" = backtracked_iteration),
                            LIGHT_THEME.info,
                        ),
                    }
                }
            };

            flex.add(
                item(),
                Label::new(RichText::new(text).color(color).font(Font::default(20.0))),
            );
            flex.grow();
        });
}

fn button(flex: &mut FlexInstance, app: &mut App, icon: ImageSource, disabled: bool) -> Response {
    let icon_size = Vec2::splat(Constant::CONTROL_ICON_SIZE);
    flex.add(
        item(),
        Button::image(
            Image::new(icon)
                .fit_to_exact_size(icon_size)
                .tint(if disabled {
                    LIGHT_THEME.border
                } else {
                    LIGHT_THEME.icon
                }),
        )
        .frame(false)
        .sense(if disabled {
            Sense::empty()
        } else {
            Sense::click()
        }),
    )
}
