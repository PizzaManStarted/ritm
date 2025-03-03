use std::fmt::Debug;

use crate::turing_state::{TuringRubon, TuringState};


pub struct TuringMachine
{
    states: Vec<TuringState>,
    reading_rubon: TuringRubon,
    write_rubons: TuringRubon,
}

impl TuringMachine {
    pub fn new() -> Self
    {
        // Add the write rubons
        let init_state = TuringState::new(false);
        let accepting_state = TuringState::new(true);

        Self
        {
            states: vec!(init_state, accepting_state),
            reading_rubon: TuringRubon::new(),
            write_rubons: TuringRubon::new()
        }


    }
}

impl Debug for TuringMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringMachine").field("states", &self.states).field("reading_rubon", &self.reading_rubon).field("write_rubons", &self.write_rubons).finish()
    }
}

