use egui::{
    Color32, Context, Shadow, Stroke, Visuals,
    style::{Selection, WidgetVisuals, Widgets},
};
use egui::{Ui, hex_color, style::TextCursorStyle};

pub const LIGHT_THEME: Theme = Theme {
    primary: hex_color!("#1b54f3"),
    secondary: hex_color!("#7eb2e6"),
    background: hex_color!("#cee5ff"),
    surface: hex_color!("#ffffff"),
    scene: hex_color!("#f3ebd9"),
    border: hex_color!("#717171"),
    text_primary: hex_color!("#000000"),
    text_secondary: hex_color!("#444444"),
    text_disabled: hex_color!("#999999"),
    icon: hex_color!("#1b54f3"),
    hover: hex_color!("#ff00ff"),
    active: hex_color!("#ff00ff"),
    focus: hex_color!("#ff00ff"),
    success: hex_color!("#55dd55"),
    warning: hex_color!("#ffff11"),
    error: hex_color!("#dd1111"),
    info: hex_color!("#ff00ff"),
    selection: hex_color!("#ff00ff"),
    code_background: hex_color!("#13203c"),
    code: hex_color!("#ffffff"),
    syntax_keyword: hex_color!("#ffffff"),
    syntax_string: hex_color!("#ffffff"),
    syntax_comment: hex_color!("#55a04b"),
    highlight: hex_color!("#ff00ff"),
    disabled: hex_color!("#cccccc"),
};

/// Theme of the application, holding different color for each part
#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Theme {
    pub primary: Color32,
    pub secondary: Color32,
    pub background: Color32,
    pub surface: Color32,
    pub scene: Color32,
    pub border: Color32,
    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_disabled: Color32,
    pub icon: Color32,
    pub hover: Color32,
    pub active: Color32,
    pub focus: Color32,
    pub success: Color32,
    pub warning: Color32,
    pub error: Color32,
    pub info: Color32,
    pub selection: Color32,
    pub code_background: Color32,
    pub code: Color32,
    pub syntax_keyword: Color32,
    pub syntax_string: Color32,
    pub syntax_comment: Color32,
    pub highlight: Color32,
    pub disabled: Color32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: hex_color!("#8d6346ff"),
            secondary: hex_color!("#fff0e0ff"),
            background: hex_color!("#68c251ff"),
            surface: hex_color!("#ffffffff"),
            scene: hex_color!("#f3ebd9"),
            border: hex_color!("#454545ff"),
            text_primary: hex_color!("#111111ff"),
            text_secondary: hex_color!("#a4a4a4ff"),
            text_disabled: hex_color!("#777777ff"),
            icon: hex_color!("#323232ff"),
            hover: hex_color!("#ff0000ff"),
            active: hex_color!("#acff00ff"),
            focus: hex_color!("#ff0000ff"),
            success: hex_color!("#19c832ff"),
            warning: hex_color!("#ffff00ff"),
            error: hex_color!("#ff1964ff"),
            info: hex_color!("#ff0000ff"),
            selection: hex_color!("#5adbffff"),
            code_background: hex_color!("#313e45ff"),
            code: hex_color!("#fcf3edff"),
            syntax_keyword: hex_color!("#000000ff"),
            syntax_string: hex_color!("#000000ff"),
            syntax_comment: hex_color!("#55a04bff"),
            highlight: hex_color!("#ff5e5eff"),
            disabled: hex_color!("#dddddd"),
        }
    }
}

impl Theme {
    pub fn retro() -> Self {
        Self {
            primary: hex_color!("#74bdcbff"),
            secondary: hex_color!("#fff0e0ff"),
            background: hex_color!("#ffa384ff"),
            surface: hex_color!("#fffafaff"),
            scene: hex_color!("#f3ebd9"),
            border: hex_color!("#7d7d7dff"),
            text_primary: hex_color!("#323232ff"),
            text_secondary: hex_color!("#a4a4a4ff"),
            text_disabled: hex_color!("#777777ff"),
            icon: hex_color!("#ffffffff"),
            hover: hex_color!("#ff0000ff"),
            active: hex_color!("#C73E1Dff"),
            focus: hex_color!("#ff5080ff"),
            success: hex_color!("#00a824ff"),
            warning: hex_color!("#ffff00ff"),
            error: hex_color!("#ff1932ff"),
            info: hex_color!("#7bbcffff"),
            selection: hex_color!("#68C3D4ff"),
            code_background: hex_color!("#313e45ff"),
            code: hex_color!("#fcf3edff"),
            syntax_keyword: hex_color!("#000000ff"),
            syntax_string: hex_color!("#000000ff"),
            syntax_comment: hex_color!("#55a04bff"),
            highlight: hex_color!("#ff5e5eff"),
            disabled: hex_color!("#dddddd"),
        }
    }

