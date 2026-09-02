use egui::{
    Align, Layout, Ui, include_image, vec2
};

use crate::{
    App,
    ui::{
        component::menu_item::MenuItem, popup::RitmPopupEnum
    },
};

// static EXAMPLES: Dir = include_directory!("ritm_core/resources");

// The menu provide access to the features and settings of the application.
#[derive(Default)]
pub struct Menu {
}

impl Menu {
    pub fn desktop_show(app: &mut App, ui: &mut Ui) {
        ui.allocate_ui_with_layout(vec2(ui.available_width(), 30.0), Layout::left_to_right(Align::Min), |ui| {
            ui.spacing_mut().item_spacing = vec2(15.0, 0.0);
            if ui.add(MenuItem::new(include_image!("../../assets/icon/account.svg")).with_text("Account")).clicked() {
                app.ui.popup.open(RitmPopupEnum::Account);
            }
            if ui.add(MenuItem::new(include_image!("../../assets/icon/setting.svg")).with_text("Settings")).clicked() {
                app.ui.popup.open(RitmPopupEnum::Settings);
            }
            if ui.add(MenuItem::new(include_image!("../../assets/icon/save.svg")).with_text("Save")).clicked() {
                app.ui.popup.open(RitmPopupEnum::Save);
            }
            if ui.add(MenuItem::new(include_image!("../../assets/icon/machine_folder.svg")).with_text("Machines")).clicked() {
                app.ui.popup.open(RitmPopupEnum::Tspace);
            }
            if ui.add(MenuItem::new(include_image!("../../assets/icon/help.svg")).with_text("Help")).clicked() {
                // open tutorial
            }
            if ui.add(MenuItem::new(include_image!("../../assets/icon/graph.svg")).with_text("Convert")).clicked() {
                app.ui.popup.open(RitmPopupEnum::Convert);
            }

        });
    }

    pub fn mobile_show(app: &mut App, ui: &mut Ui) {
        ui.allocate_ui_with_layout(vec2(ui.available_width(), 30.0), Layout::left_to_right(Align::Center), |ui| {
            ui.spacing_mut().item_spacing = vec2(15.0, 0.0);
            if ui.add(MenuItem::new(include_image!("../../assets/icon/account.svg"))).clicked() {
                app.ui.popup.open(RitmPopupEnum::Account);
            }
            if ui.add(MenuItem::new(include_image!("../../assets/icon/setting.svg"))).clicked() {
                app.ui.popup.open(RitmPopupEnum::Settings);
            }
            if ui.add(MenuItem::new(include_image!("../../assets/icon/machine_folder.svg"))).clicked() {
                app.ui.popup.open(RitmPopupEnum::Tspace);
            }
            if ui.add(MenuItem::new(include_image!("../../assets/icon/help.svg"))).clicked() {
                // open tutorial
            }

        });
    }

    // fn machine_folder(app: &mut App, ui: &mut Ui) -> Result<(), RitmError> {
    //     let res = ui.add(
    //         Button::image(
    //             Image::new(include_image!("../../assets/icon/machine_folder.svg"))
    //                 .fit_to_exact_size(Vec2::splat(Constant::ICON_SIZE))
    //                 .tint(LIGHT_THEME.icon),
    //         )
    //         .frame(false),
    //     );

    //     Popup::menu(&res)
    //         .gap(if app.ui.code.is_closed() { 10.0 } else { 5.0 })
    //         .align(if app.ui.code.is_closed() {
    //             RectAlign::RIGHT_START
    //         } else {
    //             RectAlign::BOTTOM_START
    //         })
    //         .close_behavior(PopupCloseBehavior::CloseOnClick)
    //         .show(|ui| {
    //             for example in EXAMPLES.files() {
    //                 let filename = example
    //                     .path()
    //                     .file_stem()
    //                     .expect("should exist")
    //                     .to_str()
    //                     .expect("should translate");
    //                 let code = example.contents_utf8().expect("should exist").to_string();
    //                 let button = Button::new(
    //                     RichText::new(filename)
    //                         .font(Font::default_small())
    //                         .color(app.theme.text_primary),
    //                 )
    //                 .frame(false)
    //                 .min_size(vec2(0.0, 25.0));
    //                 if ui.add(button).clicked() {
    //                     app.ui.code.new_tab(filename.to_string(), code);
    //                     app.code_to_graph()?; // TODO: add a setting to toggle this
    //                 }
    //             }

    //             ui.visuals_mut().widgets.noninteractive.bg_stroke =
    //                 Stroke::new(1.0, app.theme.border);
    //             ui.add(Separator::default().grow(6.0));

    //             let img = Image::new(include_image!("../../assets/icon/upload.svg"))
    //                 .fit_to_exact_size(Vec2::splat(25.0))
    //                 .tint(app.theme.surface)
    //                 .atom_size(Vec2::splat(25.0));

    //             if ui
    //                 .add(
    //                     Button::new((
    //                         RichText::new("Upload")
    //                             .font(Font::default_small())
    //                             .color(app.theme.text_primary),
    //                         img,
    //                     ))
    //                     .frame(false),
    //                 )
    //                 .clicked()
    //             {
    //                 // app.ui.menu.file.open();
    //             }

    //             Ok::<(), RitmError>(())
    //         });

        // if let Some(file) = app.ui.menu.file.get() {
        //     app.ui.transient.temp_code = Some(
        //         std::str::from_utf8(&file)
        //             .map_err(|e| {
        //                 RitmError::GuiError(GuiError::FileError {
        //                     error: e.to_string(),
        //                 })
        //             })?
        //             .to_string(),
        //     );
        // }

        // if let Some(code) = &app.ui.transient.temp_code {
        //     let code = code.clone();
        //     if let Some(answer) = boolean_popup(ui, app, "Do you want to create a new tab ?")? {
        //         if answer {
        //             app.ui.code.new_tab(app.ui.code.default_tab_name(), code);
        //         } else {
        //             *app.ui.code.current_code_mut()? = code;
        //         }
        //         app.ui.transient.temp_code = None;
        //     }
        // }
        // Ok(())
    // }
}
