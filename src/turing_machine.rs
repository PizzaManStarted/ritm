use std::{collections::HashMap, fmt::Debug};

use crate::{turing_errors::TuringError, turing_state::{TuringDirection, TuringRibbon, TuringState, TuringTransition, TuringWriteRibbon}};


/// A struct representing a Turing Machine with k rubbons.
pub struct TuringMachine
{
    name_index_hashmap: HashMap<String, u8>, 
    states: Vec<TuringState>,
    k: u8,
}

impl TuringMachine {
    /// Creates a new empty Turing Machine that has `k` writting rubbons.
    pub fn new(k: u8) -> Self
    {
        // Add the write ribbons
        let init_state = TuringState::new(false);
        let accepting_state = TuringState::new(true);
        let rejecting_state = TuringState::new(false);
        
        // Create the hash map with the already known states
        let mut name_index_hashmap: HashMap<String, u8> = HashMap::new();
        name_index_hashmap.insert("q_0".to_string(), 0);    // init
        name_index_hashmap.insert("q_a".to_string(), 1);    // accepting
        name_index_hashmap.insert("q_r".to_string(), 2);    // rejecting

        let s = Self
        {
            name_index_hashmap,
            states: vec!(init_state, accepting_state, rejecting_state),
            k,
        };

        s
    }

    /// Adds a new rule to a state of the machine.
    /// 
    /// If the given state didn't already exists, the state will be created.
    pub fn add_rule_state(&mut self, from: String, mut transition: TuringTransition, to: String) -> Result<(), TuringError>
    {
        // Checks if the given correct of number transitions was given
        if transition.chars_write.len() != self.k as usize
        {
            return Err(TuringError::NotEnougthArgsError);
        }
        
        let from_index = *self.name_index_hashmap.get(&from).unwrap(); // FIXME : Change unwrap
        let to_index = *self.name_index_hashmap.get(&to).unwrap();
        // Change transition index
        transition.index_to_state = to_index;

        let state = self.states.get_mut(from_index as usize).unwrap();
        state.add_transition(transition);
        return Ok(());
    }
}







impl Debug for TuringMachine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringMachine").field("states", &self.states).finish()
    }
}

/// A struct made to execute a word to a turing machine
pub struct TuringMachineExecutor<'a>
{
    /// The turing machine that will execute a word
    turing_machine: &'a TuringMachine,
    /// The reading rubbon containing the word
    reading_ribbon:  TuringWriteRibbon,
    /// A vector containing all writting rubbons
    write_ribbons: Vec<TuringWriteRibbon>,
    /// The current word to read
    word: String,
    /// The index of the current state of the turing machine
    state_pointer: u8,
}


impl<'a> TuringMachineExecutor<'a> {
    /// Create a new [TuringMachineExecutor] for a given word.
    pub fn new(mt: &'a TuringMachine, word: String) -> Self
    {
        let mut s = 
        Self 
        {
            state_pointer: 0,
            reading_ribbon: TuringWriteRibbon::new(),
            write_ribbons: {
                // Creates k ribbons
                let mut v = vec!();
                for _ in 0..mt.k
                {
                    v.push(TuringWriteRibbon::new());
                }
                v
            },
            word,
            turing_machine: mt,
        };
        s.reading_ribbon.transition_state('ç', 'ç', TuringDirection::Right);
        s.reading_ribbon.transition_state('_', 'a', TuringDirection::Right);
        s.reading_ribbon.transition_state('_', 'b', TuringDirection::Left);
        s.reading_ribbon.transition_state('a', 'g', TuringDirection::Right);
        println!("{}", s.reading_ribbon);
        s
    }
}