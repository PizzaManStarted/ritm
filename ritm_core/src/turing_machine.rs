use std::{collections::HashMap, f32::consts::E, fmt::{Debug, Display}, usize};
use rand::{rng, Rng};
use crate::{turing_errors::TuringError, turing_ribbon::{TuringReadRibbon, TuringRibbon, TuringWriteRibbon}, turing_state::{TuringState, TuringTransition}};


/// A struct representing a Turing Machine graph with k rubbons.
pub struct TuringMachineGraph
{
    /// The hashmap containing a mapping of all nodes names and their related index in the `states` field.
    pub name_index_hashmap: HashMap<String, u8>, 
    /// The vector containing all the nodes of the turing machine graph
    states: Vec<TuringState>,
    /// The number of ribbons this graph was made for
    k: u8,
}




impl TuringMachineGraph {
    /// Creates a new empty Turing Machine graph that has `k` writting rubbons.
    /// 
    /// Three default states will be created :
    /// * `q_i` : The initial state
    /// * `q_a` : The default accepting state
    /// * `q_r` : The default rejecting state
    pub fn new(k: u8) -> Result<Self, TuringError>
    {
        if k == 0 {
            return Err(TuringError::IllegalActionError { cause: "Tried to create a turing machine graph with 0 ribbon".to_string() });
        }
        // Add the default states
        let init_state = TuringState::new(false).set_name("i");
        let accepting_state = TuringState::new(true).set_name("a");
        let rejecting_state = TuringState::new(true).set_name("r");
        
        // Create the hash map with the already known states
        let mut name_index_hashmap: HashMap<String, u8> = HashMap::new();
        name_index_hashmap.insert("i".to_string(), 0);    // init
        name_index_hashmap.insert("a".to_string(), 1);    // accepting
        name_index_hashmap.insert("r".to_string(), 2);    // rejecting

        Ok(Self
        {
            name_index_hashmap,
            states: vec!(init_state, accepting_state, rejecting_state),
            k,
        })
    }



    /// Adds a new rule to a state of the machine of the form : `from {transition} to`.
    /// Meaning, a new edge is added to the graph.
    /// 
    /// If one of the given state didn't already exists, a new one with that name will be created.
    pub fn append_rule_state_by_name(&mut self, from: String, transition: TuringTransition, to: String) -> Result<(), TuringError>
    {
        // Checks if the given number of ribbons is correct
        if transition.get_number_of_affected_ribbons() != self.k as usize
        {
            return Err(TuringError::ArgsSizeTransitionError);
        }
        let from_index = self.add_state(&from);
        let to_index = self.add_state(&to);

        match self.add_rule_state_ind(from_index, transition, to_index) {
            Ok(()) => {
                return Ok(());
            },
            Err(e) => {
                return Err(e);
            },
        };
    }

    /// Adds a new rule to a state of the machine of the form : `from {transition} to`.
    /// Meaning, a new edge is added to the graph.
    /// 
    /// ## Returns
    /// * If everything went correctly : `Ok(())`
    /// * Otherwise, it will return a [TuringError]
    pub fn append_rule_state(&mut self, from_index: u8, transition: TuringTransition, to_index: u8) -> Result<(), TuringError>
    {
        // Checks if the given correct of number transitions was given
        if transition.chars_write.len() != self.k as usize
        {
            return Err(TuringError::ArgsSizeTransitionError);
        }

        match self.add_rule_state_ind(from_index, transition, to_index) {
            Ok(()) => {
                return Ok(());
            },
            Err(e) => {
                return Err(e);
            },
        };

    }

    /// Adds a new rule to a state of the machine and returns the machine.
    /// 
    /// If the given state didn't already exists, the state will be created.
    pub fn append_rule_state_self(mut self, from: String, transition: TuringTransition, to: String) -> Result<Self, TuringError>
    {
        match self.append_rule_state_by_name(from, transition, to) {
            Ok(()) => return Ok(self),
            Err(e) => return Err(e),
        };
    }

