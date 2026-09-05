use std::format;

use egui::{
    Align, Align2, Atom, AtomExt, Atoms, Button, Color32, FontId, Frame, Id, Image, IntoAtoms,
    Label, Layout, Margin, RichText, ScrollArea, Stroke, TextEdit, TextFormat, Ui, Vec2,
    include_image, scroll_area::ScrollBarVisibility, text::LayoutJob, vec2,
};
use ritm_core::turing_parser::TuringParserError;

use crate::{
    App,
    error::{GuiError, RitmError},
    ui::theme::LIGHT_THEME,
    utils::{file::FileDialog, font::Font},
};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Code {
    current_tab: usize,
    tabs: Vec<Tab>,

    // Is the code section closed ?
    code_closed: bool,

    // Do we need to scroll to the end ?
    auto_scroll: bool,

    // Is the tab name currently being edited ?
    #[serde(skip)]
    editing_name: bool,

    // The current parsing error
    #[serde(skip)]
    curr_parsing_error: Option<TuringParserError>,

    // Used to save file
    #[serde(skip)]
    pub file: FileDialog,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Tab {
    name: String,
    code: String,
    marked_as_delete: bool,
}

impl Tab {
    fn mark_to_delete(&mut self) {
        self.marked_as_delete = true;
    }
}

impl Default for Code {
    fn default() -> Self {
        Self {
            tabs: vec![Tab {
                code: "accepting = q_a;

// Initialisation
q_i {ç, ç -> R, ç, R} q_copy;

// Copy to write ribbon
q_copy {0, _ -> R, 0, R} q_copy;
q_copy {1, _ -> R, 1, R} q_copy;
q_copy {$, _ -> L, _, N} q_return;

// Reset reading ribbon position
q_return {0, _ -> L, _, N} q_return;
q_return {1, _ -> L, _, N} q_return;
q_return {ç, _ -> R, _, L} q_check;

// Compare each side until end
q_check {0, 0 -> R, 0, L} q_check;
q_check {1, 1 -> R, 1, L} q_check;
q_check {$, ç -> N, ç, N} q_a;"
                    .to_string(),
                name: "binary_palindrome".to_string(),
                marked_as_delete: false,
            }],
            code_closed: Default::default(),
            current_tab: 0,
            editing_name: false,
            auto_scroll: false,
            curr_parsing_error: None,
            file: FileDialog::default(),
        }
    }
}

impl Code {
    pub fn current_code(&self) -> Result<String, RitmError> {
        Ok(self
            .tabs
            .get(self.current_tab)
            .ok_or(RitmError::GuiError(GuiError::InvalidApplicationState))?
            .code
            .clone())
    }

    pub fn current_code_mut(&mut self) -> Result<&mut String, RitmError> {
        Ok(&mut self
            .tabs
            .get_mut(self.current_tab)
            .ok_or(RitmError::GuiError(GuiError::InvalidApplicationState))?
            .code)
    }

    pub fn tab_name_check(&mut self) {
        // If empty then default name
        if self.tabs[self.current_tab].name.is_empty() {
            self.tabs[self.current_tab].name = self.default_tab_name();
            return;
        }

        let mut flag = 1;
        while flag > 0 && self.tabs.len() > 1 {
            flag -= 1;
            if self
                .tabs
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != self.current_tab)
                .any(|(_, t)| t.name == self.tabs[self.current_tab].name)
            {
                self.tabs[self.current_tab].name.push('2');
                flag += 1;
            };
        }
    }

    pub fn new_tab(&mut self, tab_name: String, code: String) {
        self.tabs.push(Tab {
            code,
            name: tab_name,
            marked_as_delete: false,
        });

        self.auto_scroll = true;
        self.switch_to(self.tabs.len() - 1);
        self.tab_name_check();
    }

    pub fn add_default_tab(&mut self) {
        self.new_tab(self.default_tab_name(), "".to_string());
    }

    pub fn is_closed(&self) -> bool {
        self.code_closed
    }

    pub fn close(&mut self) {
        self.code_closed = true;
    }

    pub fn open(&mut self) {
        self.code_closed = false;
    }

    pub(crate) fn default_tab_name(&self) -> String {
        format!("tab{}", self.tabs.len() + 1)
    }

    pub(crate) fn switch_to(&mut self, id: usize) {
        if self.tabs.len() <= id {
            return;
        }

        self.editing_name = false;
        self.current_tab = id;
        self.curr_parsing_error = None;
    }

    pub(crate) fn set_curr_parsing_error(&mut self, error: Option<TuringParserError>) {
        self.curr_parsing_error = error;
    }

    pub(crate) fn save_current_tab(&self) -> Result<(), RitmError> {
        if !self.current_code()?.is_empty() {
            FileDialog::default().save(
                &format!("{}.tm", self.tabs[self.current_tab].name),
                self.current_code()?.as_bytes().to_vec(),
            );
        }
        Ok(())
    }
}

