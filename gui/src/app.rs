use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    sync::{Arc, atomic::AtomicBool},
};

use egui::{FontData, FontDefinitions, FontFamily, Pos2, Rect, vec2};
use egui_extras::install_image_loaders;
use rand::random;
use ritm_core::{
    turing_graph::TuringMachineGraph,
    turing_machine::{Mode, TuringExecutionSteps, TuringMachines},
    turing_state::{TuringDirection, TuringStateType, TuringTransitionMultRibbons},
};

use crate::{
    turing::{State, Transition, TuringTransitionString},
    ui::{self, theme::Theme},
};

/// The only structure that is persistent each redraw of the application
pub struct App {
    /// The turing machine itself
    pub turing: TuringMachines,

    /// Current step of the turing machine or None if no step
    pub step: (TuringExecutionSteps, usize),

    /// User input for the turing machine
    pub input: String,

    /// Used for graph display, zooming and moving
    pub graph_rect: Rect,

    /// Hold graph information of each states
    pub states: HashMap<usize, State>,

    /// The code used to create the turing machine
    pub code: String,

    /// The event/state of the application
    pub event: Event,

    /// Current theme
    pub theme: Theme,

    /// Selected state
    pub selected_state: Option<usize>,

    /// Selected transition
    pub selected_transition: Option<(usize, usize)>,

    /// Interval between each iteration
    pub interval: i32,

    pub rules_edit: Vec<TuringTransitionString>,
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

    /// Is the user moving as state around ?
    pub is_dragging: bool,

    /// Has the Graph changed ?
    pub has_changed: bool,

    /// Do we need to go to the next iteration ?
    pub is_next: Arc<AtomicBool>,

    /// Is the selected element in editing mode ?
    pub is_editing: bool,
}

