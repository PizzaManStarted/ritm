use std::{collections::VecDeque, mem::replace, todo};

use egui::{
    Align, Atom, AtomLayout, Button, Color32, CornerRadius, FontId, Frame, Id, Image, ImageSource,
    Layout, Margin, Modal, RichText, Sense, Separator, Stroke, TextWrapMode, Ui, Vec2,
    include_image, vec2,
};

use crate::{
    App,
    error::RitmError,
    turing::TransitionId,
    ui::{
        popup::{convert::Convert, save::Save},
        theme::LIGHT_THEME,
    },
    utils::font::Font,
};

pub mod convert;
pub mod save;
pub mod settings;
pub mod state_edit;
pub mod transition_edit;

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum RitmPopupEnum {
    NewTransition,
    NewState,
    TransitionEdit(TransitionId),
    StateEdit(usize),
    Settings,
    Confirmation,
    Help,
    Save,
    Convert,
    Error(String),
}

impl<'a> RitmPopupEnum {
    pub fn title(&self) -> String {
        match self {
            Self::NewTransition => "New Transition",
            Self::NewState => "New State",
            Self::TransitionEdit(_) => "Transition",
            Self::StateEdit(_) => "State",
            Self::Settings => "Settings",
            Self::Confirmation => "Confirmation",
            Self::Help => "Help",
            Self::Save => "Save",
            Self::Convert => "Convert",
            Self::Error(_) => "Error",
        }
        .to_string()
    }

    pub fn icon(&self) -> ImageSource<'a> {
        match self {
            Self::NewState => include_image!("../../assets/icon/add.svg"),
            Self::NewTransition => include_image!("../../assets/icon/add.svg"),
            Self::StateEdit(_) => include_image!("../../assets/icon/edit.svg"),
            Self::TransitionEdit(_) => include_image!("../../assets/icon/edit.svg"),
            Self::Settings => include_image!("../../assets/icon/setting.svg"),
            Self::Help => include_image!("../../assets/icon/help.svg"),
            Self::Confirmation => include_image!("../../assets/icon/warning.svg"),
            Self::Save => include_image!("../../assets/icon/save.svg"),
            Self::Convert => include_image!("../../assets/icon/convert.svg"),
            Self::Error(_) => include_image!("../../assets/icon/warning.svg"),
        }
    }

    pub fn show(&self, app: &mut App, ui: &mut Ui) {
        let _ = match self {
            Self::NewState => state_edit::show(ui, app),
            Self::NewTransition => transition_edit::show(ui, app),
            Self::StateEdit(_) => state_edit::show(ui, app),
            Self::TransitionEdit(_) => transition_edit::show(ui, app),
            Self::Settings => settings::show(ui, app),
            Self::Help => todo!(),
            Self::Confirmation => todo!(),
            Self::Save => {
                Save::save(ui, app);
                Ok(())
            }
            Self::Convert => {
                Convert::show(ui, app);
                Ok(())
            }
            Self::Error(_) => Ok(()),
        };
    }

    pub fn close(&self, app: &mut App, ui: &mut Ui) {
        match self {
            _ => {}
        }
        app.ui.popup.close();
        ui.close();
    }

    pub fn min_size(&self, ui: &Ui) -> Vec2 {
        match self {
            _ => Vec2::ZERO,
        }
    }
}

#[derive(Debug, Default)]
pub struct RitmPopup {
    current_popups: Vec<RitmPopupEnum>,
    just_opened: bool,
}

impl RitmPopup {
    pub fn current(&self) -> Option<&RitmPopupEnum> {
        self.current_popups.last()
    }

    pub fn open(&mut self, popup: RitmPopupEnum) {
        self.current_popups.push(popup);
        self.just_opened = true;
    }

    pub fn close(&mut self) {
        self.current_popups.pop();
    }

    /// Consume the value
    pub fn just_opened(&mut self) -> bool {
        replace(&mut self.just_opened, false)
    }
}

pub fn show(ui: &Ui, app: &mut App) -> Result<(), RitmError> {
    for i in 0..app.ui.popup.current_popups.len() {
        modal(ui, app, i)?
    }
    Ok(())
}