pub fn show(app: &mut App, ui: &mut Ui) {
    if app.ui.code.code_closed {
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::from_white_alpha(25);
            ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_white_alpha(50);
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
            ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
            ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;

            // Close Code
            ui.allocate_ui_with_layout(
                vec2(40.0, 40.0),
                Layout::left_to_right(Align::Min)
                    .with_cross_align(Align::Center)
                    .with_cross_justify(true),
                |ui: &mut Ui| {
                    Frame::new().inner_margin(5).show(ui, |ui| {
                        if ui
                            .add(
                                Button::image(
                                    Image::new(include_image!("../../assets/icon/panel_open.svg"))
                                        .fit_to_exact_size(Vec2::splat(Font::ICON)),
                                )
                                .min_size(Vec2::splat(30.0)),
                            )
                            .clicked()
                        {
                            app.ui.code.open();
                        }
                    })
                },
            );
        });
    } else {
        ui.set_width(ui.content_rect().width() / 3.0);
        codebar(app, ui);
        code(app, ui);
    }
}

fn codebar(app: &mut App, ui: &mut Ui) {
    Frame::new().show(ui, |ui| {
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 36.0),
            Layout::right_to_left(Align::Min),
            |ui| {
                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
                ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::from_white_alpha(25);
                ui.style_mut().visuals.widgets.active.weak_bg_fill = Color32::from_white_alpha(50);
                ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::NONE;
                ui.style_mut().visuals.widgets.hovered.bg_stroke = Stroke::NONE;
                ui.style_mut().visuals.widgets.active.bg_stroke = Stroke::NONE;
                ui.spacing_mut().item_spacing.x = 0.0;

                actions(ui, app);
                tabs(ui, app);
            },
        );
    });
}

fn actions(ui: &mut Ui, app: &mut App) {
    Frame::new()
        .fill(Color32::from_white_alpha(2))
        .inner_margin(Margin::symmetric(5, 3))
        .show(ui, |ui| {
            ui.with_layout(
                Layout::right_to_left(Align::Min)
                    .with_cross_justify(true)
                    .with_cross_align(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 5.0;
                    // Close Code
                    if ui
                        .add(
                            Button::image(
                                Image::new(include_image!("../../assets/icon/panel_close.svg"))
                                    .fit_to_exact_size(Vec2::splat(Font::ICON)),
                            )
                            .min_size(Vec2::splat(30.0)),
                        )
                        .clicked()
                    {
                        app.ui.code.close();
                    }

                    // Add a new tab
                    if ui
                        .add(
                            Button::image(
                                Image::new(include_image!("../../assets/icon/plus.svg"))
                                    .fit_to_exact_size(Vec2::splat(Font::ICON)),
                            )
                            .min_size(Vec2::splat(30.0)),
                        )
                        .clicked()
                    {
                        app.ui.code.add_default_tab();
                    }
                },
            );
        });
}