    pub fn monochrome() -> Self {
        Self {
            primary: hex_color!("#bfbfbfff"),
            secondary: hex_color!("#ffffffff"),
            background: hex_color!("#919191ff"),
            surface: hex_color!("#ffffffff"),
            scene: hex_color!("#f3ebd9"),
            border: hex_color!("#565656ff"),
            text_primary: hex_color!("#000000ff"),
            text_secondary: hex_color!("#000000ff"),
            text_disabled: hex_color!("#777777ff"),
            icon: hex_color!("#000000ff"),
            hover: hex_color!("#ff0000ff"),
            active: hex_color!("#c93700ff"),
            focus: hex_color!("#ff006bff"),
            success: hex_color!("#00be00ff"),
            warning: hex_color!("#ffff00ff"),
            error: hex_color!("#ff1932ff"),
            info: hex_color!("#7bbcffff"),
            selection: hex_color!("#44e7ffff"),
            code_background: hex_color!("#ffffffff"),
            code: hex_color!("#000000ff"),
            syntax_keyword: hex_color!("#000000ff"),
            syntax_string: hex_color!("#000000ff"),
            syntax_comment: hex_color!("#36bc00ff"),
            highlight: hex_color!("#ff00f4ff"),
            disabled: hex_color!("#dddddd"),
        }
    }

    pub fn save_new_theme(&mut self) {
        // TODO save here the custom theme in a file or with persistence
    }

    pub fn default_widget() -> WidgetVisuals {
        WidgetVisuals {
            bg_fill: LIGHT_THEME.surface,
            bg_stroke: Stroke::new(1.0, LIGHT_THEME.border),
            corner_radius: 2.into(),
            expansion: 0.0,
            fg_stroke: Stroke::new(1.0, LIGHT_THEME.border),
            weak_bg_fill: LIGHT_THEME.surface,
        }
    }

    /// Set the global theme used in egui widget
    pub fn as_global_theme(ctx: &Context) {
        let default_widget = Self::default_widget();

        let default_shadow = Shadow {
            offset: [2, 4],
            blur: 4,
            spread: 0,
            color: Color32::from_black_alpha(25),
        };

        ctx.set_visuals(Visuals {
            text_cursor: TextCursorStyle {
                stroke: Stroke::new(1.0, LIGHT_THEME.border),
                ..Default::default()
            },
            window_fill: LIGHT_THEME.background,
            window_corner_radius: 5.into(),
            window_stroke: Stroke::new(1.0, LIGHT_THEME.border),
            window_shadow: Shadow::NONE,
            popup_shadow: default_shadow,
            override_text_color: Some(LIGHT_THEME.text_primary),
            text_edit_bg_color: Some(LIGHT_THEME.surface),
            widgets: Widgets {
                active: WidgetVisuals { ..default_widget },
                hovered: WidgetVisuals { ..default_widget },
                inactive: WidgetVisuals { ..default_widget },
                noninteractive: WidgetVisuals { ..default_widget },
                open: WidgetVisuals { ..default_widget },
            },
            selection: Selection {
                bg_fill: Color32::from_black_alpha(50),
                stroke: Stroke::new(1.0, LIGHT_THEME.highlight),
            },
            ..Default::default()
        });
    }

    pub fn set_widget(ui: &mut Ui, widget: WidgetVisuals) {
        ui.visuals_mut().widgets.inactive = widget;
        ui.visuals_mut().widgets.active = widget;
        ui.visuals_mut().widgets.hovered = widget;
        ui.visuals_mut().widgets.noninteractive = widget;
        ui.visuals_mut().widgets.open = widget;
    }

    /// Compute the best contrast color between white and black for any RGB color
    pub fn constrast_color(color: Color32) -> Color32 {
        let luminance =
            (0.299 * color.r() as f32 + 0.587 * color.g() as f32 + 0.114 * color.b() as f32)
                / 255.0;

        if luminance > 0.5 {
            Color32::BLACK
        } else {
            Color32::WHITE
        }
    }
}