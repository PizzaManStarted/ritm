

use std::collections::{BTreeMap, HashMap};

use egui::{Color32, FontData, FontDefinitions, FontFamily, Pos2, Rect};
use egui_extras::install_image_loaders;
use ritm_core::turing_machine::{TuringExecutionStep, TuringMachine};

use crate::{turing::{State, Transition}, ui::{self, theme::{Theme}}};

/// The only structure that is persistent each redraw of the application
pub struct App {
    /// The turing machine itself
    pub turing: TuringMachine,

    /// Current step of the turing machine or None if no step
    pub step: Option<TuringExecutionStep>,

    /// User input for the turing machine
    pub input: String,

    /// Used for graph display, zooming and moving
    pub graph_rect: Rect,

    /// Hold graph information of each states
    pub states: HashMap<u8, State>,

    /// The code used to create the turing machine
    pub code: String,

    /// The event/state of the application
    pub event: Event,

    /// Current theme
    pub theme: Theme,

    /// Selected state
    pub selected_state: Option<u8>,

    /// Selected transition
    pub selected_transition: Option<u8>,
}


/// Keep the state of the application
/// 
/// Used to check what the user see and/or can do
pub struct Event {

    /// Is the user adding a transition ?
    pub is_adding_transition: bool,

    /// Is the user adding a state ?
    pub is_adding_state: bool,

    /// Is the machine running ?
    pub is_running: bool,

    /// Is the input accepted ? None if result is not computed
    pub is_accepted: Option<bool>,

    /// Is the graph stable ?
    pub is_stable: bool,
}


impl Default for App {
    fn default() -> Self {

        let mut turing = TuringMachine::new(2);
        turing.add_state(&"test".to_string());

        let mut sf = Self {
            turing: turing,
            step: None,
            input: "".to_string(),
            graph_rect: Rect::ZERO,
            states: HashMap::new(),
            code: "".to_string(), // TODO display a message as comment instead
            event: Event::default(),
            theme: Theme::DEFAULT,
            selected_state: None,
            selected_transition: None,
        };

        // Update the graph data with the turing data at initialization
        sf.turing_to_graph();

        sf
    }
}


impl Default for Event {
    fn default() -> Self {
        Self {
            is_accepted: None,
            is_adding_transition: false,
            is_adding_state: false,
            is_running: false,
            is_stable: false,
        }
    }
}


impl App {

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // cc.egui_ctx.set_debug_on_hover(true);
        
        // Load the fonts used in the application
        load_font(cc);

        Default::default()
    }


    /// Add a state to the turing machine
    pub fn add_state(&mut self, position: Pos2) {

        let state_count = self.turing.name_index_hashmap.len();
        let default_name = format!("state{}", state_count+1);
        let id = self.turing.add_state(&default_name);
        self.states.insert(id, State {
            id: id,
            name: default_name,
            position,
            ..Default::default()
        });
    }


    /// Create a graphical representation of a turing machine by copying each states and transitions information into GUI-oriented struct
    pub fn turing_to_graph(&mut self) {

        self.states = HashMap::new();

        for (name, index) in self.turing.name_index_hashmap.iter() {

            let transitions: Vec<Transition> = self.turing.get_state(*index).unwrap().transitions.iter().enumerate().map(|(i, f)| Transition {
                text: f.to_string(),
                id: i as u8,
                parent_id: *index,
                target_id: f.index_to_state 
            }).collect();

            self.states.insert(
                *index,
                State {
                    id: *index,
                    name: name.to_string(),
                    transitions: transitions,
                    ..Default::default()
                }
            );
        }
    }
}



/// Update loop
impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        install_image_loaders(ctx);
        ui::show(self, ctx);
    }
}



/// Load the necessary font for the application
fn load_font(cc: &eframe::CreationContext<'_>) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "Roboto".into(),
        FontData::from_static(include_bytes!("../assets/fonts/Roboto.ttf")).into(),
    );
    fonts.font_data.insert(
        "Roboto-regular".into(),
        FontData::from_static(include_bytes!("../assets/fonts/Roboto-Regular.ttf")).into(),
    );
    fonts.font_data.insert(
        "RobotoMono-regular".into(),
        FontData::from_static(include_bytes!("../assets/fonts/RobotoMono-Regular.ttf")).into(),
    );


    let mut newfam = BTreeMap::new();
    newfam.insert(
        FontFamily::Name("Roboto".into()),
        vec!["Roboto".to_owned()]
    );
    newfam.insert(
        FontFamily::Name("Roboto-regular".into()),
        vec!["Roboto-regular".to_owned()],
    );
    newfam.insert(
        FontFamily::Name("RobotoMono-regular".into()),
        vec!["RobotoMono-regular".to_owned()],
    );
    fonts.families.append(&mut newfam);


    cc.egui_ctx.set_fonts(fonts);
}