fn tabs(ui: &mut Ui, app: &mut App) {
    // List of tabs
    Frame::new()
        .fill(Color32::from_gray(128).blend(LIGHT_THEME.code_background.gamma_multiply_u8(230)))
        .show(ui, |ui| {
            ui.spacing_mut().scroll.bar_width = ui.spacing().scroll.floating_width;
            ui.spacing_mut().scroll.active_background_opacity = 0.0;
            ui.spacing_mut().scroll.dormant_background_opacity = 0.0;
            ScrollArea::horizontal()
                .id_salt("tabs")
                .max_height(ui.available_height())
                .show(ui, |ui| {
                    ui.with_layout(
                        Layout::left_to_right(Align::Min).with_cross_justify(true),
                        |ui| {
                            ui.spacing_mut().item_spacing = vec2(4.0, 0.0);

                            // Iterate over the tabs
                            for i in 0..app.ui.code.tabs.len() {
                                tab(app, ui, i);
                            }

                            // Remove the tabs closed
                            app.ui.code.tabs.retain(|tab| !tab.marked_as_delete);

                            // If we create a remove method on the Code struct
                            // we can detect this outside the UI
                            if app.ui.code.tabs.is_empty() {
                                app.ui
                                    .code
                                    .new_tab(app.ui.code.default_tab_name(), "".to_string());
                            }

                            // Same for this
                            if app.ui.code.current_tab > app.ui.code.tabs.len() - 1 {
                                app.ui.code.switch_to(app.ui.code.tabs.len() - 1);
                            }

                            Ok::<(), RitmError>(())
                        },
                    );
                });
        });
}

