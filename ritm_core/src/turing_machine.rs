use std::{collections::{vec_deque, VecDeque}, fmt::{Debug, Display}, os::linux::raw::stat, path::Iter};

use rand::{rng, Rng};

use crate::{turing_errors::TuringError, turing_graph::TuringMachineGraph, turing_ribbon::{TuringReadRibbon, TuringRibbon, TuringWriteRibbon}, turing_state::{TuringState, TuringStateType, TuringTransitionMultRibbons}};


/// Represents the different mode a turing machine can have during it's execution
pub enum Mode {
    SaveAll, // May god bless your ram
    StopAfter(usize),
    OverwriteAfter(usize),
    StopFirstReject,
}


pub struct SavedState {
    /// The index of the saved state
    saved_state_index : u8,
    /// A stack containing all the indexes of the transitions left to take 
    next_transitions : VecDeque<u8>,
    /// The value of the [TuringReadRibbon] when it was saved
    saved_read_ribbon : TuringReadRibbon,
    /// The value of the [TuringWriteRibbon] when they were saved
    saved_write_ribbons : Vec<TuringWriteRibbon>
}

impl Debug for SavedState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SavedState").field("saved_state", &self.saved_state_index).field("next_transitions", &self.next_transitions).field("saved_read_ribbon", &self.saved_read_ribbon.to_string()).field("saved_write_ribbons", &self.saved_write_ribbons).finish()
    }
}

impl Clone for SavedState {
    fn clone(&self) -> Self {
        Self { saved_state_index: self.saved_state_index.clone(), next_transitions: self.next_transitions.clone(), saved_read_ribbon: self.saved_read_ribbon.clone(), saved_write_ribbons: self.saved_write_ribbons.clone() }
    }
}


pub enum TuringMachines<'a>
{
    TuringMachineWithRef {
        /// The **reference** to a turing machine graph that will execute a word
        graph : &'a TuringMachineGraph,
        data : IterationData
    },
    TuringMachine {
        /// The turing machine graph that will execute a word
        graph : TuringMachineGraph,
        data : IterationData
    }
}

struct IterationData {
    /// The reading rubbon containing the word
    reading_ribbon:  TuringReadRibbon,
    /// A vector containing all writting rubbons
    write_ribbons: Vec<TuringWriteRibbon>,
    /// The current word to read
    word: String,
    /// The index of the current state of the turing machine
    state_pointer: u8,
    /// Represents if the structs was just initialised or reset 
    is_first_state: bool,
    /// A stack representing the memory of the exploration of this turing machine.
    memory: VecDeque<SavedState>,
    /// Represents the mode used for the execution of this turing machine
    mode: Mode
}

impl<'a> TuringMachines<'a> 
{
    /// Create a new [TuringIteratorE::TuringMachineWithRef] for a given word.
    pub fn new_with_ref(mt: &'a TuringMachineGraph, word: String, mode: Mode) -> Result<Self, TuringError>
    {
        let mut s = 
        TuringMachines::TuringMachineWithRef
        {
            graph: mt,
            data : IterationData {
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
                word: word.clone(),
                is_first_state: true,
                memory: VecDeque::new(),
                mode

            }
        };
        // Add the word to the reading ribbon
        s.get_reading_ribbon().feed_word(word);

        Ok(s)
    }

    // Create a new [TuringMachineWithRef] for a given word.
    pub fn new(mt: TuringMachineGraph, word: String, mode: Mode) -> Result<Self, TuringError>
    {
        if word.is_empty() {
            return Err(TuringError::IllegalActionError { cause: String::from("Tried to feed an empty word to the turing machine") });
        }
        let mut s = 
        TuringMachines::TuringMachine 
        {
            data : IterationData {
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
                word: word.clone(),
                is_first_state: true,
                memory: VecDeque::new(),
                mode
            },
            graph: mt
        };
        // Add the word to the reading ribbon
        s.get_reading_ribbon().feed_word(word);
        
        Ok(s)
    }

    /// Adds a new [SavedState] to the front of the memory stack.
    fn push_to_memory_stack(&mut self, to_save: SavedState)
    {
        self.get_memory_mut().push_front(to_save);
    }

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

        // Reset first iteration
        self.set_first_iteration(true);
        
        Ok(())
    }

}