fn modal(ui: &Ui, app: &mut App, i: usize) -> Result<(), RitmError> {
    let popup = app.ui.popup.current_popups[i].clone();

    Modal::new(Id::new(popup.title()))
        .frame(Frame {
            fill: LIGHT_THEME.surface,
            corner_radius: CornerRadius::same(10),
            inner_margin: Margin {
                bottom: 20,
                left: 10,
                right: 10,
                top: 10,
            },
            stroke: Stroke::new(1.0, LIGHT_THEME.border),
            ..Default::default()
        })
        .show(ui, |ui| {
            let min_header_width = 70.0
                + Font::get_width_word(ui, &FontId::proportional(Font::BIG), &popup.title()) * 1.1;

            if popup.min_size(ui).x > min_header_width {
                ui.allocate_ui_with_layout(
                    popup.min_size(ui).max(vec2(min_header_width, 0.0)),
                    Layout::top_down(Align::Min),
                    |ui| {
                        let close_id = Id::new("close");
                        let res = AtomLayout::new((
                            Image::new(popup.icon())
                                .tint(LIGHT_THEME.text_primary)
                                .fit_to_exact_size(Vec2::splat(30.0)),
                            RichText::new(popup.title())
                                .color(LIGHT_THEME.text_primary)
                                .text_style(Font::DEFAULT_FONT)
                                .size(Font::BIG),
                            Atom::grow(),
                            Atom::custom(close_id, Vec2::splat(30.0)),
                        ))
                        .gap(5.0)
                        .wrap_mode(TextWrapMode::Extend)
                        .min_size(vec2(ui.available_width(), 30.0))
                        .max_width(ui.available_width())
                        .allocate(ui)
                        .paint(ui);

                        ui.add(Separator::default().horizontal().spacing(10.0));

                        popup.show(app, ui);

                        if let Some(rect) = res.rect(close_id) {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
                                Color32::TRANSPARENT;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill =
                                Color32::TRANSPARENT;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                                Color32::TRANSPARENT;
                            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
                            ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
                            ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                            if ui
                                .put(
                                    rect,
                                    Button::image(
                                        Image::new(include_image!("../../assets/icon/close.svg"))
                                            .sense(Sense::click())
                                            .tint(LIGHT_THEME.text_primary)
                                            .fit_to_exact_size(Vec2::splat(30.0)),
                                    ),
                                )
                                .clicked()
                            {
                                app.ui.popup.close();
                                ui.close();
                            }
                        }
                    },
                );
            } else {
                ui.allocate_ui_with_layout(
                    popup.min_size(ui).max(vec2(min_header_width, 0.0)),
                    Layout::bottom_up(Align::Min),
                    |ui| {
                        let close_id = Id::new("close");
                        let layout = AtomLayout::new((
                            Image::new(popup.icon())
                                .tint(LIGHT_THEME.text_primary)
                                .fit_to_exact_size(Vec2::splat(30.0)),
                            RichText::new(popup.title())
                                .color(LIGHT_THEME.text_primary)
                                .text_style(Font::DEFAULT_FONT)
                                .size(Font::BIG),
                            Atom::grow(),
                            Atom::custom(close_id, Vec2::splat(30.0)),
                        ))
                        .gap(5.0)
                        .wrap_mode(TextWrapMode::Extend);

                        popup.show(app, ui);

                        ui.add(Separator::default().horizontal().spacing(10.0));

                        let res = layout
                            .min_size(vec2(ui.available_width(), 30.0))
                            .max_width(ui.available_width())
                            .allocate(ui)
                            .paint(ui);

                        if let Some(rect) = res.rect(close_id) {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
                                Color32::TRANSPARENT;
                            ui.style_mut().visuals.widgets.active.weak_bg_fill =
                                Color32::TRANSPARENT;
                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                                Color32::TRANSPARENT;
                            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
                            ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
                            ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                            if ui
                                .put(
                                    rect,
                                    Button::image(
                                        Image::new(include_image!("../../assets/icon/close.svg"))
                                            .sense(Sense::click())
                                            .tint(LIGHT_THEME.text_primary)
                                            .fit_to_exact_size(Vec2::splat(30.0)),
                                    ),
                                )
                                .clicked()
                            {
                                app.ui.popup.close();
                                ui.close();
                            }
                        }
                    },
                );
            }
        });

    // if let Some(ritmpopup) = app.popup.current_popup.clone() {
    //     match ritmpopup {
    //         RitmPopupEnum::TransitionEdit(transition_id) => {
    //             let source = app.turing.get_state(transition_id.source_id)?.get_name();
    //             let target = app.turing.get_state(transition_id.target_id)?.get_name();
    //             let max_size = vec2(
    //                 200.0
    //                     + app.turing.tm.graph_ref().get_k() as f32 * 125.0
    //                     + if app.turing.tm.graph_ref().get_k() == 0 {
    //                         25.0
    //                     } else {
    //                         0.0
    //                     },
    //                 ui.available_rect_before_wrap().height(),
    //             );
    //             modal_hold(
    //                 ui,
    //                 app,
    //                 format!("{} -> {}", source, target),
    //                 false,
    //                 |ui, app| {
    //                     ui.set_max_size(max_size);
    //                     // ui.set_min_size(vec2(max_size.x, 0.0));
    //                     transition_edit::show(ui, app)
    //                 },
    //             )?
    //         }
    //         RitmPopupEnum::StateEdit(state_id) => {
    //             let title = if let Some(state_id) = state_id {
    //                 app.turing.get_state(state_id)?.get_name().to_string()
    //             } else {
    //                 "New State".to_string()
    //             };
    //             let max_size = vec2(300.0, ui.available_rect_before_wrap().height());
    //             modal_hold(ui, app, title, false, |ui, app| {
    //                 ui.set_max_size(max_size);
    //                 ui.set_min_size(vec2(max_size.x, 0.0));
    //                 state_edit::show(ui, app)
    //             })?
    //         }
    //         RitmPopupEnum::Settings => {
    //             modal_hold(ui, app, t!("settings").to_string(), true, |ui, app| {
    //                 ui.set_max_size(ui.content_rect().size() * 0.8);
    //                 ui.set_min_size(ui.content_rect().size() * 0.8);
    //                 settings::show(ui, app)
    //             })?
    //         }
    //     }
    // }

    Ok(())
}
