use egui::{Image, RichText, Ui, Vec2, include_image};
use ritm_core::turing_parser::{graph_to_string, parse_turing_graph_string};

use crate::{App, error::RitmError, turing::Turing, ui::component::button::RitmButton};

/// Convert the graph to code and opposite.
pub struct Convert;

impl Convert {
    pub fn show(ui: &mut Ui, app: &mut App) {
        ui.vertical_centered(|ui| {
            ui.spacing_mut().item_spacing.y = 10.0;
            if ui
                .add(RitmButton::new((
                    Image::new(include_image!("../../../assets/icon/graph.svg"))
                        .fit_to_exact_size(Vec2::splat(30.0)),
                    RichText::new("Graph"),
                    Image::new(include_image!("../../../assets/icon/right_arrow.svg"))
                        .fit_to_exact_size(Vec2::splat(30.0)),
                    RichText::new("Code"),
                    Image::new(include_image!("../../../assets/icon/code.svg"))
                        .fit_to_exact_size(Vec2::splat(30.0)),
                )))
                .clicked()
            {
                Self::graph_to_code(app);
                app.ui.popup.close();
            }

            if ui
                .add(RitmButton::new((
                    Image::new(include_image!("../../../assets/icon/code.svg"))
                        .fit_to_exact_size(Vec2::splat(30.0)),
                    RichText::new("Code"),
                    Image::new(include_image!("../../../assets/icon/right_arrow.svg"))
                        .fit_to_exact_size(Vec2::splat(30.0)),
                    RichText::new("Graph"),
                    Image::new(include_image!("../../../assets/icon/graph.svg"))
                        .fit_to_exact_size(Vec2::splat(30.0)),
                )))
                .clicked()
            {
                let _ = Self::code_to_graph(app);
                app.ui.popup.close();
            }
        });
    }

    /// Convert the graph to code
    fn graph_to_code(app: &mut App) {
        let code = graph_to_string(app.turing.tm.graph_ref());
        app.ui.code.new_tab(app.ui.code.default_tab_name(), code);
    }

    /// Convert the current tab code into a graph
    /// TODO: handle in case the code is invalid
    fn code_to_graph(app: &mut App) -> Result<(), RitmError> {
        match parse_turing_graph_string(app.ui.code.current_code()?) {
            Ok(graph) => {
                app.turing = Turing::new_graph(graph)?;
                app.turing.layer_graph();
                app.ui.code.set_curr_parsing_error(None);
            }
            Err(e) => {
                app.ui.code.set_curr_parsing_error(Some(e));
            }
        }
        app.ui.graph.recenter();
        Ok(())
    }
}