/// A tab
fn tab(app: &mut App, ui: &mut Ui, index: usize) {
    let is_current_tab = app.ui.code.current_tab == index;
    // Tab frame
    Frame::new()
        .fill(if !is_current_tab {
            Color32::from_gray(128).blend(LIGHT_THEME.code_background.gamma_multiply_u8(200))
        } else {
            LIGHT_THEME.code_background
        })
        .inner_margin(Margin::symmetric(5, 2))
        .show(ui, |ui| {

            ui.horizontal_centered(|ui| {

            // Define the id for the sub buttons
            let text_edit_id = Id::new("text_edit");
            let delete_button_id = Id::new("delete_button");

            // Get the tab data
            let tab = &mut app.ui.code.tabs[index];

            // Layout the button
            ui.spacing_mut().icon_spacing = 4.0;
            ui.spacing_mut().button_padding = vec2(0.0, 0.0);
            let button = Button::new(if is_current_tab && app.ui.code.editing_name {
                Atoms::new(
                    // Atom::custom(Id::NULL, vec2(0.0, 30.0)),
                    Atom::custom(
                        text_edit_id,
                        vec2(
                            Font::get_width_word(
                                ui,
                                &FontId::proportional(Font::MEDIUM),
                                &app.ui.code.tabs[app.ui.code.current_tab].name,
                            ) + 10.0,
                            30.0,
                        ),
                    ),
                )
            } else {
                (
                    Atom::custom(Id::NULL, vec2(0.0, 30.0)),
                    RichText::new(tab.name.clone())
                        .color(LIGHT_THEME.code)
                        .font(FontId::proportional(Font::MEDIUM)),
                    Atom::custom(delete_button_id, Vec2::splat(Font::ICON))
                        .atom_align(Align2::RIGHT_CENTER),
                )
                    .into_atoms()
            })
            .min_size(vec2(0.0, ui.available_height()))
            .frame(false)
            .fill(LIGHT_THEME.code_background)
            .atom_ui(ui);

            // Textedit
            // TODO: change text_edit color
            if let Some(rect) = button.rect(text_edit_id) {
                let text_edit =
                    TextEdit::singleline(&mut app.ui.code.tabs[app.ui.code.current_tab].name)
                        // .margin(Margin::symmetric(
                        //     2,
                        //     -((Font::get_heigth(ui, &FontId::proportional(Font::SMALL)) - Font::BIG_SIZE) / 2.0)
                        //         as i8,
                        // ))
                        .vertical_align(Align::Center)
                        .background_color(Color32::PLACEHOLDER)
                        .font(Font::default(20.0))
                        .frame(Frame::NONE)
                        .text_color(LIGHT_THEME.code);

                let response = ui.put(rect, text_edit);

                if response.lost_focus() {
                    app.ui.code.tab_name_check();
                    app.ui.code.editing_name = false;
                }

                if app.ui.code.editing_name {
                    response.request_focus();
                }

                // TODO: maybe reenable this ?
                // no (i mean put a setting at least)
                // response.request_focus();
            }

            if !is_current_tab && button.response.clicked() {
                app.ui.code.switch_to(index);
            }

            // A bunch of visual, can't wait for a style update
            ui.visuals_mut().widgets.hovered.weak_bg_fill = if !is_current_tab {
                Color32::from_gray(128).blend(LIGHT_THEME.code_background.gamma_multiply_u8(180))
            } else {
                Color32::from_gray(128).blend(LIGHT_THEME.code_background.gamma_multiply_u8(210))
            };
            ui.visuals_mut().widgets.inactive.weak_bg_fill = if !is_current_tab {
                Color32::from_gray(128).blend(LIGHT_THEME.code_background.gamma_multiply_u8(210))
            } else {
                LIGHT_THEME.code_background
            };
            ui.visuals_mut().widgets.inactive.bg_stroke = Stroke::NONE;
            ui.visuals_mut().widgets.hovered.bg_stroke = Stroke::NONE;
            ui.visuals_mut().widgets.active.bg_stroke = Stroke::NONE;
            ui.spacing_mut().button_padding.x = 0.0;

            // Show the close button only if hovered or if it's the current tab
            if !(app.ui.code.editing_name)
                && (ui.rect_contains_pointer(button.response.rect) || is_current_tab)
                && let Some(rect) = button.rect(delete_button_id)
                && ui
                    .put(
                        rect,
                        Button::image(
                            Image::new(include_image!("../../assets/icon/close_small.svg"))
                                .shrink_to_fit()
                                .tint(LIGHT_THEME.code),
                        ),
                    )
                    .clicked()
            {
                app.ui.code.tabs[index].mark_to_delete();
            }

            if button.response.double_clicked() {
                app.ui.code.editing_name = true;
            }

            });
            Ok::<(), RitmError>(())
        });
}

/// If there is no tabe, propose the creation of a new one or loading
/// an existing machine code
/// TODO i'm lazy
#[allow(unused)]
pub fn no_code(app: &mut App, ui: &mut Ui) {
    todo!()
}

