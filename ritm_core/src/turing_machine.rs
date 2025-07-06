use std::fmt::Display;

use rand::{rng, Rng};

use crate::{turing_errors::TuringError, turing_graph::TuringMachineGraph, turing_ribbon::{TuringReadRibbon, TuringRibbon, TuringWriteRibbon}, turing_state::TuringTransitionMultRibbons};


/// A trait used to iterate over all the states of a turing machine.
pub trait TuringIterator
{
    /// Gets the stored turing machine graph.
    fn get_turing_machine_graph(&self) -> &TuringMachineGraph;
    /// Gets the current state pointer of this struct. 
    fn get_state_pointer(&self) -> u8;
    /// Sets a new value to the state pointer.
    fn set_state_pointer(&mut self, new_val: u8);
    /// Gets the reading ribbon stored inside this struct.
    fn get_reading_ribbon(&mut self) -> &mut TuringReadRibbon;
    /// Gets the writtings ribbons stored inside this struct.
    fn get_writting_ribbons(&mut self) -> &mut Vec<TuringWriteRibbon>;
    /// Gets the word that was feed to this machine.
    fn get_word(&mut self) -> &String;
    /// Checks if the current iteration is the first iteration or not.
    fn is_first_iteration(&mut self) -> bool;
    /// Sets the state of this turing machine to be considered or not its first iteration.
    fn set_first_iteration(&mut self, set: bool);
    /// Transforms the current struct to a [TuringIterator] in order to start 
    /// iterating.
    fn as_iter(&mut self) -> &mut dyn TuringIterator;
}


impl dyn TuringIterator {
    /// Resets the turing machine to its initial state and re-feeds it the current stored word.
    pub fn reset(&mut self) -> Result<(), TuringError>
    {
        let word = self.get_word().clone();
        return self.reset_word(&word);
    }

    /// Resets the turing machine to its initial state and feeds it the given word.
    pub fn reset_word(&mut self, word: &String) -> Result<(), TuringError>
    {
        if word.is_empty() {
            return Err(TuringError::IllegalActionError { cause: String::from("Tried to feed an empty word to the turing machine") });
        }
        // Reset reading ribbon
        self.get_reading_ribbon().feed_word(word.clone());

        // Reset write ribbons
        for i in 0..self.get_writting_ribbons().len() {
            self.get_writting_ribbons()[i] = TuringWriteRibbon::new();
        }

        // Reset state pointers
        self.set_state_pointer(0);

        Ok(())
    }
}



/// A struct made to execute a word to a turing machine
pub struct TuringMachineWithRef<'a>
{
    /// The **reference** to a turing machine that will execute a word
    graph: &'a TuringMachineGraph,
    /// The reading rubbon containing the word
    reading_ribbon:  TuringReadRibbon,
    /// A vector containing all writting rubbons
    write_ribbons: Vec<TuringWriteRibbon>,
    /// The current word to read
    word: String,
    /// The index of the current state of the turing machine
    state_pointer: u8,
    /// Represents if the structs was just initialised or reset 
    is_first_state: bool
}

impl<'a> TuringMachineWithRef<'a> {
    /// Create a new [TuringMachineWithRef] for a given word.
    pub fn new(mt: &'a TuringMachineGraph, word: String) -> Result<Self, TuringError>
    {
        let mut s = 
        Self 
        {
            state_pointer: 0,
            reading_ribbon: TuringReadRibbon::new(),
            write_ribbons: {
                // Creates k ribbons
                let mut v = vec!();
                for _ in 0..mt.get_k()
                {
                    v.push(TuringWriteRibbon::new());
                }
                v
            },
            word,
            graph: mt,
            is_first_state: true
        };
        // Add the word to the reading ribbon
        s.reading_ribbon.feed_word(s.word.to_string());

        Ok(s)
    }
}



impl<'a> TuringIterator for TuringMachineWithRef<'a> {

    fn get_turing_machine_graph(&self) -> &TuringMachineGraph {
        self.graph
    }

    fn get_state_pointer(&self) -> u8 {
        self.state_pointer
    }

    fn set_state_pointer(&mut self, new_val: u8) {
        self.state_pointer = new_val;
    }

    fn get_reading_ribbon(&mut self) -> &mut TuringReadRibbon {
        &mut self.reading_ribbon
    }

    fn get_writting_ribbons(&mut self) -> &mut Vec<TuringWriteRibbon> {
        &mut self.write_ribbons
    }

    fn as_iter(&mut self) -> &mut dyn TuringIterator
    {
        self as &mut dyn TuringIterator
    }
    
    fn get_word(&mut self) -> &String {
        &self.word
    }
    
    fn is_first_iteration(&mut self) -> bool {
        self.is_first_state
    }
    
