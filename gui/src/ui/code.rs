use egui::{
    Color32, Label, Layout, Margin, RichText, ScrollArea, TextEdit, Ui,
    scroll_area::ScrollBarVisibility, text::LayoutJob, vec2,
};

use crate::{App, ui::font::Font};

/// Display the code section of the application
pub fn show(app: &mut App, ui: &mut Ui) {
    ScrollArea::vertical()
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), ui.available_height()),
                Layout::left_to_right(egui::Align::Min),
                |ui| {
                    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

                    let code_width =
                        ui.available_width() - 35.0 - Font::get_width(ui, &Font::default()) * 3.0;
                    let job = LayoutJob::simple(
                        app.code.clone(),
                        Font::default(),
                        Color32::PLACEHOLDER,
                        code_width,
                    );
                    let galley = ui.painter().layout_job(job);

                    let mut number: String = "".to_string();

                    for i in 1..=galley.rows.len() {
                        number.push_str(format!("{}\n", i).as_str());
                    }

                    let code = TextEdit::multiline(&mut app.code)
                        .code_editor()
                        .font(Font::default())
                        .frame(false)
                        .margin(Margin::same(0))
                        .background_color(Color32::from_black_alpha(50));

                    let line_number =
                        Label::new(RichText::new(number).color(app.theme.gray.gamma_multiply(0.5)))
                            .halign(egui::Align::Max)
                            .selectable(false);

                    ui.add_space(10.0);
                    ui.add_sized(
                        vec2(
                            Font::get_width(ui, &Font::default()) * 3.0,
                            Font::get_heigth(ui, &Font::default()) * galley.rows.len() as f32,
                        ),
                        line_number,
                    );
                    ui.add_space(20.0);

                    ui.add_sized(ui.available_size() - vec2(5.0, 0.0), code);
                    ui.add_space(5.0);
                },
            );
        });
}
