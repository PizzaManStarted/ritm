use std::{collections::VecDeque, time::Duration};

use egui::{Pos2, Ui};
use egui_extras::install_image_loaders;
use ritm_core::{
    turing_graph::TuringGraph,
    turing_parser::{graph_to_string, parse_turing_graph_string},
};

use crate::{
    error::RitmError,
    turing::{StateEdit, Turing},
    ui::{
        self, RitmUi, popup::settings::Settings, theme::Theme
    },
    utils::{
        constant::Constant,
        file::FileData,
        font::load_font,
    },
};

/// The only structure that is persistent each redraw of the application
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct App {
    /// The turing machine itself
    #[serde(skip)]
    pub turing: Turing,

    pub settings: Settings,

    /// The event/state of the application
    #[serde(skip)]
    pub transient: Transient,

    /// The ui data
    pub ui: RitmUi,

    /// Errors that happen during execution
    #[serde(skip)]
    pub errors: VecDeque<RitmError>
}

/// Keep the state of the application
///
/// Used to check what the user see and/or can do
pub struct Transient {
    /// Is the user moving as state around ?
    pub is_small_window: bool,

    pub listen_to_keybind: bool,

    pub temp_code: Option<String>,

    pub add_transition: bool,

    pub taking_screenshot: bool,

    pub temp_screenshot: Option<FileData>,
}

impl Default for Transient {
    fn default() -> Self {
        Self {
            is_small_window: false,
            listen_to_keybind: true,
            temp_code: None,
            add_transition: false,
            taking_screenshot: false,
            temp_screenshot: None,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_debug_on_hover(true);

        // Load the fonts used in the application
        load_font(cc);

        // Load the saved state from a previous session
        let app: App = if let Some(storage) = cc.storage {
            eframe::get_value(storage, "ritm").unwrap_or_default()
        } else {
            Default::default()
        };

        Theme::as_global_theme(&cc.egui_ctx);
        app
    }

    /// Reset the machine execution
    pub fn reset(&mut self) {
        self.turing.reset();
    }

    /// Reset the machine execution with the new input
    pub fn set_input(&mut self) -> Result<(), RitmError> {
        self.turing.set_word(self.ui.control.input())
    }

    /// Change the number of tapes in the graph
    pub fn update_k(&mut self, k: usize) -> Result<(), RitmError> {
        self.ui.graph.reset();
        let new_turing = Turing::new_graph(TuringGraph::new(k, true))?;
        self.turing = new_turing;
        self.turing.layer_graph();
        Ok(())
    }

    /// Prepare the edition of a state and open the popup
    pub fn edit_state(&mut self, state_id: usize) -> Result<(), RitmError> {
        self.turing.prepare_state_edit(state_id)?;
        self.ui.popup
            .open(ui::popup::RitmPopupEnum::StateEdit(state_id));
        Ok(())
    }

    /// Prepare the edition of a state and open the popup
    pub fn edit_transition(&mut self, source_id: usize, target_id: usize) -> Result<(), RitmError> {
        self.turing.prepare_transition_edit(source_id, target_id)?;
        self.ui.popup
            .open(ui::popup::RitmPopupEnum::TransitionEdit(
                (source_id, target_id).into(),
            ));
        Ok(())
    }

    pub fn new_state_at_pos(&mut self, pos: Pos2) {
        let mut state_edit = StateEdit::empty(self.turing.tm.graph_ref().get_next_id());

        state_edit.get_edit().state.position = pos;
        state_edit.get_edit().name = format!(
            "q_{}",
            self.turing.tm.graph_ref().get_state_hashmap().len() + 1
        );

        self.turing.state_edit = Some(state_edit);
        self.ui.popup.open(ui::popup::RitmPopupEnum::NewState);
    }
}

impl eframe::App for App {
    /// Save the state of the application
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "ritm", self);
    }

    /// Draw every frame the application
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        install_image_loaders(ui);

        // Update the scale to enable responsive UI when the window size change
        // Constant::update_scale(ui);
        // Check if the window is too small to display the menu icons
        self.transient.is_small_window =
            ui.content_rect().width() < ((Constant::ICON_SIZE + 10.0) * 6.0) * 3.0;

        // Draw the whole application
        let _ = ui::show(self, ui).err();

        // Stack the errors founds
        // if let Some(error) = err {
        //     self.error.push_back(error);
        // }

        // Display any error that arose
        // TODO use popup
        // error::show(ui, self);

        // Draw the tutorial
        // tutorial::show(ui, self);

        // End the tutorial
        // if let Some(tutorial) = self.tutorial.has_finished {
        //     self.tutorial.has_finished = None;
        //     self.tutorial
        //         .already_played
        //         .entry(tutorial)
        //         .and_modify(|b| *b = true);
        // }

        // Force the Ui to update if the machine is running
        if self.ui.control.is_running() && self.ui.control.update_time(ui.input(|r| r.time)) {
            self.turing.next_step();
            if self.turing.accepted.is_some() {
                self.ui.control.pause();
                ui.request_repaint(); // To update the ui one last time
            }
        }

        // While the machine is running we update the application 100 times per step
        if self.ui.control.is_running() {
            ui.request_repaint_after(Duration::from_millis(
                (self.ui.control.interval() * 10.0) as u64,
            ));
        }

        // keybind(ui, self);

        // if self.settings.enable_debug {
        //     debug_show(ui, self);
        // }

        // if let Some(screenshot) = &self.transient.temp_screenshot {
        //     FileDialog::default().save("test.png", screenshot.to_vec());
        //     self.transient.temp_screenshot = None;
        // }
    }
}