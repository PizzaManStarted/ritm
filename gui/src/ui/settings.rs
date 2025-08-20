use egui::{Image, ImageButton, Ui, Vec2, include_image, vec2};
use egui_flex::{Flex, FlexAlign, FlexAlignContent, FlexDirection, item};
use ritm_core::{
    turing_machine::{Mode, TuringMachines},
    turing_parser::parse_turing_graph_string,
};

use crate::{ui::{constant::Constant, popup::Popup}, App};

/// Global application control, like settings, compile or load file
pub fn show(app: &mut App, ui: &mut Ui) {
    let mut flex = Flex::new()
        .align_items(FlexAlign::Center)
        .align_content(FlexAlignContent::Start)
        .gap(vec2(10.0, 10.0));

    flex = if app.event.is_code_closed {
        flex.direction(FlexDirection::Vertical).h_full()
    } else {
        flex.direction(FlexDirection::Horizontal).w_full()
    };

    flex.show(ui, |ui| {

        app.event.is_small_window = ui.ui().ctx().screen_rect().width() < ((Constant::ICON_SIZE + 10.0) * 6.0) * 3.0;

        if app.event.is_small_window {
            app.event.is_code_closed = true;
        }

        if app.event.is_code_closed {
            if !app.event.is_small_window && ui
                .add(
                    item(),
                    ImageButton::new(
                        Image::new(include_image!("../../assets/icon/panel_open.svg"))
                            .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
                            .tint(app.theme.white),
                    )
                    .frame(false),
                )
                .clicked()
            {
                app.event.is_code_closed = false;
            }
        }

        if ui
            .add(
                item(),
                ImageButton::new(
                    Image::new(include_image!("../../assets/icon/setting.svg"))
                        .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
                        .tint(app.theme.white),
                )
                .frame(false),
            )
            .clicked()
        {
            app.popup = Popup::Setting;
        }

        if ui
            .add(
                item(),
                ImageButton::new(
                    Image::new(include_image!("../../assets/icon/save.svg"))
                        .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
                        .tint(app.theme.white),
                )
                .frame(false),
            )
            .clicked()
        {
            app.event.are_settings_visible = true;
        }

        if ui
            .add(
                item(),
                ImageButton::new(
                    Image::new(include_image!("../../assets/icon/upload.svg"))
                        .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
                        .tint(app.theme.white),
                )
                .frame(false),
            )
            .clicked()
        {
            app.file.open();
        }

        if let Some(file) = app.file.get() {
            app.code = std::str::from_utf8(&file).unwrap().to_string()
        }

        if ui
            .add(
                item(),
                ImageButton::new(
                    Image::new(include_image!("../../assets/icon/help.svg"))
                        .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
                        .tint(app.theme.white),
                )
                .frame(false),
            )
            .clicked()
        {
            app.event.are_settings_visible = true;
        }

        if !app.event.is_code_closed {
            if ui
                .add(
                    item(),
                    ImageButton::new(
                        Image::new(include_image!("../../assets/icon/graph.svg"))
                            .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
                            .tint(app.theme.white),
                    )
                    .frame(false),
                )
                .clicked()
            {
                match parse_turing_graph_string(app.code.to_string()) {
                    Ok(graph) => {
                        app.turing = TuringMachines::new(
                            graph,
                            app.input.to_string(),
                            Mode::StopFirstReject,
                        )
                        .unwrap();
                        app.turing_to_graph();
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }

            if ui
                .add(
                    item(),
                    ImageButton::new(
                        Image::new(include_image!("../../assets/icon/panel_close.svg"))
                            .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
                            .tint(app.theme.white),
                    )
                    .frame(false),
                )
                .clicked()
            {
                app.event.is_code_closed = true;
            }
        }
    });
}
