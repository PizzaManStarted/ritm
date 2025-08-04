use std::{collections::VecDeque, fmt::{Debug, Display}};


use crate::{turing_errors::TuringError, turing_graph::TuringMachineGraph, turing_ribbon::{TuringReadRibbon, TuringRibbon, TuringWriteRibbon}, turing_state::{TuringState, TuringStateType, TuringTransitionMultRibbons}};


/// Represents the different mode a turing machine can have during it's execution
pub enum Mode {
    /// Explores all possible paths (and possibilities using backtracking) until an accepting state is found or no path is left is to take. 
    SaveAll, // May god bless your ram
    /// Stops after the specified amount of iteration is reached even if the execution is not over.
    StopAfter(usize),
    /// Stops after meeting the first rejecting state or when the execution is blocked, even if backtracking is possible 
    StopFirstReject,
}


pub struct SavedState {
    /// The index of the saved state
    saved_state_index : usize,
    /// A stack containing all the indexes of the transitions left to take 
    next_transitions : VecDeque<usize>,
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


pub enum TuringMachines
{
    TuringMachine {
        /// The turing machine graph that will execute a word
        graph : TuringMachineGraph,
        data : IterationData,
        /// The current number of iterations already done
        iteration : usize
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
    state_pointer: usize,
    /// Represents if the structs was just initialised or reset 
    is_first_state: bool,
    /// A stack representing the memory of the exploration of this turing machine.
    memory: VecDeque<SavedState>,
    /// Represents the mode used for the execution of this turing machine
    mode: Mode,
    backtracked_info: Option<usize>
}

impl TuringMachines
{

    // Create a new [TuringMachineWithRef] for a given word.
    pub fn new(mt: TuringMachineGraph, word: String, mode: Mode) -> Result<Self, TuringError>
    {
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
                mode,
                backtracked_info: None
            },
            graph: mt,
            iteration : 0
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
        self.set_iteration(0);
        let word = self.get_word().clone();
        return self.reset_word(&word);
    }

    /// Resets the turing machine to its initial state and feeds it the given word.
    pub fn reset_word(&mut self, word: &String) -> Result<(), TuringError>
    {
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


impl TuringMachines {
    /// Gets *reference* of the stored turing machine graph.
    pub fn graph_ref(&self) -> &TuringMachineGraph {
        match self {
            TuringMachines::TuringMachine { graph, data:_, iteration:_ } => &graph,
        }
    }

    /// Gets *mutable reference* of the stored turing machine graph.
    pub fn graph_mut(&mut self) -> &mut TuringMachineGraph {
        match self {
            TuringMachines::TuringMachine { graph, data:_, iteration:_ } => graph,
        }
    }

    /// Gets the stored turing machine graph.
    /// 
    /// This will free the turing machine since it will drop the ownership
    pub fn graph(self) -> TuringMachineGraph {
        match self {
            TuringMachines::TuringMachine { graph, data:_, iteration:_ } => graph,
        }
    }

    
    
    /// Gets the current state pointer of this struct.
    pub fn get_state_pointer(&self) -> usize {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => data.state_pointer,
        }
        
    }
    
    /// Sets a new value to the state pointer.
    fn set_state_pointer(&mut self, new_val: usize) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => data.state_pointer = new_val,
        }
    }
    
