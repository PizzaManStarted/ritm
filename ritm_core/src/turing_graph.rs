use std::{collections::HashMap, fmt::{Debug, Display}, os::linux::raw::stat, usize};
use crate::{turing_errors::TuringError, turing_state::{TuringStateType, TuringState, TuringTransitionMultRibbons}};


/// A struct representing a Turing Machine graph with `k` **writting** ribbons (`k >= 1`).
pub struct TuringMachineGraph
{
    /// The hashmap containing a mapping of all nodes names and their related index in the `states` field.
    name_index_hashmap: HashMap<String, usize>, 
    /// The vector containing all the nodes of the turing machine graph
    states: Vec<TuringState>,
    /// The number of ribbons this graph was made for
    k: usize,
}




impl TuringMachineGraph {
    /// Creates a new empty Turing Machine graph that has `k` writting ribbons (`k >= 1`).
    /// 
    /// Three default states will be created :
    /// * `q_i` : The initial state
    /// * `q_a` : The default accepting state
    /// * `q_r` : The default rejecting state
    pub fn new(k: usize) -> Result<Self, TuringError>
    {
        if k == 0 {
            return Err(TuringError::IllegalActionError { cause: "Tried to create a turing machine graph with 0 writting ribbon".to_string() });
        }
        // Add the default states
        let init_state = TuringState::new(TuringStateType::Normal, &String::from("i"));
        let accepting_state = TuringState::new(TuringStateType::Accepting, &String::from("a"));
        let rejecting_state = TuringState::new(TuringStateType::Rejecting, &String::from("r"));
        
        // Create the hash map with the already known states
        let mut name_index_hashmap: HashMap<String, usize> = HashMap::new();
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
    /// If one of the given state didn't already exists, a [TuringError::UnknownStateError] will be returned.
    pub fn append_rule_state_by_name(&mut self, from: &String, transition: TuringTransitionMultRibbons, to: &String) -> Result<(), TuringError>
    {
        // Checks if the given number of ribbons is correct
        if transition.get_number_of_affected_ribbons() != (self.k + 1) as usize
        {
            return Err(TuringError::IncompatibleTransitionError {expected: self.get_k(), received: transition.get_number_of_affected_ribbons() - 1 });
        }
        let from_index = self.name_index_hashmap.get(from);
        if let None = from_index {
            return Err(TuringError::UnknownStateError { state_name: from.clone() });
        }
        let from_index = *from_index.unwrap();


        let to_index = self.name_index_hashmap.get(to);
        if let None = to_index {
            return Err(TuringError::UnknownStateError { state_name: to.clone() });
        }
        let to_index = *to_index.unwrap();

        
        match self.add_rule_state_ind(from_index, transition, to_index) {
            Ok(()) => {
                return Ok(());
            },
            Err(e) => {
                return Err(e);
            },
        };
    }

    /// Adds a new rule/transition to a state of the machine of the form : `from {transition} to`.
    /// Meaning, a new edge is added to the graph.
    /// 
    /// ## Returns
    /// * If everything went correctly : `Ok(())`
    /// * Otherwise, it will return a [TuringError]
    pub fn append_rule_state(&mut self, from_index: usize, transition: TuringTransitionMultRibbons, to_index: usize) -> Result<(), TuringError>
    {
        // Checks if the given correct of number transitions was given
        if transition.chars_write.len() != self.k as usize
        {
            return Err(TuringError::IncompatibleTransitionError {expected: self.k, received: transition.chars_write.len()});
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
    pub fn append_rule_state_self(mut self, from: &String, transition: TuringTransitionMultRibbons, to: &String) -> Result<Self, TuringError>
    {
        match self.append_rule_state_by_name(from, transition, to) {
            Ok(()) => return Ok(self),
            Err(e) => return Err(e),
        };
    }

    /// Adds a new state to the turing machine graph and returns its index. Meaning a new node is added to the graph.
    /// 
    /// If the state name already existed then the index of the already existing state is returned.
    pub fn add_state(&mut self, name: &String) -> usize
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
                self.states.push(TuringState::new(TuringStateType::Normal, name));
                // Adds the index of this state to the hashmap 
                self.name_index_hashmap.insert(name.to_string(), (self.states.len()-1) as usize);
                // Returns the index of the newly created state
                return (self.states.len()-1) as usize;
            },
        }
    }

    /// Adds a new state to the turing machine graph using variables indexes
    fn add_rule_state_ind(&mut self, from: usize, mut transition: TuringTransitionMultRibbons, to: usize) -> Result<(), TuringError>
    {
        if self.states.len() <= from as usize {
            return Err(TuringError::OutOfRangeStateError { accessed_index: from as usize, states_len: self.states.len() });
        }
        if self.states.len() <= to as usize {
            return Err(TuringError::OutOfRangeStateError { accessed_index: to as usize, states_len: self.states.len() });
        }
        // Change transition index
        transition.index_to_state = Some(to);

        let state  = self.states.get_mut(from as usize).unwrap();
        return state.add_transition(transition);
    }