impl Default for App {
    fn default() -> Self {
        let mut graph = TuringMachineGraph::new(1).ok().unwrap();
        {
            graph.add_state(&"q0".to_string());
            graph.add_state(&"q1".to_string());
            graph.add_state(&"q2".to_string());

            graph
                .append_rule_state_by_name(
                    &"i".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['ç', 'ç'],
                        move_read: TuringDirection::Right,
                        chars_write: vec![('ç', TuringDirection::Right)],
                        index_to_state: None,
                    },
                    &"q0".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q0".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['0', 'ç'],
                        move_read: TuringDirection::Right,
                        chars_write: vec![('0', TuringDirection::Right)],
                        index_to_state: None,
                    },
                    &"q0".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q0".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['1', 'ç'],
                        move_read: TuringDirection::Right,
                        chars_write: vec![('1', TuringDirection::Right)],
                        index_to_state: None,
                    },
                    &"q0".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q0".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['0', '_'],
                        move_read: TuringDirection::Right,
                        chars_write: vec![('0', TuringDirection::Right)],
                        index_to_state: None,
                    },
                    &"q0".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q0".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['1', '_'],
                        move_read: TuringDirection::Right,
                        chars_write: vec![('1', TuringDirection::Right)],
                        index_to_state: None,
                    },
                    &"q0".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q0".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['$', '_'],
                        move_read: TuringDirection::None,
                        chars_write: vec![('_', TuringDirection::Left)],
                        index_to_state: None,
                    },
                    &"q1".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q1".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['$', '0'],
                        move_read: TuringDirection::None,
                        chars_write: vec![('0', TuringDirection::Left)],
                        index_to_state: None,
                    },
                    &"q1".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q1".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['$', '1'],
                        move_read: TuringDirection::None,
                        chars_write: vec![('1', TuringDirection::Left)],
                        index_to_state: None,
                    },
                    &"q1".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q1".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['$', 'ç'],
                        move_read: TuringDirection::Left,
                        chars_write: vec![('_', TuringDirection::Right)],
                        index_to_state: None,
                    },
                    &"q2".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q2".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['0', '0'],
                        move_read: TuringDirection::Left,
                        chars_write: vec![('0', TuringDirection::Right)],
                        index_to_state: None,
                    },
                    &"q2".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q2".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['1', '1'],
                        move_read: TuringDirection::Left,
                        chars_write: vec![('1', TuringDirection::Right)],
                        index_to_state: None,
                    },
                    &"q2".to_string(),
                )
                .expect("failed to create transition");

            graph
                .append_rule_state_by_name(
                    &"q2".to_string(),
                    TuringTransitionMultRibbons {
                        chars_read: vec!['ç', '_'],
                        move_read: TuringDirection::None,
                        chars_write: vec![('_', TuringDirection::None)],
                        index_to_state: None,
                    },
                    &"a".to_string(),
                )
                .expect("failed to create transition");
        }

        let mut turing = TuringMachines::new(graph, "11011001".to_string(), Mode::SaveAll).unwrap();
        let step = turing.next().unwrap();

        let mut sf = Self {
            turing: turing,
            step: (step, 0),
            input: "11011011".to_string(),
            graph_rect: Rect::ZERO,
            states: HashMap::new(),
            code: "".to_string(), // TODO display a message as comment instead
            event: Event::default(),
            theme: Theme::DEFAULT,
            selected_state: None,
            selected_transition: None,
            interval: 0,
            rules_edit: vec![],
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
            is_dragging: false,
            has_changed: false,
            is_next: AtomicBool::new(false).into(),
            is_editing: false,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // cc.egui_ctx.set_debug_on_hover(true);

        // Load the fonts used in the application
        load_font(cc);

        let app: App = Default::default();

        Theme::set_global_theme(&app.theme, &cc.egui_ctx);

        app
    }

    /// Add a state to the turing machine at a certain position
    pub fn add_state(&mut self, position: Pos2) {
        let state_count = self.turing.graph().name_index_hashmap.len();
        let default_name = format!("state{}", state_count + 1);

        // Create the state in the core of the application to miror the id into
        // the gui structure
        let id = self.turing.graph_mut().add_state(&default_name);
        self.states.insert(
            id,
            State {
                id: id,
                name: default_name,
                position,
                ..Default::default()
            },
        );
    }

    /// Remove the selected state from the core and the gui structure
    pub fn remove_state(&mut self) {
        for (_, state) in self.states.iter_mut() {
            state
                .transitions
                .retain(|t| t.target_id != self.selected_state.unwrap());
        }
        self.states.remove(&self.selected_state.unwrap());

        self.selected_state = None;
    }

    /// Add a transition graphically and logically
    pub fn add_transition(&mut self, target: usize) {
        let transition = TuringTransitionMultRibbons::new(
            vec!['ç'; self.turing.graph().get_k() as usize],
            TuringDirection::None,
            vec![('ç', TuringDirection::None); self.turing.graph().get_k() as usize],
        );

        let transition_rule = transition.to_string();
        self.turing
            .graph_mut()
            .append_rule_state(self.selected_state.unwrap(), transition, target)
            .ok();

        let source = State::get_mut(self, self.selected_state.unwrap());
        let transition =
            Transition::new(transition_rule, source.transitions.len(), source.id, target);
        source.transitions.push(transition);
        self.event.is_adding_transition = false;
    }

    /// Continue the execution of the turing machine by one iteration
    pub fn next(&mut self) {
        match self.turing.next() {
            Some(step) => self.step = (step, self.step.1 + 1),
            None => {
                // Store the result of the computation
                self.event.is_accepted = Some(
                    self.turing
                        .graph()
                        .get_state(self.turing.get_state_pointer())
                        .unwrap()
                        .state_type
                        == TuringStateType::Accepting,
                );

                // Disable auto-play if the user reset the machine
                self.event.is_running = false;
            }
        }
    }

    /// Reset the machine execution
    pub fn reset(&mut self) {
        self.turing = TuringMachines::new(
            self.turing.graph().clone(),
            self.turing.get_word().to_string(),
            self.turing.get_mode().clone(),
        )
        .unwrap();
        self.step = (self.turing.next().unwrap(), 0);
        self.event.is_accepted = None;
    }

    /// Reset the machine execution with the new input
    pub fn set_input(&mut self) {
        let _ = self.turing.reset();
        self.step = (self.turing.next().unwrap(), 0);
        self.event.is_accepted = None;
    }

    /// Apply the changes made to the transition
    pub fn apply_transition_change(&mut self) {
        // Need to have a transition selected
        
        if let Some(selected_transition) = self.selected_transition && !self.rules_edit.iter().any(|transition| transition.to().is_err()) {
            // Reset the machine to avoid problem when removing transition, especially transition_taken
            // MEMO : maybe block the removing of the last transition taken ?
            let _ = self.turing.reset();

            let transitions = &mut self.turing
                .graph_mut()
                .get_state_mut(selected_transition.0)
                .unwrap()
                .transitions;
            transitions.clear();
            transitions.append(self.rules_edit.iter().map(|transition| transition.to().unwrap()).collect::<Vec<TuringTransitionMultRibbons>>().as_mut());
            
            let transitions_gui: Vec<Transition> = transitions
                .iter()
                .enumerate()
                .map(|(i, f)| Transition::new(f.to_string(), i, selected_transition.0, f.index_to_state.unwrap()))
                .collect();

            State::get_mut(self, selected_transition.0).transitions = transitions_gui;
        };
        self.event.is_editing = false;
    }

    /// Cancel the changes made to the transition
    pub fn cancel_transition_change(&mut self) {
        self.rules_edit.clear();
        self.event.is_editing = false;
    }

    /// Create a graphical representation of a turing machine by copying each states and transitions information into GUI-oriented struct
    pub fn turing_to_graph(&mut self) {
        self.states = HashMap::new();

        let mut state_list: BTreeSet<usize> = BTreeSet::new();
        let mut layer_state: Vec<usize> = vec![];

        for (name, index) in self.turing.graph().name_index_hashmap.iter() {
            let state = self.turing.graph().get_state(*index).unwrap();

            let transitions: Vec<Transition> = state
                .transitions
                .iter()
                .enumerate()
                .map(|(i, f)| Transition::new(f.to_string(), i, *index, f.index_to_state.unwrap()))
                .collect();

            if state.state_type == TuringStateType::Accepting || state.transitions.is_empty() {
                layer_state.push(*index);
            } else {
                state_list.insert(*index);
            }

            self.states.insert(
                *index,
                State {
                    id: *index,
                    name: name.to_string(),
                    transitions: transitions,
                    ..Default::default()
                },
            );
        }

        let mut j = 0.0;
        while !(state_list.is_empty() && layer_state.is_empty()) {
            let mut next_layer_state: Vec<usize> = vec![];

            let layer_count = layer_state.len() as f32 - 1.0;
            for (i, state_id) in layer_state.iter().enumerate() {
                State::get_mut(self, *state_id).position = Pos2::new(
                    (j - (layer_count / 2.0 - i as f32)) * -200.0,
                    (j + (layer_count / 2.0 - i as f32)) * -200.0,
                ) + vec2(random(), random());
            }

            for state_id in &state_list {
                let state = self.turing.graph().get_state(*state_id).unwrap();
                if state
                    .transitions
                    .iter()
                    .any(|f| layer_state.contains(&f.index_to_state.unwrap()))
                {
                    next_layer_state.push(*state_id);
                }
            }

            state_list.retain(|k| !next_layer_state.contains(k));

            layer_state = next_layer_state;

            j += 1.0;
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
        "RobotoMono-regular".into(),
        FontData::from_static(include_bytes!("../assets/fonts/RobotoMono-Regular.ttf")).into(),
    );
    fonts.font_data.insert(
        "RobotoMono-Bold".into(),
        FontData::from_static(include_bytes!("../assets/fonts/RobotoMono-Bold.ttf")).into(),
    );

    let mut newfam = BTreeMap::new();

    newfam.insert(
        FontFamily::Name("RobotoMono-Bold".into()),
        vec!["RobotoMono-Bold".to_owned()],
    );
    newfam.insert(
        FontFamily::Name("RobotoMono-regular".into()),
        vec!["RobotoMono-regular".to_owned()],
    );
    fonts.families.append(&mut newfam);

    cc.egui_ctx.set_fonts(fonts);
}
