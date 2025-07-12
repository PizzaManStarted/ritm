use egui::{Color32, Pos2};
use rand::random_range;

use crate::App;

/// State graphical representation
#[derive(PartialEq, Debug)]
pub struct State {
    pub id: u8,
    pub name: String,
    pub position: Pos2,
    pub color: Color32,
    pub transitions: Vec<Transition>,
}

/// Transition graphical representation
#[derive(Default, PartialEq, Debug)]
pub struct Transition {
    pub text: String,
    pub id: u8,
    pub parent_id: u8,
    pub target_id: u8,
    pub identifier: (u8, u8)
}

impl State {

    pub fn new_at_pos(id: u8, name: String, position: Pos2) -> Self {
        State {
            id: id,
            name: name,
            position: position,
            color: Color32::WHITE,
            transitions: vec![]
        }
    }

    pub fn new(id: u8, name: String) -> Self {
        State {
            id: id,
            name: name,
            position: Pos2::new(random_range(0.0..1.0), random_range(0.0..1.0)),
            color: Color32::WHITE,
            transitions: vec![]
        }
    }

    pub fn get(app: &App, id: u8) -> &Self {
        app.states.get(&id).unwrap()
    }

    pub fn get_mut(app: &mut App, id: u8) -> &mut Self {
        app.states.get_mut(&id).unwrap()
    }
}

impl Transition {

    pub fn new(text: String, id: u8, parent_id: u8, target_id: u8) -> Self {
        Transition {
            text: text,
            id: id,
            parent_id: parent_id,
            target_id: target_id,
            identifier: (parent_id, id),
        }
    }

    pub fn get(app: &App, identifier: (u8, u8)) -> &Self {
        State::get(app, identifier.0).transitions.get(identifier.1 as usize).unwrap()
    }

    pub fn get_mut(app: &mut App, identifier: (u8, u8)) -> &mut Self {
        State::get_mut(app, identifier.0).transitions.get_mut(identifier.1 as usize).unwrap()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            id: 0,
            color: Color32::WHITE,
            name: "N/a".to_string(),
            transitions: vec![],
            position: Pos2::new(random_range(0.0..1.0), random_range(0.0..1.0)),
        }
    }
}