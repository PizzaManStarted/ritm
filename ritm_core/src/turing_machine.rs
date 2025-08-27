use std::{collections::VecDeque, fmt::{Debug, Display}};


use crate::{turing_errors::TuringError, turing_graph::TuringMachineGraph, turing_ribbon::{TuringReadRibbon, TuringRibbon, TuringWriteRibbon}, turing_state::{TuringState, TuringStateType, TuringTransitionMultRibbons}};


#[derive(Clone, Debug)]
/// Represents the different mode a turing machine can have during it's execution
#[derive(PartialEq)]
pub enum Mode {
    /// Explores all possible paths (and possibilities using backtracking) until an accepting state is found or no path is left is to take. 
    SaveAll, // May god bless your ram
    /// Stops after the specified amount of iteration is reached even if the execution is not over.
    StopAfter(usize),
    /// Stops after meeting the first rejecting state or when the execution is blocked, even if backtracking is possible 
    StopFirstReject,
}


impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Mode::SaveAll => "Saves All and does a full exploration".to_string(),
            Mode::StopAfter(val) => format!("Stops After {} iterations", val),
            Mode::StopFirstReject => "Stops after the First Reject".to_string(),
        })
    }
}


#[derive(Debug, Clone)]
pub struct SavedState {
    /// The index of the saved state
    pub saved_state_index : usize,
    /// A stack containing all the indexes of the transitions left to take 
    pub next_transitions : VecDeque<usize>,
    /// The value of the [TuringReadRibbon] when it was saved
    pub saved_read_ribbon : TuringReadRibbon,
    /// The value of the [TuringWriteRibbon] when they were saved
    pub saved_write_ribbons : Vec<TuringWriteRibbon>,
    /// The value of the iteration that was saved.
    pub iteration: usize,
}


#[derive(Debug)]
pub enum TuringMachines
{
    TuringMachine {
        /// The turing machine graph that will execute a word
        graph : TuringMachineGraph,
        data : IterationData,
        /// The current number of iterations already done
        iteration : usize,
        /// Copy of the iteration step returned (if any).
        last_iteration : Option<TuringExecutionSteps>,
        /// Checks wether or not the iteration is over or not
        is_over : bool
    }
}

#[derive(Debug)]
pub struct IterationData {
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
            iteration : 0,
            last_iteration: None,
            is_over: false
        };
        // Add the word to the reading ribbon
        if let Err(e) = s.get_reading_ribbon_mut().feed_word(word) {
            Err(e)
        }
        else {
            Ok(s)
        }
    }

    /// Adds a new [SavedState] to the front of the memory stack.
    fn push_to_memory_stack(&mut self, to_save: SavedState)
    {
        self.get_memory_mut().push_front(to_save);
    }

    /// Resets the turing machine to its initial state and re-feeds it the current stored word.
    pub fn reset(&mut self)
    {
        let word = self.get_word().clone();
        self.reset_word(&word).unwrap();
    }

    /// Resets the turing machine to its initial state and feeds it the given word.
    pub fn reset_word(&mut self, word: &String) -> Result<(), TuringError>
    {
        // Reset reading ribbon
        self.get_reading_ribbon_mut().feed_word(word.clone())?;
        
        self.set_word(word);


        // Reset write ribbons
        for i in 0..self.get_writting_ribbons_mut().len() {
            self.get_writting_ribbons_mut()[i] = TuringWriteRibbon::new();
        }

        // Reset state pointers
        self.set_state_pointer(0);

        // Reset first iteration
        self.set_first_iteration(true);

        // Sets the number of iterations to 0
        self.set_iteration(0);

        // Reset backtracking info
        self.set_backtracking_info(None);

        self.set_last_step(None);

        self.set_is_over(false);

        // And clear memory
        self.get_memory_mut().clear();
        
        Ok(())
    }
    /// Changes the current execution mode of the turing machine.
    pub fn set_mode(&mut self, mode: &Mode) 
    {
        // Change mode
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_ , is_over:_} => data.mode = mode.clone(),
        }
    }

    /// Gets the path to the accepting state if any exists. 
    /// In other terms, it will only store the steps that will lead to an accepting path without any backtracking.
    /// Always resets the execution before starting (but doesn't reset after ending). 
    /// ## Returns
    /// [None] if no path to an accepting state is found.
    /// [Some] containing a [Vec] of [TuringExecutionSteps] leading to the accepting state.
    /// 
    /// 
    /// ## Infinite iterations problems
    /// **Beware** that this function will loop forever **if** the related turing machine graph loops for the given input.
    /// In order to prevent this, it is possible to supply a function that will be called before every iteration to check if it is allowed to continue it's execution.
    /// Another mitigation would be to simply change the execution mode of this turing machine. 
    pub fn get_path_to_accept<F>(&mut self, mut exit_condition: F) 
        -> Option<Vec<TuringExecutionSteps>> where F: FnMut() -> bool
    {
        self.reset();
        let mut path = Vec::<TuringExecutionSteps>::new();
        let mut last_step_type = None;
        for step in &mut *self {
            if !exit_condition() {
                return None;
            }
            last_step_type = Some(step.get_current_state().state_type.clone());
            match &step {
                TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons:_ } => {
                    path.push(step);
                },
                TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state:_, state_pointer:_, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_, iteration:_ } => {
                    path.push(step);
                },
                TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, state_pointer:_, read_ribbon:_, write_ribbons:_, iteration:_, backtracked_iteration } => {
                    // Pop stack until we find the iteration we backtracked to
                    while path.last().unwrap().get_nb_iterations() != *backtracked_iteration {
                        path.pop();
                    }
                },
            }
        }
        // If the last step did not result in an accepting state, 
        // then we know that no path results in an accepting state.
        if let Some(t) = last_step_type
            && TuringStateType::Accepting != t {
                return None;
            }

        Some(path)
    }
}