/// Display the code section of the application
pub fn code(app: &mut App, ui: &mut Ui) {
    ScrollArea::vertical()
        .id_salt("code")
        .scroll_bar_visibility(ScrollBarVisibility::AlwaysHidden)
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                ui.available_size(),
                Layout::left_to_right(egui::Align::Min),
                |ui| {
                    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

                    let code_width = ui.available_width()
                        - 30.0
                        - Font::get_width(ui, &FontId::proportional(Font::SMALL)) * 3.0;

                    let job = LayoutJob::simple(
                        app.ui.code.tabs[app.ui.code.current_tab].code.clone(),
                        FontId::proportional(Font::SMALL),
                        // FontId::proportional(Font::SMALL),
                        Color32::PLACEHOLDER,
                        code_width,
                    );

                    let galley = ui.painter().layout_job(job);

                    let mut number: String = "".to_string();

                    for i in 1..=galley.rows.len() {
                        number.push_str(
                            &(" ".repeat(
                                ((galley.rows.len() as f32).log10() as usize
                                    - (i as f32).log10() as usize)
                                    .max(0),
                            ) + format!("{}\n", i).as_str()),
                        );
                    }
                    let mut line = 1;
                    let mut col = 1;

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
                                        FontId::proportional(Font::SMALL),
                                        LIGHT_THEME.syntax_comment,
                                    ),
                                );
                                code = &code[end..];
                            } else {
                                let mut it = code.char_indices();
                                it.next();
                                let end = it.next().map_or(code.len(), |(idx, _chr)| idx);
                                let mut format = TextFormat::simple(
                                    FontId::proportional(Font::SMALL),
                                    LIGHT_THEME.code,
                                );
                                if &code[..end] == "\n" {
                                    line += 1;
                                    col = 0;
                                } else {
                                    col += 1;
                                }
                                if let Some(err) = &app.ui.code.curr_parsing_error {
                                    match err {
                                        TuringParserError::FileError {
                                            given_path: _,
                                            error: _,
                                        } => (),
                                        TuringParserError::ParsingError {
                                            line_col_pos,
                                            value: _,
                                            missing_value: _,
                                        }
                                        | TuringParserError::TuringError {
                                            line_col_pos,
                                            turing_error: _,
                                            value: _,
                                        } => {
                                            if let Some(line_col) = line_col_pos
                                                && line_col.0 == line
                                            {
                                                format.underline = if line_col.1 == col {
                                                    Stroke::new(3.5, Color32::LIGHT_RED)
                                                } else {
                                                    Stroke::new(2.5, Color32::DARK_RED)
                                                }
                                            }
                                        }
                                    }
                                }
                                layout_job.append(&code[..end], 0.0, format);
                                code = &code[end..];
                            }
                        }
                        layout_job.wrap.max_width = wrap_width;
                        ui.fonts_mut(|f| f.layout_job(layout_job))
                    };

                    let salt = app.ui.code.tabs[app.ui.code.current_tab].name.clone();
                    let code =
                        TextEdit::multiline(&mut app.ui.code.tabs[app.ui.code.current_tab].code)
                            .code_editor()
                            .id_salt(salt)
                            .font(FontId::proportional(Font::SMALL))
                            .frame(Frame::NONE)
                            .margin(Margin::same(0))
                            .background_color(LIGHT_THEME.code_background)
                            .layouter(&mut layouter);

                    let line_number = Label::new(
                        RichText::new(number)
                            .color(LIGHT_THEME.text_secondary.gamma_multiply(0.5))
                            .font(FontId::proportional(Font::SMALL)),
                    )
                    .halign(egui::Align::Min)
                    .selectable(false)
                    .extend();

                    ui.add_space(10.0);
                    ui.add_sized(
                        vec2(
                            Font::get_width(ui, &FontId::proportional(Font::SMALL)) * 2.0,
                            Font::get_heigth(ui, &FontId::proportional(Font::SMALL))
                                * galley.rows.len() as f32,
                        ),
                        line_number,
                    );
                    ui.add_space(20.0);

                    let resp = ui.add_sized(ui.available_size() - vec2(5.0, 0.0), code);

                    // When the code changes, we can assume the error is irrelevant :
                    if resp.changed() {
                        app.ui.code.curr_parsing_error = None;
                    }

                    if resp.has_focus() {
                        app.transient.listen_to_keybind = false;
                    }
                    if let Some(error) = &app.ui.code.curr_parsing_error {
                        resp.on_hover_text_at_pointer(match error {
                            TuringParserError::FileError {
                                given_path: _,
                                error,
                            } => error.to_string(),
                            TuringParserError::ParsingError {
                                line_col_pos: _,
                                value: _,
                                missing_value,
                            } => format!(
                                "Parsing failed due to a missing value{}",
                                if let Some(val) = missing_value {
                                    format!(", try adding the following : \"{val}\".")
                                } else {
                                    ".".to_string()
                                }
                            ),
                            TuringParserError::TuringError {
                                line_col_pos: _,
                                turing_error,
                                value: _,
                            } => format!("Ran into an error while parsing a line : {turing_error}"),
                        });
                    }

                    ui.add_space(5.0);
                },
            );
        });
}