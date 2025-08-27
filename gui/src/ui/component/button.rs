use egui::{AtomLayout, Button, Frame, IntoAtoms, Margin, Response, Sense, Ui, Widget};

pub struct RitmButton<'a> {
    layout: AtomLayout<'a>,
    disabled: bool,
    margin: Margin,
}

impl<'a> RitmButton<'a> {
    pub fn new(atoms: impl IntoAtoms<'a>) -> Self {
        Self {
            layout: AtomLayout::new(atoms.into_atoms()).sense(Sense::click()),
            disabled: false,
            margin: Margin::symmetric(4, 2),
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn atom_ui(self, ui: &mut Ui) -> Response {
        let Self {
            mut layout,
            disabled,
            margin,
        } = self;

        let response = ui.ctx().read_response(ui.next_auto_id());

        let visuals = response.map_or(&ui.style().visuals.widgets.inactive, |response| {
            ui.style().interact(&response)
        });

        let layout = layout.frame(
            Frame::new()
                .fill(visuals.bg_fill)
                .corner_radius(visuals.corner_radius)
                .stroke(visuals.bg_stroke)
                .inner_margin(margin),
        );

        layout.show(ui).response
    }
}

impl Widget for RitmButton<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        self.atom_ui(ui)
    }
}
