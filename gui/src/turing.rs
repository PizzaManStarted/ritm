use egui::{accesskit::Invalid, Color32, Pos2};
use rand::random_range;
use ritm_core::turing_state::{TuringDirection, TuringTransitionMultRibbons};

use crate::App;

/// State graphical representation
#[derive(PartialEq, Debug)]
pub struct State {
    pub id: usize,
    pub name: String,
    pub position: Pos2,
    pub is_pinned: bool,
    pub color: Color32,
    pub transitions: Vec<Transition>,
}

/// Transition are identified by the pair source state/own id
pub type TransitionId = (usize, usize);

/// Transition graphical representation
#[derive(Default, PartialEq, Debug)]
pub struct Transition {
    pub text: String,
    pub id: usize,
    pub parent_id: usize,
    pub target_id: usize,
    pub identifier: (usize, usize)
}

/// Copy of the [`TuringTransitionMultRibbons`] but with string to allow empty char
pub struct TuringTransitionString {
    pub chars_read: Vec<String>,
    pub move_read: TuringDirection,
    pub chars_write: Vec<(String, TuringDirection)>,
    pub index_to_state: Option<usize>,
}

impl TuringTransitionString {
    
    pub fn from(ttmr: &TuringTransitionMultRibbons) -> Self {

        Self {
            chars_read: ttmr.chars_read.iter().map(|f| f.to_string()).collect::<Vec<String>>(),
            move_read: ttmr.move_read.clone(),
            chars_write: ttmr.chars_write.iter().map(|(char, dir)| (char.to_string(), dir.clone())).collect::<Vec<(String, TuringDirection)>>(),
            index_to_state: ttmr.index_to_state
        }
    }

    pub fn to(&self) -> Result<TuringTransitionMultRibbons, Invalid> {

        if self.chars_read.iter().any(|string| string.is_empty())
        || self.chars_write.iter().any(|(string, _)| string.is_empty()) {
            return Err(Invalid::True);
        }

        Ok(TuringTransitionMultRibbons {
            chars_read: self.chars_read.iter().map(|string| string.chars().next().unwrap()).collect::<Vec<char>>(),
            move_read: self.move_read.clone(),
            chars_write: self.chars_write.iter().map(|(string, dir)| (string.chars().next().unwrap(), dir.clone())).collect::<Vec<(char, TuringDirection)>>(),
            index_to_state: self.index_to_state,
        })
    }
}

impl State {

    pub fn new_at_pos(id: usize, name: String, position: Pos2) -> Self {
        State {
            id: id,
            name: name,
            position: position,
            is_pinned: true,
            color: Color32::WHITE,
            transitions: vec![]
        }
    }

    pub fn new(id: usize, name: String) -> Self {
        State {
            id: id,
            name: name,
            position: Pos2::new(random_range(0.0..1.0), random_range(0.0..1.0)),
            is_pinned: true,
            color: Color32::WHITE,
            transitions: vec![]
        }
    }

    pub fn get(app: &App, id: usize) -> &Self {
        app.states.get(&id).unwrap()
    }

    pub fn get_mut(app: &mut App, id: usize) -> &mut Self {
        app.states.get_mut(&id).unwrap()
    }
}

impl Transition {

    pub fn new(text: String, id: usize, parent_id: usize, target_id: usize) -> Self {
        Transition {
            text: text,
            id: id,
            parent_id: parent_id,
            target_id: target_id,
            identifier: (parent_id, id),
        }
    }

    pub fn get(app: &App, identifier: (usize, usize)) -> &Self {
        State::get(app, identifier.0).transitions.get(identifier.1).unwrap()
    }

    pub fn get_mut(app: &mut App, identifier: (usize, usize)) -> &mut Self {
        State::get_mut(app, identifier.0).transitions.get_mut(identifier.1).unwrap()
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
            is_pinned: true,
        }
    }
}