impl TuringMachines<'_> {
    /// Gets the stored turing machine graph.
    fn get_turing_machine_graph(&self) -> &TuringMachineGraph {
        match self {
            TuringMachines::TuringMachineWithRef { graph, data:_ } => &graph,
            TuringMachines::TuringMachine { graph, data:_ } => &graph,
        }
    }
    
    /// Gets the current state pointer of this struct.
    fn get_state_pointer(&self) -> u8 {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => data.state_pointer,
        }
        
    }
    
    /// Sets a new value to the state pointer.
    fn set_state_pointer(&mut self, new_val: u8) {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => data.state_pointer = new_val,
        }
    }
    
    /// Gets the reading ribbon stored inside this struct.
    fn get_reading_ribbon(&mut self) -> &mut TuringReadRibbon {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => &mut data.reading_ribbon,
        }
    }
    

    /// Sets the reading ribbon stored inside this struct.
    fn set_reading_ribbon(&mut self, ribbon: TuringReadRibbon) {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => data.reading_ribbon = ribbon,
        }
    }
    

    /// Gets the writtings ribbons stored inside this struct.
    fn get_writting_ribbons(&mut self) -> &mut Vec<TuringWriteRibbon> {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => &mut data.write_ribbons,
        }
    }
    

    /// Sets the writting ribbons stored inside this struct.
    fn set_writting_ribbons(&mut self, ribbons: Vec<TuringWriteRibbon>) {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => data.write_ribbons = ribbons,
        }
    }
    
    /// Gets the word that was feed to this machine.
    fn get_word(&self) -> &String {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => &data.word,
        }
    }
    

    /// Checks if the current iteration is the first iteration or not.
    fn is_first_iteration(&mut self) -> bool {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => data.is_first_state,
        }
    }
    
    /// Sets the state of this turing machine to be considered or not its first iteration.
    fn set_first_iteration(&mut self, set: bool) {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => data.is_first_state = set,
        }
    }
    
    /// Fetches the mode of the iterator.
    fn get_mode(&self) -> &Mode {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => &data.mode,
        }
    }
    
    /// Get the **mutable** stack containing all the [SavedState].
    fn get_memory_mut(&mut self) -> &mut VecDeque<SavedState> {
        match self {
            TuringMachines::TuringMachineWithRef { graph:_, data } | TuringMachines::TuringMachine { graph:_, data } => &mut data.memory,
        }
    }
}



pub struct TuringExecutionStep
{
    pub reached_state : TuringState,
    /// The index of the transition taken from the current state to the next one.
    pub transition_index_taken : Option<usize>,
    /// A clone of the transition that was just taken
    pub transition_taken : Option<TuringTransitionMultRibbons>,
    /// A clone representing the current state of the reading ribbon after taking that transition.
    pub read_ribbon: TuringReadRibbon,
    /// A clone representing the current state of the writting ribbons after taking that transition.
    pub write_ribbons: Vec<TuringWriteRibbon>,
    /// Is set to true when this step resulted directly from a backtrack compared to the previous state.
    pub backtracked : Option<u8>,
}


impl<'a> Iterator for TuringMachines<'_>
{
    type Item = TuringExecutionStep;
    