impl TuringMachines {
    /// Gets *reference* of the stored turing machine graph.
    pub fn graph_ref(&self) -> &TuringMachineGraph {
        match self {
            TuringMachines::TuringMachine { graph, data:_, iteration:_, last_iteration:_, is_over:_ } => graph,
        }
    }

    /// Gets *mutable reference* of the stored turing machine graph.
    pub fn graph_mut(&mut self) -> &mut TuringMachineGraph {
        match self {
            TuringMachines::TuringMachine { graph, data:_, iteration:_, last_iteration:_, is_over:_ } => graph,
        }
    }

    /// Gets the stored turing machine graph.
    /// 
    /// This will free the turing machine since it will drop the ownership
    pub fn graph(self) -> TuringMachineGraph {
        match self {
            TuringMachines::TuringMachine { graph, data:_, iteration:_, last_iteration:_, is_over:_ } => graph,
        }
    }

    
    
    /// Gets the current state pointer of this struct.
    pub fn get_state_pointer(&self) -> usize {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_, is_over:_ } => data.state_pointer,
        }
    }
    
    /// Sets a new value to the state pointer.
    fn set_state_pointer(&mut self, new_val: usize) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_, is_over:_ } => data.state_pointer = new_val,
        }
    }
    
    /// Gets mutable ref to the reading ribbon stored inside this struct.
    fn get_reading_ribbon_mut(&mut self) -> &mut TuringReadRibbon {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_, is_over:_ } => &mut data.reading_ribbon,
        }
    }

    /// Gets ref to the reading ribbon stored inside this struct.
    pub fn get_reading_ribbon(&self) -> &TuringReadRibbon {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_, is_over:_ } => &data.reading_ribbon,
        }
    }

    /// Sets the reading ribbon stored inside this struct.
    fn set_reading_ribbon(&mut self, ribbon: TuringReadRibbon) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_ , is_over:_} => data.reading_ribbon = ribbon,
        }
    }
    

    /// Gets the mut ref writtings ribbons stored inside this struct.
    fn get_writting_ribbons_mut(&mut self) -> &mut Vec<TuringWriteRibbon> {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_, is_over:_ } => &mut data.write_ribbons,
        }
    }

    /// Gets the reference to the writtings ribbons stored inside this struct.
    pub fn get_writting_ribbons(&self) -> &Vec<TuringWriteRibbon> {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_, is_over:_ } => &data.write_ribbons,
        }
    }
    

    /// Sets the writting ribbons stored inside this struct.
    fn set_writting_ribbons(&mut self, ribbons: Vec<TuringWriteRibbon>) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_, is_over:_ } => data.write_ribbons = ribbons,
        }
    }
    
    /// Gets the word that was feed to this machine.
    pub fn get_word(&self) -> &String {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_, is_over:_ } => &data.word,
        }
    }

    /// Gets the word that was feed to this machine.
    fn set_word(&mut self, word: &String) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_ , is_over:_} => data.word = word.to_string(),
        }
    }

    /// Checks if the current iteration is the first iteration or not.
    fn is_first_iteration(&mut self) -> bool {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_ , is_over:_} => data.is_first_state,
        }
    }
    
    /// Sets the state of this turing machine to be considered or not its first iteration.
    fn set_first_iteration(&mut self, set: bool) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_, is_over:_ } => data.is_first_state = set,
        }
    }
    
    /// Fetches the mode of the iterator.
    pub fn get_mode(&self) -> &Mode {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_ , is_over:_} => &data.mode,
        }
    }
    
    /// Get the **mutable** stack containing all the [SavedState].
    fn get_memory_mut(&mut self) -> &mut VecDeque<SavedState> {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_ , is_over:_} => &mut data.memory,
        }
    }
    
    /// Get the reference to the stack containing all the [SavedState].
    pub fn get_memory(&self) -> &VecDeque<SavedState> {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_, is_over:_ } => &data.memory,
        }
    }

    fn get_backtracking_info(&self) -> Option<usize> {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_ , is_over:_} => data.backtracked_info,
        }
    }

    fn set_backtracking_info(&mut self, val: Option<usize>) {
        match self {
            TuringMachines::TuringMachine { graph:_, data, iteration:_, last_iteration:_ , is_over:_} => data.backtracked_info = val,
        }
    }

    fn set_iteration(&mut self, val: usize) {
        match self {
            TuringMachines::TuringMachine { graph:_, data:_, iteration, last_iteration:_ , is_over:_} => *iteration = val,
        }
    }

    pub fn get_iteration(&self) -> usize {
        match self {
            TuringMachines::TuringMachine { graph:_, data:_, iteration , last_iteration:_, is_over:_} => *iteration,
        }
    }


    /// Returns the last step that was returned.
    pub fn get_last_step(&self) -> &Option<TuringExecutionSteps>
    {
        match self {
            TuringMachines::TuringMachine { graph:_, data:_, iteration:_, last_iteration, is_over:_ } => last_iteration,
        }
    }

    fn set_last_step(&mut self, step: Option<TuringExecutionSteps>) 
    {
        match self {
            TuringMachines::TuringMachine { graph:_, data:_, iteration:_, last_iteration, is_over:_ } => *last_iteration = step
        }
    }

    /// Checks if the iteration is over or not
    pub fn is_over(&self) -> bool
    {
        match self {
            TuringMachines::TuringMachine { graph:_, data:_, iteration:_, last_iteration:_ , is_over} => { *is_over},
        }
    }


    fn set_is_over(&mut self, val: bool)
    {
        match self {
            TuringMachines::TuringMachine { graph:_, data:_, iteration:_, last_iteration:_, is_over } => *is_over = val,
        }
    }

}