    /// Returns the state (*node*) at the given index.
    pub fn get_state(&self, pointer: usize) -> Result<&TuringState, TuringError>
    {
        if self.states.len() <= pointer as usize {
            return Err(TuringError::OutOfRangeStateError { accessed_index: pointer as usize, states_len: self.states.len() });
        }
        Ok(&self.states[pointer as usize])
    }

    /// Returns the **mutable** state (*node*) at the given index.
    fn get_state_mut(&mut self, pointer: usize) -> Result<&mut TuringState, TuringError>
    {
        if self.states.len() <= pointer as usize {
            return Err(TuringError::OutOfRangeStateError { accessed_index: pointer as usize, states_len: self.states.len() });
        }
        Ok(&mut self.states[pointer as usize])
    }

    /// Returns the state (*node*) that has the given name.
    pub fn get_state_from_name(&self, name: &String) -> Result<&TuringState, TuringError>
    {
        match self.name_index_hashmap.get(name) {
            Some(index) => self.get_state(*index),
            None => Err(TuringError::UnknownStateError { state_name: name.to_string() }),
        }
    }
    
    /// Returns the **mutable** state (*node*) that has the given name.
    fn get_state_from_name_mut(&mut self, name: &String) -> Result<&mut TuringState, TuringError>
    {
        match self.name_index_hashmap.get(name) {
            Some(index) => self.get_state_mut(*index),
            None => Err(TuringError::UnknownStateError { state_name: name.to_string() }),
        }
    }

    /// Get the transition index between two nodes if it exists.
    pub fn get_transition_indexes_by_name(&self, n1: &String, n2: &String) -> Result<Vec<usize>, TuringError>
    {
        let mut res = vec!();
        // Get n1 and n2 indexes if they exists
        let n1_state = match self.name_index_hashmap.get(n1) 
        {
            Some(i) => &self.states[*i as usize],
            None => return Err(TuringError::UnknownStateError { state_name: n1.clone() }),
        };
        let n2_index = match self.name_index_hashmap.get(n2) 
        {
            Some(i) => *i,
            None => return Err(TuringError::UnknownStateError { state_name: n2.clone() }),
        };

        for (i, t) in n1_state.transitions.iter().enumerate() {
            if t.index_to_state.unwrap() == n2_index {
                res.push(i);
            }
        }
        
        return Ok(res);
    }

    /// Get all the transitions between two nodes.
    pub fn get_transitions_by_index(&self, n1: usize, n2: usize) -> Result<Vec<&TuringTransitionMultRibbons>, TuringError>
    {
        let mut vec = vec!();
        // Get n1 index
        let n1_state = self.get_state(n1);
        if let Err(e) = n1_state {
            return Err(e);
        }
        let n1_state = n1_state.unwrap();

        // Fetch all transition that go toward n2
        for t in n1_state.transitions.iter() {
            if t.index_to_state.unwrap() == n2 {
                vec.push(t);
            }
        }
        
        return Ok(vec);
    }

    /// Removes **all** the transitions from this state to the given node
    pub fn remove_transitions(&mut self, from: &String, to: &String) -> Result<(), TuringError>
    {
        let val = self.fetch_n1_state_n2_index(from, to);
        if let Err(e) = val {
            return Err(e);
        }
        let (n1_state, n2_index) = val.unwrap();

        // Remove all transitions from n1 to n2
        n1_state.remove_transitions(n2_index);
        Ok(())
    }

    /// Removes **all** the transitions from a state to the given node, using the nodes `to` node index.
    pub fn remove_transitions_with_index(&mut self, from: &String, to: usize) -> Result<(), TuringError>
    {
        // check that `from` state exists
        let val = self.get_state_from_name_mut(from);
        
        if let Err(e) = val {
            return Err(e);
        }
        let n1_state = val.unwrap();

        // Remove all transitions from n1 to n2
        n1_state.remove_transitions(to);
        Ok(())
    }

    


    /// Removes all transitions of the form `from {transition} to` using the given parameters.
    ///  
    /// The `transition`'s `index_to_state` field, will not be be taken into account here (as it will be changed with the index of `to` anyways), the rest however is still important.
    /// 
    /// Only `from`'s existance will be verified (and will return an error if it does not exists). But `to`'s index can be outside the bounds.
    pub fn remove_transition(&mut self, from: &String, transition: &TuringTransitionMultRibbons, to: &String) -> Result<(), TuringError>
    {
        let val = self.fetch_n1_state_n2_index(from, to);
        if let Err(e) = val {
            return Err(e);
        }
        let (n1_state, n2_index) = val.unwrap();

        let mut trans = transition.clone();
        // In order to make sure it is removed, we change the index to the correct one 
        trans.index_to_state = Some(n2_index);
        n1_state.remove_transition(&trans);

        Ok(())
    }