    /// Adds a new state to the turing machine graph and returns its index. Meaning a new node is added to the graph.
    /// 
    /// If the state name already existed then the index of the already existing state is returned.
    pub fn add_state(&mut self, name: &String) -> u8
    {
        // Try to find the index of the state inside the hashmap 
        match self.name_index_hashmap.get(name) 
        {
            // If the index was found, return it
            Some(e) => {
                return *e;
            },
            // If not
            None => 
            {
                // Pushes in the vector of states a new state with the given name 
                self.states.push(TuringState::new(false).set_name(name));
                // Adds the index of this state to the hashmap 
                self.name_index_hashmap.insert(name.to_string(), (self.states.len()-1) as u8);
                // Returns the index of the newly created state
                return (self.states.len()-1) as u8;
            },
        }
    }

    /// Adds a new state to the turing machine graph using variables indexes
    fn add_rule_state_ind(&mut self, from: u8, mut transition: TuringTransition, to: u8) -> Result<(), TuringError>
    {
        if self.states.len() <= from as usize {
            return Err(TuringError::OutOfRangeStateError { accessed_index: from as usize, states_len: self.states.len() });
        }
        if self.states.len() <= to as usize {
            return Err(TuringError::OutOfRangeStateError { accessed_index: to as usize, states_len: self.states.len() });
        }
        // Change transition index
        transition.index_to_state = to;

        let state  = self.states.get_mut(from as usize).unwrap();
        state.add_transition(transition);
        return Ok(());
    }

    /// Returns the state (*node*) at the given index.
    pub fn get_state(&self, pointer: u8) -> Result<&TuringState, TuringError>
    {
        if self.states.len() <= pointer as usize {
            return Err(TuringError::OutOfRangeStateError { accessed_index: pointer as usize, states_len: self.states.len() });
        }
        Ok(&self.states[pointer as usize])
    }

    /// Returns the state (*node*) that has the given name.
    pub fn get_state_from_name(&self, name: &String) -> Result<&TuringState, TuringError>
    {
        match self.name_index_hashmap.get(name) {
            Some(index) => self.get_state(*index),
            None => Err(TuringError::UnknownStateError { state_name: name.to_string() }),
        }
    }

    /// Get the transition index between two nodes if it exists.
    /// Returns the first one found.
    pub fn get_transition_index_by_name(&self, n1: &String, n2: &String) -> Option<usize>
    {
        // Get n1 and n2 indexes if they exists
        let n1_state = match self.name_index_hashmap.get(n1) 
        {
            Some(i) => &self.states[*i as usize],
            None => return None,
        };
        let n2_index = match self.name_index_hashmap.get(n2) 
        {
            Some(i) => *i,
            None => return None,
        };

        for (i, t) in n1_state.transitions.iter().enumerate() {
            if t.index_to_state == n2_index{
                return Some(i);
            }
        }
        
        return None;
    }

    /// Get all the transitions indexes between two nodes.
    pub fn get_transition_index(&self, n1: u8, n2: u8) -> Result<Vec<usize>, TuringError>
    {
        let mut vec = vec!();
        // Get n1 index
        let n1_state = self.get_state(n1);
        if let Err(e) = n1_state {
            return Err(e);
        }
        let n1_state = n1_state.unwrap();

        // Fetch all transition that go toward n2
        for (i, t) in n1_state.transitions.iter().enumerate() {
            if t.index_to_state == n2{
                vec.push(i);
            }
        }
        
        return Ok(vec);
    }

    /// Removes all the transitions from this state to the given node
    pub fn remove_transitions(&mut self, from: &String, to: &String) -> Result<(), TuringError>
    {
        // Fetch n1 as a state
        let n1_state = self.name_index_hashmap.get(from);
        if let None = n1_state {
            return Err(TuringError::UnknownStateError { state_name: from.to_string() });
        }

        let n1_state = &mut self.states[*n1_state.unwrap() as usize];

        // Fetch n2 as an index
        let n2_state = self.name_index_hashmap.get(to);
        if let None = n2_state {
            return Err(TuringError::UnknownStateError { state_name: to.to_string() });
        }
        let n2_index = n2_state.unwrap();


        // Remove all transitions from n1 to n2
        n1_state.remove_transitions(*n2_index);
        Ok(())
    }

