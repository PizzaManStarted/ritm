use egui::{
    Color32, Label, Layout, Margin, RichText, ScrollArea, TextEdit, TextFormat, Ui,
    scroll_area::ScrollBarVisibility, text::LayoutJob, vec2,
};

use crate::{ui::font::Font, App};

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

                    let code_width = ui.available_width()
                        - 35.0
                        - Font::get_width(ui, &Font::default_medium()) * 3.0;
                    let job = LayoutJob::simple(
                        app.code.clone(),
                        Font::default_medium(),
                        Color32::PLACEHOLDER,
                        code_width,
                    );
                    let galley = ui.painter().layout_job(job);

                    let mut number: String = "".to_string();

                    for i in 1..=galley.rows.len() {
                        number.push_str(
                            &(" ".repeat((1 - (i as f32).log10() as usize).max(0))
                                + format!("{}\n", i).as_str()),
                        );
                    }

                    let mut layouter = |ui: &Ui, buf: &dyn egui::TextBuffer, wrap_width: f32| {
                        let mut layout_job = LayoutJob::default();
                        let mut code: &str = buf.as_str();
                        while !code.is_empty() {
                            if code.starts_with("//") {
                                let end = code.find("\n").unwrap_or(code.len());
                                layout_job.append(
                                    &code[..end],
                                    0.0,
                                    TextFormat::simple(
                                        Font::default_medium(),
                                        Color32::from_rgb(85, 160, 75),
                                    ),
                                );
                                code = &code[end..];
                            } else {
                                let mut it = code.char_indices();
                                it.next();
                                let end = it.next().map_or(code.len(), |(idx, _chr)| idx);
                                layout_job.append(
                                    &code[..end],
                                    0.0,
                                    TextFormat::simple(Font::default_medium(), app.theme.gray),
                                );
                                code = &code[end..];
                            }
                        }
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts(|f| f.layout_job(layout_job))
                    };

                    let code = TextEdit::multiline(&mut app.code)
                        .code_editor()
                        .font(Font::default_medium())
                        .frame(false)
                        .margin(Margin::same(0))
                        .background_color(Color32::from_black_alpha(50))
                        .layouter(&mut layouter);

                    let line_number = Label::new(
                        RichText::new(number)
                            .color(app.theme.gray.gamma_multiply(0.5))
                            .font(Font::default_medium()),
                    )
                    .halign(egui::Align::Max)
                    .selectable(false)
                    .extend();

                    ui.add_space(10.0);
                    ui.add_sized(
                        vec2(
                            Font::get_width(ui, &Font::default_medium()) * 2.0,
                            Font::get_heigth(ui, &Font::default_medium())
                                * galley.rows.len() as f32,
                        ),
                        line_number,
                    );
                    ui.add_space(20.0);

                    if ui
                        .add_sized(ui.available_size() - vec2(5.0, 0.0), code)
                        .has_focus()
                    {
                        app.event.listen_to_keybind = false;
                    }

                    ui.add_space(5.0);
                },
            );
        });
}