    fn fetch_n1_state_n2_index(&mut self, from: &String, to: &String) -> Result<(&mut TuringState, usize), TuringError>
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

        return Ok((n1_state, *n2_index));
    }

    /// Removes a state and **all** mentions of it in **all** transitions of **all** the other states of the TuringMachine using its name.
    pub fn remove_state_with_name(&mut self, state_name: &String) -> Result<(), TuringError>
    {
        // First keep the index for later
        let index = self.name_index_hashmap.get(state_name);
        if let None = index {
            return Err(TuringError::UnknownStateError { state_name: state_name.to_string() });
        }
        let index = *index.unwrap();
        // Use that index to remove that state
        self.remove_state_with_index(index)
    }


    /// Removes a state and **all** mentions of it in **all** transitions of **all** the other states of the TuringMachine using its index.
    fn remove_state_with_index(&mut self, state_index: usize) -> Result<(), TuringError>
    {
        // if the node is one of the 3 initial nodes, throw an error
        if state_index <= 2  {
            return Err(TuringError::IllegalActionError { cause: format!("Tried to delete the state {}.", self.get_state(state_index).unwrap().name) });
        }

        // get state name
        let state_name = self.get_state(state_index).cloned();
        if let Err(e) = state_name {
            return Err(e);
        }
        let state_name = &state_name.unwrap().name;
        
        /* Remove references to this state from *all* other nodes transitions */
        let mut states = vec!();
        // Fetch all names that aren't the state we are trying to remove
        for name in self.name_index_hashmap.keys() {
            if name.eq(state_name) {
                continue;
            }
            states.push(name.clone());
        }

        let mut to_notify_neigh = vec!();

        // Remove the node
        self.states.remove(state_index.into()); // this means that other indexes might have shifted too
        // Collect all values that are gonna change

        let mut prev_val;
        for name in &states {
            prev_val = *self.name_index_hashmap.get_mut(name).unwrap();
            // If this node had a bigger index, we lower it by one in the hashmap (it moved in the `states` vector)
            if prev_val >= state_index {
                *self.name_index_hashmap.get_mut(name).unwrap() -= 1;
                // Save this state for later use
                to_notify_neigh.push(prev_val);
            }

            // Remove all transitions to the removed node
            if let Err(e) = self.remove_transitions_with_index(&name, state_index) {
                return Err(e);
            }
        }
        for state in &mut self.states {
            for changed_index in &to_notify_neigh {
                state.update_transitions(*changed_index, *changed_index - 1);
            }
        }
        
        self.name_index_hashmap.remove(state_name);
        Ok(())
    }
    
    
    pub fn get_k(&self) -> usize 
    {
        return self.k;
    }

    pub fn get_name_index_hashmap(&self) -> &HashMap<String, usize>
    {
        return &self.name_index_hashmap;
    }

    pub fn get_states(&self) -> &Vec<TuringState>
    {
        return &self.states;
    }
}

impl Debug for TuringMachineGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringMachine").field("k", &self.k).field("states", &self.states).field("hashmap", &self.name_index_hashmap).finish()
    }
}


impl Clone for TuringMachineGraph {
    fn clone(&self) -> Self {
        Self { name_index_hashmap: self.name_index_hashmap.clone(), states: self.states.clone(), k: self.k.clone() }
    }
}


impl Display for TuringMachineGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("States:\n");
        
        // Print all states
        for state in &self.states {
            res.push_str(format!("{}: {}\n", state.name, state.state_type).as_str());
        }

        res.push_str("\nTransitions:\n");
        let mut res_tr = String::new();
        // Print all transitions btw states
        for (q1, i1) in &self.name_index_hashmap {
            for (q2, i2) in &self.name_index_hashmap {
                let transitions = self.get_transitions_by_index(*i1, *i2).unwrap();
                if transitions.is_empty() {
                    continue;
                }
                res_tr.push_str(format!("q_{} {} ", q1, '{').as_str());
                let spaces = 3 + q1.len();

                for i in 0..transitions.len()-1 {
                    res_tr.push_str(format!("{} \n{}| ", transitions.get(i).unwrap(), " ".repeat(spaces)).as_str());
                }
                // add last
                res_tr.push_str(format!("{} ", transitions.last().unwrap()).as_str());
                

                res_tr.push_str(format!("{} q_{};\n\n", "}", q2).as_str());
            }
        }
        if res_tr.is_empty() {
            res.push_str("None");
        }
        else {
            res.push_str(res_tr.as_str());
        }

        write!(f, "{}", res)
    }
}