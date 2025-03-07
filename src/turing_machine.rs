use std::{collections::HashMap, fmt::Debug};
use rand::{rng, Rng};
use crate::{turing_errors::TuringError, turing_ribbon::{TuringReadRibbon, TuringRibbon, TuringWriteRibbon}, turing_state::{TuringState, TuringTransition}};


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
    pub fn add_rule_state(&mut self, from: String, transition: TuringTransition, to: String) -> Result<(), TuringError>
    {
        // Checks if the given correct of number transitions was given
        if transition.chars_write.len() != self.k as usize
        {
            return Err(TuringError::NotEnougthArgsError);
        }
        let from_index = self.add_state(&from);
        let to_index = self.add_state(&to);

        return self.add_rule_state_ind(from_index, transition, to_index);
    }

    /// Adds a new state to the turing machine and returns it's index.
    /// 
    /// If the state name already existed then the index of the already existing state is added.
    pub fn add_state(&mut self, name: &String) -> u8
    {
        match self.name_index_hashmap.get(name) 
        {
            Some(e) => {
                return *e;
            },
            None => 
            {
                self.states.push(TuringState::new(false).set_name(name.to_string()));
                self.name_index_hashmap.insert(name.to_string(), (self.states.len()-1) as u8);
                return (self.states.len()-1) as u8;
            },
        }
    }


    fn add_rule_state_ind(&mut self, from: u8, mut transition: TuringTransition, to: u8) -> Result<(), TuringError>
    {
        // Change transition index
        transition.index_to_state = to;

        let state  = self.states.get_mut(from as usize).unwrap();
        state.add_transition(transition);
        return Ok(());
    }

    pub fn get_state(&self, pointer: u8) -> &TuringState
    {
        return &self.states[pointer as usize];
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
    reading_ribbon:  TuringReadRibbon,
    /// A vector containing all writting rubbons
    write_ribbons: Vec<TuringWriteRibbon>,
    /// The current word to read
    word: String,
    /// The index of the current state of the turing machine
    state_pointer: u8,
}


impl<'a> TuringMachineExecutor<'a> {
    /// Create a new [TuringMachineExecutor] for a given word.
    pub fn new(mt: &'a TuringMachine, word: String) -> Result<Self, TuringError>
    {
        let mut s = 
        Self 
        {
            state_pointer: 0,
            reading_ribbon: TuringReadRibbon::new(),
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
        // Add the word to the reading ribbon
        s.reading_ribbon.feed_word(s.word.to_string());

        // s.reading_ribbon.transition_state('ç', 'ç', TuringDirection::Right);
        // s.reading_ribbon.transition_state('_', 'a', TuringDirection::Right);
        // s.reading_ribbon.transition_state('_', 'b', TuringDirection::Left);
        // s.reading_ribbon.transition_state('a', 'g', TuringDirection::Right);
        //println!("{}", s.reading_ribbon);
        Ok(s)
    }
}


impl<'a> Iterator for TuringMachineExecutor<'a> 
{
    type Item = ();
    

    fn next(&mut self) -> Option<Self::Item> 
    {
        // Fetch the current state
        let curr_state = self.turing_machine.get_state(self.state_pointer);
        // If one of the transition condition is true,
        // Get all current char read by **all** ribbons
        let mut char_vec = vec!(self.reading_ribbon.read_curr_char());
        for ribbon in &self.write_ribbons {
            char_vec.push(ribbon.read_curr_char());
        }
        let transitions = curr_state.get_valid_transitions(char_vec); 
        println!("{:?}", curr_state);
        
        // If no transitions can be provided
        if transitions.len() == 0 
        {
            return None;
        }
        
        // Take a random transition (non deterministic)
        let transition = transitions[rng().random_range(0..transitions.len())];

        /* TRANSITION */
        println!("What is this : {}", transition);

        // Apply the transition
        // to the read ribbons
        self.reading_ribbon.transition_state(transition.chars_read[0], ' ', &transition.move_read).unwrap();
        
        // to the write ribbons
        for i in 0..self.turing_machine.k 
        {
            self.write_ribbons[i as usize].transition_state(transition.chars_read[(i+1) as usize],
                                                                                    transition.chars_write[i as usize].0, &transition.chars_write[i as usize].1).unwrap();
        }

        // Move to the next state
        self.state_pointer = transition.index_to_state;


        // TODO CHECK IF CURRENT STATE IS ACCEPTING
        Some(())
    }
}