    /// Gets the reading ribbon stored inside this struct.
    fn get_reading_ribbon(&mut self) -> &mut TuringReadRibbon {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => &mut data.reading_ribbon,
        }
    }
    

    /// Sets the reading ribbon stored inside this struct.
    fn set_reading_ribbon(&mut self, ribbon: TuringReadRibbon) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => data.reading_ribbon = ribbon,
        }
    }
    

    /// Gets the writtings ribbons stored inside this struct.
    fn get_writting_ribbons(&mut self) -> &mut Vec<TuringWriteRibbon> {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => &mut data.write_ribbons,
        }
    }
    

    /// Sets the writting ribbons stored inside this struct.
    fn set_writting_ribbons(&mut self, ribbons: Vec<TuringWriteRibbon>) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => data.write_ribbons = ribbons,
        }
    }
    
    /// Gets the word that was feed to this machine.
    fn get_word(&self) -> &String {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => &data.word,
        }
    }
    

    /// Checks if the current iteration is the first iteration or not.
    fn is_first_iteration(&mut self) -> bool {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => data.is_first_state,
        }
    }
    
    /// Sets the state of this turing machine to be considered or not its first iteration.
    fn set_first_iteration(&mut self, set: bool) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => data.is_first_state = set,
        }
    }
    
    /// Fetches the mode of the iterator.
    fn get_mode(&self) -> &Mode {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => &data.mode,
        }
    }
    
    /// Get the **mutable** stack containing all the [SavedState].
    fn get_memory_mut(&mut self) -> &mut VecDeque<SavedState> {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => &mut data.memory,
        }
    }

    fn get_backtracking_info(&self) -> Option<usize> {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => data.backtracked_info,
        }
    }
    fn set_backtracking_info(&mut self, val: Option<usize>) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_ } => data.backtracked_info = val,
        }
    }

    fn set_iteration(&mut self, val: usize) {
        match self {
            TuringMachines::TuringMachine { graph:_, data:_, iteration } => *iteration = val,
        }
    }

    fn get_iteration(&self) -> usize {
        match self {
            TuringMachines::TuringMachine { graph:_, data:_, iteration } => *iteration,
        }
    }

}


pub enum TuringExecutionSteps
{
    FirstIteration {
        /// A clone of the initial state
        init_state: TuringState,
        /// A clone representing the initial state of the reading ribbon.
        init_read_ribbon: TuringReadRibbon,
        /// A clone representing the initial state of the writting ribbons.
        init_write_ribbons: Vec<TuringWriteRibbon>,

    },
    TransitionTaken
    {
        /// A clone of the state that was just left
        previous_state : TuringState,
        /// A clone of the state that was just reached
        reached_state : TuringState,
        /// The index of the currently reached state
        state_pointer: usize,
        /// The index of the transition taken from the current state to the next one.
        transition_index_taken : usize,
        /// A clone of the transition that was just taken
        transition_taken : TuringTransitionMultRibbons,
        /// A clone representing the current state of the reading ribbon after taking that transition.
        read_ribbon: TuringReadRibbon,
        /// A clone representing the current state of the writting ribbons after taking that transition.
        write_ribbons: Vec<TuringWriteRibbon>,
        /// The current number of iterations already done
        iteration : usize,
    },
    Backtracked 
    {
        /// A clone of the state that was just left
        previous_state : TuringState,
        /// A clone of the state that was backtracked to
        reached_state : TuringState,
        /// The index of the currently reached state
        state_pointer: usize,
        /// A clone representing the current state of the reading ribbon after backtracking.
        read_ribbon: TuringReadRibbon,
        /// A clone representing the current state of the writting ribbons after backtracking.
        write_ribbons: Vec<TuringWriteRibbon>,
        /// The current number of iterations already done
        iteration : usize,
    }
}



impl<'a> Iterator for TuringMachines
{
    type Item = TuringExecutionSteps;

