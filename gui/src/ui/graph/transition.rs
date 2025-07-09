use std::collections::HashMap;

use egui::{Pos2, Ui, Vec2};

use crate::{turing::{State, Transition}, App};

pub fn show(app: &mut App, ui: &mut Ui) {

    let mut transitions_hashmap: HashMap<(u8, u8), Vec<(&mut Transition, bool)>> = HashMap::new();

    let mut state_position: HashMap<u8, &State> = HashMap::new();

    let mut graph_center = Vec2::ZERO;
    let states_count = app.states.len();

    for (i, state) in app.states.iter_mut() {


        for transition in state.transitions.iter_mut() {

            let target_state_index = app.turing.get_state(*i).unwrap().transitions[transition.id as usize].index_to_state;

            let is_previous = app.step.unwrap().transition_taken == app.turing.get_state(*i).unwrap().transitions[transition.id as usize];
        }

        state_position.insert(*i, state);
    }
}



fn draw_transition(
    ui: &mut Ui,

) {

}