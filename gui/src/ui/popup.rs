
pub mod help;
pub mod setting;
pub mod state_edit;
pub mod transition_edit;

#[derive(PartialEq)]
pub enum RitmPopup {
    None,
    TransitionEdit,
    StateEdit,
    Setting,
    Help,
}