    fn next(&mut self) -> Option<Self::Item> 
    {
        let prev_iter = self.get_iteration();
        
        if let Mode::StopAfter(nb) = self.get_mode() {
            if *nb == prev_iter {
                return None;
            }
        }

        // Increment nb of iterations already treated
        self.set_iteration(prev_iter + 1);

        // Fetch the current state
        let curr_state =  self.graph_ref().get_state(self.get_state_pointer()).unwrap().clone();

        let mut transition_index_taken = None;

        // If this iteration is a follow up to a backtracking
        // we simply take the index found at the previous iteration
        if let Some(bracktrack_transition_index) = self.get_backtracking_info() {
            self.set_backtracking_info(None);
            transition_index_taken = Some(bracktrack_transition_index)
        }
        else{
            if self.is_first_iteration() {
                self.set_first_iteration(false);

                return Some(TuringExecutionSteps::FirstIteration { init_state: curr_state, 
                    init_read_ribbon: self.get_reading_ribbon().clone(), 
                    init_write_ribbons: self.get_writting_ribbons().clone() });
            }

            /* Checks if the state is accepting */
            if let TuringStateType::Accepting = curr_state.state_type
            {
                // The iteration is over
                return None;
            }

            // if it's normal or rejecting

            // If one of the transition condition is true,
            // Get all current char read by **all** ribbons
            let mut char_vec = vec!(self.get_reading_ribbon().read_curr_char().clone());
            for ribbon in self.get_writting_ribbons() {
                char_vec.push(ribbon.read_curr_char());
            }
            
            let mut next_transitions = VecDeque::from(curr_state.get_valid_transitions_indexes(&char_vec));
        
            // If no transitions can be provided or the current state is rejecting,
            // we reached a *dead end*, go back in the exploration if possible
            if next_transitions.is_empty() || curr_state.state_type == TuringStateType::Rejecting
            {
                if let Mode::StopFirstReject = self.get_mode() {
                    return None;
                }
                // If there are no saved state, this means the backtracking is over, and the execution too
                if self.get_memory_mut().is_empty() {
                    return None;
                }

                // While the memory still has a state saved
                while !self.get_memory_mut().is_empty() {
                    {
                        let saved_state = self.get_memory_mut().front_mut().unwrap();
                        
                        // Get the next transition to take
                        if let Some(t_i) = saved_state.next_transitions.pop_front() {
                            transition_index_taken = Some(t_i);
                        }
                        else {
                            // If no transition is left to take for this state, we move on to the next one and remove it
                            self.get_memory_mut().pop_front();
                            continue;
                        }
                    }
                    // obliged to clone because of the mutable nature
                    let saved_state = self.get_memory_mut().front().unwrap().clone();
                    
                    // Go back to the state
                    self.set_state_pointer(saved_state.saved_state_index);
                    
                    // Change the context for the reading and writing ribbons
                    self.set_reading_ribbon(saved_state.saved_read_ribbon);
                    self.set_writting_ribbons(saved_state.saved_write_ribbons);
                    // Save the index of the transition found for the next call to `.next()`
                    self.set_backtracking_info(transition_index_taken);

                    // Return backtracking info
                    return Some(TuringExecutionSteps::Backtracked { 
                        previous_state: curr_state, 
                        reached_state: self.graph_ref().get_state(saved_state.saved_state_index).unwrap().clone(),
                        read_ribbon: self.get_reading_ribbon().clone(),
                        write_ribbons: self.get_writting_ribbons().clone(),
                        iteration : self.get_iteration(),
                        state_pointer: self.get_state_pointer() });
                }
            }
        
            // If there are more than 1 transition possible at a time, it means we are in a non deterministic situation.
            // We must save the current state in order to explore all path.
            else if next_transitions.len() >= 2 {
                // take the first transition, save the rest
                transition_index_taken = Some(next_transitions.pop_front().unwrap());

                let to_save = SavedState { saved_state_index:self.get_state_pointer(), 
                                                        next_transitions: next_transitions, 
                                                        saved_read_ribbon: self.get_reading_ribbon().clone(), 
                                                        saved_write_ribbons: self.get_writting_ribbons().clone() };

                self.push_to_memory_stack(to_save);
            }
            else if next_transitions.len() == 1 {
                transition_index_taken = Some(next_transitions[0]);
            }
        }
        // if a viable transition was found
        if let Some(ind) = transition_index_taken {
            let transition = self.graph_ref().get_state(self.get_state_pointer()).unwrap().transitions[ind as usize].clone();
            // Apply the transition
            // to the read ribbons
            self.get_reading_ribbon().try_apply_transition(transition.chars_read[0], ' ', &transition.move_read).unwrap();
            
            // to the write ribbons
            for i in 0..self.graph_ref().get_k()
            {
                self.get_writting_ribbons()[i as usize].try_apply_transition(transition.chars_read[(i+1) as usize],
                                                                                        transition.chars_write[i as usize].0, &transition.chars_write[i as usize].1).unwrap();
            }
    
            // Move to the next state
            self.set_state_pointer(transition.index_to_state.unwrap());
            
            Some(TuringExecutionSteps::TransitionTaken
            {
                previous_state: curr_state.clone(),
                reached_state: self.graph_ref().get_state(self.get_state_pointer()).unwrap().clone(),
                transition_index_taken : ind as usize,
                transition_taken: transition.clone(),
                read_ribbon: self.get_reading_ribbon().clone(),
                write_ribbons: self.get_writting_ribbons().clone(),
                iteration: self.get_iteration(),
                state_pointer: self.get_state_pointer()
            })
            
        }
        // otherwise it's also the end
        else {
            None
        }
    }
}


