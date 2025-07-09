use egui::{Color32, Pos2};
use rand::random_range;

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
    pub parent_id : u8,
    pub target_id : u8,
}

impl State {

    pub fn new_at_pos(id: u8, name: String, position: Pos2) -> State {
        State {
            id: id,
            name: name,
            position: position,
            color: Color32::WHITE,
            transitions: vec![]
        }
    }

    pub fn new(id: u8, name: String) -> State {
        State {
            id: id,
            name: name,
            position: Pos2::new(random_range(0.0..1.0), random_range(0.0..1.0)),
            color: Color32::WHITE,
            transitions: vec![]
        }
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