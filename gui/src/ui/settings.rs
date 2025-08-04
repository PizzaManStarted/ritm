use egui::{include_image, vec2, Image, ImageButton, Layout, Ui, Vec2};
use egui_flex::{item, Flex, FlexAlign, FlexAlignContent, FlexDirection};

use crate::{App, ui::constant::Constant};

/// Global application control, like settings, compile or load file
pub fn show(app: &mut App, ui: &mut Ui) {

    let mut flex = Flex::new()
        .align_items(FlexAlign::Center)
        .align_content(FlexAlignContent::Start)
        .gap(vec2(10.0, 10.0));

    flex = if app.event.is_code_closed {
        flex.direction(FlexDirection::Vertical)
            .h_full()
    } else {
        flex.direction(FlexDirection::Horizontal)
            .w_full()
    };

    flex.show(ui, |ui| {

        if app.event.is_code_closed {
            if ui.add(item(),
                ImageButton::new(
                    Image::new(include_image!("../../assets/icon/panel_open.svg"))
                        .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
                        .tint(app.theme.white),
                )
                .frame(false),
            ).clicked() {
                app.event.is_code_closed = false;
            }
        }

        if ui
                .add(item(),
                    ImageButton::new(
                        Image::new(include_image!("../../assets/icon/setting.svg"))
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
                .add(item(),
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
                .add(item(),
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
                .add(item(),
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
            ui.grow();

            if ui.add(item(),
                ImageButton::new(
                    Image::new(include_image!("../../assets/icon/panel_close.svg"))
                        .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
                        .tint(app.theme.white),
                )
                .frame(false),
            ).clicked() {
                app.event.is_code_closed = true;
            }
        }

    });


    // ui.allocate_ui_with_layout(
    //     vec2(ui.available_width(), 0.0),
    //     if !app.event.is_code_closed {Layout::left_to_right(egui::Align::Center)} else {Layout::top_down(egui::Align::Center)},
    //     |ui| {
    //         ui.spacing_mut().item_spacing = vec2(10.0, 10.0);

            
    //     },
    // );
}