    /// Removes a state and **all** mentions of it in **all** transitions of **all** the other states of the TuringMachine
    pub fn remove_state(&mut self, state_name: &String) -> Result<(), TuringError>
    {
        // First keep the index for later
        let index = self.name_index_hashmap.get(state_name);
        if let None = index {
            return Err(TuringError::UnknownStateError { state_name: state_name.to_string() });
        }
        let index = *index.unwrap();
        // if the node is one of the 3 initial nodes, throw an error
        if index <= 2  {
            return Err(TuringError::IllegalActionError { cause: format!("Tried to delete the state {state_name}.") });
        }
        
        /* Remove references to this state from *all* other nodes transitions */
        let mut states = vec!();
        // Fetch all names that aren't the state we are trying to remove
        for name in self.name_index_hashmap.keys() {
            if name.eq(state_name) {
                continue;
            }
            states.push(name.clone());
        }
        let mut prev_val;
        for name in &states
        {
            // Remove all transitions to this node
            if let Err(e) = self.remove_transitions(&name, state_name) {
                return Err(e);
            }
            // if this node has a bigger index, we lower it by one in the hashmap,
            // because it means that the removed node was placed before in the vector and that the position of this node has changed
            prev_val = *self.name_index_hashmap.get_mut(name).unwrap();
            if prev_val >= index
            {
                *self.name_index_hashmap.get_mut(name).unwrap() -= 1;
                // notify all neighboring nodes about that change
                for name_other in &states 
                {
                    if !name_other.eq(state_name) {
                        continue;       
                    }
                    // TODO : simplify this line
                    self.states[*self.name_index_hashmap.get(name_other).unwrap() as usize].update_transitions(prev_val, prev_val + 1);

                }
            }
        }
        // Remove the node 
        self.name_index_hashmap.remove(state_name);
        self.states.remove(index.into());
        Ok(())
    }
}

impl Debug for TuringMachineGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringMachine").field("states", &self.states).field("hashmap", &self.name_index_hashmap).finish()
    }
}

/// A trait used to iterate over all the states of a turing machine.
pub trait TuringIterator
{
    /// Gets the stored turing machine.
    fn get_turing_machine(&self) -> &TuringMachineGraph;
    /// Gets the current state pointer of this struct. 
    fn get_state_pointer(&self) -> u8;
    /// Sets a new value to the state pointer.
    fn set_state_pointer(&mut self, new_val: u8);
    /// Gets the writtings ribbons stored inside this struct.
    fn get_reading_ribbon(&mut self) -> &mut TuringReadRibbon;
    /// Gets the writtings ribbons stored inside this struct.
    fn get_writting_ribbons(&mut self) -> &mut Vec<TuringWriteRibbon>;

    /// Transforms the current struct as a [TuringExecutor] in order to start 
    /// iterating.
    fn as_iter(&mut self) -> &mut dyn TuringIterator;
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
                for _ in 0..mt.k
                {
                    v.push(TuringWriteRibbon::new());
                }
                v
            },
            word,
            graph: mt,
        };
        // Add the word to the reading ribbon
        s.reading_ribbon.feed_word(s.word.to_string());

        Ok(s)
    }
}



impl<'a> TuringIterator for TuringMachineWithRef<'a> {

    fn get_turing_machine(&self) -> &TuringMachineGraph {
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
}

impl TuringMachine {
    /// Create a new [TuringMachine] for a given graph and word.
    pub fn new(mt: TuringMachineGraph, word: String) -> Result<Self, TuringError>
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

        Ok(s)
    }
}


impl TuringIterator for TuringMachine {
    fn get_turing_machine(&self) -> &TuringMachineGraph {
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
}




pub struct TuringExecutionStep
{
    /// The index of the transition taken from the current state to the next one.
    pub transition_index_taken : usize,
    /// A clone of the transition that was just taken
    pub transition_taken : TuringTransition,
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
        // Fetch the current state
        let curr_state =  self.get_turing_machine().get_state(self.get_state_pointer()).unwrap().clone();

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
        //println!("{:?}", curr_state);
        
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
        for i in 0..self.get_turing_machine().k
        {
            self.get_writting_ribbons()[i as usize].try_apply_transition(transition.chars_read[(i+1) as usize],
                                                                                    transition.chars_write[i as usize].0, &transition.chars_write[i as usize].1).unwrap();
        }

        // Move to the next state
        self.set_state_pointer(transition.index_to_state);

        Some(TuringExecutionStep
        {
            transition_index_taken,
            transition_taken: transition.clone(),
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

        write!(f, "* Took the following transition : {}\n* Ribbons:\nREAD:\n{}\nWRITE:\n{}", self.transition_taken, self.read_ribbon, write_str_rib)
    }
}