use egui::{Align2, Area, Frame, Id, Modal, Rect, Ui};

pub mod transition_edit;
pub mod help;
pub mod state_edit;
pub mod setting;


pub enum Popup {
    None,
    TransitionEdit,
    StateEdit,
    Setting,
    Help
}