impl<'a> Display for TuringExecutionSteps{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self {
            TuringExecutionSteps::FirstIteration { init_state, init_read_ribbon, init_write_ribbons } => {
                let mut write_str_rib = String::from(format!("{}", init_write_ribbons[0]));
                for i in 1..init_write_ribbons.len() 
                {
                    write_str_rib.push_str(format!("\n{}", init_write_ribbons[i]).as_str());
                }

                write!(f, "* Initial state : {}\n* Ribbons:\nREAD:\n{}\nWRITE:\n{}", init_state, init_read_ribbon, write_str_rib)
            },
            TuringExecutionSteps::TransitionTaken { previous_state, reached_state, transition_index_taken:_, transition_taken, read_ribbon, write_ribbons, iteration:_, state_pointer:_ } => {
                    let mut write_str_rib = String::from(format!("{}", write_ribbons[0]));
                    for i in 1..write_ribbons.len() 
                    {
                        write_str_rib.push_str(format!("\n{}", write_ribbons[i]).as_str());
                    }

                    write!(f, "* Left state : {}\n* Current state : {}\n* Took the following transition : {}\n* Ribbons:\nREAD:\n{}\nWRITE:\n{}", previous_state, reached_state, transition_taken, read_ribbon, write_str_rib)
            },
            TuringExecutionSteps::Backtracked { previous_state, reached_state, read_ribbon, write_ribbons, iteration:_ , state_pointer:_} => {
                let mut write_str_rib = String::from(format!("{}", write_ribbons[0]));
                for i in 1..write_ribbons.len() 
                {
                    write_str_rib.push_str(format!("\n{}", write_ribbons[i]).as_str());
                }

                write!(f, "* Backtracked from : {}\n* To  : {}\n* Ribbons:\nREAD:\n{}\nWRITE:\n{}", previous_state, reached_state, read_ribbon, write_str_rib)
            },
        }
        
    }
}


impl TuringExecutionSteps {
    pub fn get_current_state(&self) -> &TuringState
    {
        match self {
            TuringExecutionSteps::FirstIteration { init_state, init_read_ribbon:_, init_write_ribbons:_ } => init_state,
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state, state_pointer:_, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_, iteration:_ } => reached_state,
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state, state_pointer:_, read_ribbon:_, write_ribbons:_, iteration:_ } => reached_state,
        }
    }

    pub fn get_previous_state(&self) -> Option<&TuringState>
    {
        match self {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons:_ } => None,
            TuringExecutionSteps::TransitionTaken { previous_state, reached_state:_, state_pointer:_, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_, iteration:_ } => Some(previous_state),
            TuringExecutionSteps::Backtracked { previous_state, reached_state:_, state_pointer:_, read_ribbon:_, write_ribbons:_, iteration:_ } => Some(previous_state),
        }
    }

    pub fn get_nb_iterations(&self) -> usize
    {
        match self {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons:_ } => 0,
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state:_, state_pointer:_, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_, iteration } => *iteration,
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, state_pointer:_, read_ribbon:_, write_ribbons:_, iteration } => *iteration,
        }
    }


    pub fn get_state_pointer(&self) -> usize
    {
        match self {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons:_ } => 0,
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state:_, state_pointer, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_, iteration :_} => *state_pointer,
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, state_pointer, read_ribbon:_, write_ribbons:_, iteration:_ } => *state_pointer,
        }
    }

    pub fn get_reading_ribbon(&self) -> &TuringReadRibbon {
        match self {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon, init_write_ribbons:_ } => init_read_ribbon,
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state:_, state_pointer:_, transition_index_taken:_, transition_taken:_, read_ribbon, write_ribbons:_, iteration:_ } => read_ribbon,
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, state_pointer:_, read_ribbon, write_ribbons:_, iteration:_ } => read_ribbon,
        }
    }


    pub fn get_writting_ribbons(&self) -> &Vec<TuringWriteRibbon> {
        match self {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons } => init_write_ribbons,
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state:_, state_pointer:_, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons, iteration:_ } => write_ribbons,
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, state_pointer:_, read_ribbon:_, write_ribbons, iteration:_ } => write_ribbons,
        }
    }


}