    fn set_first_iteration(&mut self, set: bool) {
        self.is_first_state = set;
    }
}


/// A struct representing an executable turing machine.
pub struct TuringMachine
{
    /// The turing machine that will execute a word
    turing_machine: TuringMachineGraph,
    /// The reading rubbon containing the word
    reading_ribbon:  TuringReadRibbon,
    /// A vector containing all writting rubbons
    write_ribbons: Vec<TuringWriteRibbon>,
    /// The current word to read
    word: String,
    /// The index of the current state of the turing machine
    state_pointer: u8,
    /// Represents if the structs was just initialised or reset 
    is_first_state: bool
}

impl TuringMachine {
    /// Create a new [TuringMachine] for a given graph and word.
    pub fn new(mt: TuringMachineGraph, word: String) -> Result<Self, TuringError>
    {
        if word.is_empty() {
            return Err(TuringError::IllegalActionError { cause: String::from("Tried to feed an empty word to the turing machine") });
        }
        let mut s = 
        Self 
        {
            state_pointer: 0,
            reading_ribbon: TuringReadRibbon::new(),
            write_ribbons: {
                // Creates k ribbons
                let mut v = vec!();
                for _ in 0..mt.get_k()
                {
                    v.push(TuringWriteRibbon::new());
                }
                v
            },
            word,
            turing_machine: mt,
            is_first_state: true,
        };
        // Add the word to the reading ribbon
        s.reading_ribbon.feed_word(s.word.to_string());
        
        Ok(s)
    }
}


impl TuringIterator for TuringMachine {
    fn get_turing_machine_graph(&self) -> &TuringMachineGraph {
        & self.turing_machine
    }

    fn get_state_pointer(&self) -> u8 {
        self.state_pointer
    }

    fn set_state_pointer(&mut self, new_val: u8) {
        self.state_pointer = new_val;
    }

    fn get_reading_ribbon(&mut self) -> &mut TuringReadRibbon {
        &mut self.reading_ribbon
    }

    fn get_writting_ribbons(&mut self) -> &mut Vec<TuringWriteRibbon> {
        &mut self.write_ribbons
    }
    
    fn as_iter(&mut self) -> &mut dyn TuringIterator
    {
        self as &mut dyn TuringIterator
    }

    fn get_word(&mut self) -> &String {
        &self.word
    }
    
    fn is_first_iteration(&mut self) -> bool {
        self.is_first_state
    }
    
    fn set_first_iteration(&mut self, set: bool) {
        self.is_first_state = set;
    }
}




pub struct TuringExecutionStep
{
    /// The index of the transition taken from the current state to the next one.
    pub transition_index_taken : Option<usize>,
    /// A clone of the transition that was just taken
    pub transition_taken : Option<TuringTransitionMultRibbons>,
    /// A clone representing the current state of the reading ribbon after taking that transition.
    pub read_ribbon: TuringReadRibbon,
    /// A clone representing the current state of the writting ribbons after taking that transition.
    pub write_ribbons: Vec<TuringWriteRibbon>
}


impl<'a> Iterator for &mut dyn TuringIterator
{
    type Item = TuringExecutionStep;
    

    fn next(&mut self) -> Option<Self::Item> 
    {
        if self.is_first_iteration() {
            self.set_first_iteration(false);

            return Some(TuringExecutionStep {
                transition_index_taken: None,
                transition_taken: None,
                read_ribbon: self.get_reading_ribbon().clone(),
                write_ribbons: self.get_writting_ribbons().clone(),
            });
        }


        // Fetch the current state
        let curr_state =  self.get_turing_machine_graph().get_state(self.get_state_pointer()).unwrap().clone();
        /* Checks if the state is accepting */
        if curr_state.is_final
        {
            return None;
        }

        // If one of the transition condition is true,
        // Get all current char read by **all** ribbons
        let mut char_vec = vec!(self.get_reading_ribbon().read_curr_char().clone());
        for ribbon in self.get_writting_ribbons() {
            char_vec.push(ribbon.read_curr_char());
        }
        let transitions = curr_state.get_valid_transitions(char_vec); 
        println!("{:?}", transitions);
        
        // If no transitions can be provided
        if transitions.len() == 0 
        {
            return None;
        }
        
        // Take a random transition (non deterministic)
        let transition_index_taken = rng().random_range(0..transitions.len());
        let transition = transitions[transition_index_taken];

        // Apply the transition
        // to the read ribbons
        self.get_reading_ribbon().try_apply_transition(transition.chars_read[0], ' ', &transition.move_read).unwrap();
        
        // to the write ribbons
        for i in 0..self.get_turing_machine_graph().get_k()
        {
            self.get_writting_ribbons()[i as usize].try_apply_transition(transition.chars_read[(i+1) as usize],
                                                                                    transition.chars_write[i as usize].0, &transition.chars_write[i as usize].1).unwrap();
        }

        // Move to the next state
        self.set_state_pointer(transition.index_to_state.unwrap());

        Some(TuringExecutionStep
        {
            transition_index_taken : Some(transition_index_taken),
            transition_taken: Some(transition.clone()),
            read_ribbon: self.get_reading_ribbon().clone(),
            write_ribbons: self.get_writting_ribbons().clone(),
        })
    }
}


impl<'a> Display for TuringExecutionStep{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        let mut write_str_rib = String::from(format!("{}",self.write_ribbons[0]));
        for i in 1..self.write_ribbons.len() 
        {
            write_str_rib.push_str(format!("\n{}", self.write_ribbons[i]).as_str());
        }
        let trans_taken = {
            if let Some(val) = &self.transition_taken {
                val.to_string()
            }
            else {
                String::from("None")
            }
        };

        write!(f, "* Took the following transition : {}\n* Ribbons:\nREAD:\n{}\nWRITE:\n{}", trans_taken, self.read_ribbon, write_str_rib)
    }
}