#[derive(Clone, Debug)]
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
        /// The number of the iteration that was bactracked to
        backtracked_iteration: usize
    }
}



impl Iterator for &mut TuringMachines
{
    type Item = TuringExecutionSteps;

    fn next(&mut self) -> Option<Self::Item>
    {
        // Get next step
        let next_step = next_iteration(self);
        if let Some(step) = next_step {
            // Save & return it
            self.set_last_step(Some(step.clone()));

            Some(step)
        }
        else {
            self.set_is_over(true);
            None
        }
    }
}

fn next_iteration(tm: &mut TuringMachines) -> Option<TuringExecutionSteps>
{
    let prev_iter = tm.get_iteration();
    
    if let Mode::StopAfter(nb) = tm.get_mode()
        && *nb == prev_iter {
            return None;
        }

    // Increment nb of iterations already treated
    tm.set_iteration(prev_iter + 1);

    // Fetch the current state
    let curr_state =  tm.graph_ref().get_state(tm.get_state_pointer()).unwrap().clone();

    let mut transition_index_taken = None;

    // If this iteration is a follow up to a backtracking
    // we simply take the index found at the previous iteration
    if let Some(bracktrack_transition_index) = tm.get_backtracking_info() {
        tm.set_backtracking_info(None);
        transition_index_taken = Some(bracktrack_transition_index)
    }
    else{
        if tm.is_first_iteration() {
            tm.set_first_iteration(false);

            return Some(TuringExecutionSteps::FirstIteration { init_state: curr_state, 
                init_read_ribbon: tm.get_reading_ribbon_mut().clone(), 
                init_write_ribbons: tm.get_writting_ribbons_mut().clone() });
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
        let mut char_vec = vec!(tm.get_reading_ribbon_mut().read_curr_char());
        for ribbon in tm.get_writting_ribbons_mut() {
            char_vec.push(ribbon.read_curr_char());
        }
        
        let mut next_transitions = VecDeque::from(curr_state.get_valid_transitions_indexes(&char_vec));
    
        // If no transitions can be provided or the current state is rejecting,
        // we reached a *dead end*, go back in the exploration if possible
        if next_transitions.is_empty() || curr_state.state_type == TuringStateType::Rejecting
        {
            if let Mode::StopFirstReject = tm.get_mode() {
                return None;
            }
            // If there are no saved state, this means the backtracking is over, and the execution too
            if tm.get_memory_mut().is_empty() {
                return None;
            }

            // While the memory still has a state saved
            while !tm.get_memory_mut().is_empty() {
                {
                    let saved_state = tm.get_memory_mut().front_mut().unwrap();
                    
                    // Get the next transition to take
                    if let Some(t_i) = saved_state.next_transitions.pop_front() {
                        transition_index_taken = Some(t_i);
                    }
                    else {
                        // If no transition is left to take for this state, we move on to the next one and remove it
                        tm.get_memory_mut().pop_front();
                        continue;
                    }
                }
                // obliged to clone because of the mutable nature
                let saved_state = tm.get_memory_mut().front().unwrap().clone();
                
                // Go back to the state
                tm.set_state_pointer(saved_state.saved_state_index);
                
                // Change the context for the reading and writing ribbons
                tm.set_reading_ribbon(saved_state.saved_read_ribbon);
                tm.set_writting_ribbons(saved_state.saved_write_ribbons);
                // Save the index of the transition found for the next call to `.next()`
                tm.set_backtracking_info(transition_index_taken);

                // If the saved state has no more transitions, it can already be removed
                if saved_state.next_transitions.is_empty() {
                    tm.get_memory_mut().pop_front();
                }

                // Return backtracking info
                return Some(TuringExecutionSteps::Backtracked { 
                    previous_state: curr_state, 
                    reached_state: tm.graph_ref().get_state(saved_state.saved_state_index).unwrap().clone(),
                    read_ribbon: tm.get_reading_ribbon_mut().clone(),
                    write_ribbons: tm.get_writting_ribbons_mut().clone(),
                    iteration : prev_iter,
                    state_pointer: tm.get_state_pointer(),
                    backtracked_iteration: saved_state.iteration });
            }
        }
    
        // If there are more than 1 transition possible at a time, it means we are in a non deterministic situation.
        // We must save the current state in order to explore all path.
        else if next_transitions.len() >= 2 {
            // take the first transition, save the rest
            transition_index_taken = Some(next_transitions.pop_front().unwrap());

            let to_save = SavedState { saved_state_index:tm.get_state_pointer(), 
                                                    next_transitions, 
                                                    saved_read_ribbon: tm.get_reading_ribbon_mut().clone(), 
                                                    saved_write_ribbons: tm.get_writting_ribbons_mut().clone(),
                                                    iteration: prev_iter - 1 };

            tm.push_to_memory_stack(to_save);
        }
        else if next_transitions.len() == 1 {
            transition_index_taken = Some(next_transitions[0]);
        }
    }
    // if a viable transition was found
    if let Some(ind) = transition_index_taken {
        let transition = tm.graph_ref().get_state(tm.get_state_pointer()).unwrap().transitions[ind].clone();
        // Apply the transition
        // to the read ribbons
        tm.get_reading_ribbon_mut().try_apply_transition(transition.chars_read[0], ' ', &transition.move_read).unwrap();
        
        // to the write ribbons
        for i in 0..tm.graph_ref().get_k()
        {
            tm.get_writting_ribbons_mut()[i].try_apply_transition(transition.chars_read[i+1],
                                                                                    transition.chars_write[i].0, &transition.chars_write[i].1).unwrap();
        }

        // Move to the next state
        tm.set_state_pointer(transition.index_to_state.unwrap());
        
        Some(TuringExecutionSteps::TransitionTaken
        {
            previous_state: curr_state.clone(),
            reached_state: tm.graph_ref().get_state(tm.get_state_pointer()).unwrap().clone(),
            transition_index_taken : ind,
            transition_taken: transition.clone(),
            read_ribbon: tm.get_reading_ribbon_mut().clone(),
            write_ribbons: tm.get_writting_ribbons_mut().clone(),
            iteration: prev_iter,
            state_pointer: tm.get_state_pointer()
        })
        
    }
    // otherwise it's also the end
    else {
        None
    }
}


impl Display for TuringExecutionSteps{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self {
            TuringExecutionSteps::FirstIteration { init_state, init_read_ribbon, init_write_ribbons } => {
                let mut write_str_rib = init_write_ribbons[0].to_string();
                for write_ribbon in init_write_ribbons.iter().skip(1) {
                    write_str_rib.push_str(format!("\n{}", write_ribbon).as_str());
                }

                write!(f, "* Initial state : {}\n* Ribbons:\nREAD:\n{}\nWRITE:\n{}", init_state, init_read_ribbon, write_str_rib)
            },
            TuringExecutionSteps::TransitionTaken { previous_state, reached_state, transition_index_taken:_, transition_taken, read_ribbon, write_ribbons, iteration:_, state_pointer:_ } => {
                    let mut write_str_rib = write_ribbons[0].to_string();
                    for write_ribbon in write_ribbons.iter().skip(1) {
                        write_str_rib.push_str(format!("\n{}", write_ribbon).as_str());
                    }
                    write!(f, "* Left state : {}\n* Current state : {}\n* Took the following transition : {}\n* Ribbons:\nREAD:\n{}\nWRITE:\n{}", previous_state, reached_state, transition_taken, read_ribbon, write_str_rib)
            },
            TuringExecutionSteps::Backtracked { previous_state, reached_state, read_ribbon, write_ribbons, iteration:_ , state_pointer:_, backtracked_iteration} => {
                let mut write_str_rib = write_ribbons[0].to_string();
                for write_ribbon in write_ribbons.iter().skip(1) {
                    write_str_rib.push_str(format!("\n{}", write_ribbon).as_str());
                }

                write!(f, "* Backtracked from : {}\n* To  : {}(back to iteration: {})\n* Ribbons:\nREAD:\n{}\nWRITE:\n{}", previous_state, reached_state, backtracked_iteration, read_ribbon, write_str_rib)
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
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state, state_pointer:_, read_ribbon:_, write_ribbons:_, iteration:_, backtracked_iteration:_ } => reached_state,
        }
    }

    pub fn get_previous_state(&self) -> Option<&TuringState>
    {
        match self {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons:_ } => None,
            TuringExecutionSteps::TransitionTaken { previous_state, reached_state:_, state_pointer:_, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_, iteration:_ } => Some(previous_state),
            TuringExecutionSteps::Backtracked { previous_state, reached_state:_, state_pointer:_, read_ribbon:_, write_ribbons:_, iteration:_, backtracked_iteration:_ } => Some(previous_state),
        }
    }

    pub fn get_nb_iterations(&self) -> usize
    {
        match self {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons:_ } => 0,
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state:_, state_pointer:_, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_, iteration } => *iteration,
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, state_pointer:_, read_ribbon:_, write_ribbons:_, iteration, backtracked_iteration:_ } => *iteration,
        }
    }


    pub fn get_state_pointer(&self) -> usize
    {
        match self {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons:_ } => 0,
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state:_, state_pointer, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_, iteration :_} => *state_pointer,
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, state_pointer, read_ribbon:_, write_ribbons:_, iteration:_, backtracked_iteration:_ } => *state_pointer,
        }
    }

    pub fn get_reading_ribbon(&self) -> &TuringReadRibbon {
        match self {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon, init_write_ribbons:_ } => init_read_ribbon,
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state:_, state_pointer:_, transition_index_taken:_, transition_taken:_, read_ribbon, write_ribbons:_, iteration:_ } => read_ribbon,
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, state_pointer:_, read_ribbon, write_ribbons:_, iteration:_, backtracked_iteration:_ } => read_ribbon,
        }
    }


    pub fn get_writting_ribbons(&self) -> &Vec<TuringWriteRibbon> {
        match self {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons } => init_write_ribbons,
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state:_, state_pointer:_, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons, iteration:_ } => write_ribbons,
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, state_pointer:_, read_ribbon:_, write_ribbons, iteration:_, backtracked_iteration:_ } => write_ribbons,
        }
    }
}