    fn next(&mut self) -> Option<Self::Item> 
    {
        // Fetch the current state
        let curr_state =  self.get_turing_machine_graph().get_state(self.get_state_pointer()).unwrap().clone();

        if self.is_first_iteration() {
            self.set_first_iteration(false);

            return Some(TuringExecutionStep {
                reached_state: curr_state,
                transition_index_taken: None,
                transition_taken: None,
                read_ribbon: self.get_reading_ribbon().clone(),
                write_ribbons: self.get_writting_ribbons().clone(),
                backtracked: None
            });
        }

        /* Checks if the state is accepting */
        if let TuringStateType::Accepting = curr_state.state_type
        {
            // The iteration is over
            return None;
        }


        // If it's rejecting or normal


        // If one of the transition condition is true,
        // Get all current char read by **all** ribbons
        let mut char_vec = vec!(self.get_reading_ribbon().read_curr_char().clone());
        for ribbon in self.get_writting_ribbons() {
            char_vec.push(ribbon.read_curr_char());
        }
        
        println!("Currently reading : {:?}", char_vec);
        let transitions = curr_state.get_valid_transitions(&char_vec);
        println!("transitions possible : {:?}", transitions);
        
        let mut transition_index_taken = 0 as u8;
        // If there are more than 1 transition possible at a time, it means we are in a non deterministic situation.
        // We must save the current state in order to explore all path.
        if transitions.len() >= 2 {
            // FIXME there is a problem with the logic of giving a transition
            // take the first transition, save the rest
            let mut next_transitions = VecDeque::from(curr_state.get_valid_transitions_indexes(&char_vec));
            transition_index_taken = next_transitions.pop_front().unwrap();

            let to_save = SavedState { saved_state_index:self.get_state_pointer(), 
                                                    next_transitions: next_transitions, 
                                                    saved_read_ribbon: self.get_reading_ribbon().clone(), 
                                                    saved_write_ribbons: self.get_writting_ribbons().clone() };

            self.push_to_memory_stack(to_save);
        }

        
        let mut backtracked = None;
        // If no transitions can be provided or the current state is rejecting,
        // we reached a *dead end*, go back in the exploration if possible
        if transitions.len() == 0 || curr_state.state_type == TuringStateType::Rejecting
        {
            // If there are no saved state, this means the backtracking is over, and the execution too
            if self.get_memory_mut().is_empty() {
                return None;
            }


            // While the memory still has a state saved

            while !self.get_memory_mut().is_empty() {
                println!("{:?}\n\nFUCK\n", self.get_memory_mut());
                {
                    let saved_state = self.get_memory_mut().front_mut().unwrap();
                    
                    
                    // Get the next transition to take
                    if let Some(t_i) = saved_state.next_transitions.pop_front() {
                        transition_index_taken = t_i;
                    }
                    else {
                        // If no transition is left to take for this state, we move on to the next one
                        continue;
                    }
                    println!("Saved state after next transitions : {:?}", saved_state.next_transitions);
                }
                // obliged to clone because of the mutable nature
                let saved_state = self.get_memory_mut().front().unwrap().clone();
                
                // Go back to the state
                self.set_state_pointer(saved_state.saved_state_index);
                
                // Change the context for the reading and writing ribbons
                self.set_reading_ribbon(saved_state.saved_read_ribbon);
                self.set_writting_ribbons(saved_state.saved_write_ribbons);
                
                backtracked = Some(saved_state.saved_state_index);
                break;
            }
        }

        let transition = self.get_turing_machine_graph().get_state(self.get_state_pointer()).unwrap().transitions[transition_index_taken as usize].clone();
        println!("Took : {}", transition);
        
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
            reached_state: self.get_turing_machine_graph().get_state(self.get_state_pointer()).unwrap().clone(),
            transition_index_taken : Some(transition_index_taken as usize),
            transition_taken: Some(transition.clone()),
            read_ribbon: self.get_reading_ribbon().clone(),
            write_ribbons: self.get_writting_ribbons().clone(),
            backtracked,
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

        write!(f, "* Current state : {}\n* Took the following transition : {}\n* Ribbons:\nREAD:\n{}\nWRITE:\n{}\nBacktracked ? {:?}", self.reached_state, trans_taken, self.read_ribbon, write_str_rib, self.backtracked)
    }
}


