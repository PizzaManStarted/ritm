use std::{
    char, f32::consts::E, fmt::{Debug, Display}
};

use crate::turing_errors::TuringError;

/// Represents a state of a turing machine
pub struct TuringState {
    /// Represents if the state is a final state or not
    pub is_final: bool,
    /// The vector containing all the transitions to the neighboring states 
    pub transitions: Vec<TuringTransitionMultRibbons>,
    /// The name of this state
    pub name: String,
}

impl TuringState {
    /// Creates a new [TuringState]
    pub fn new(is_final: bool, name: &String) -> Self {
        Self {
            is_final,
            transitions: vec![],
            name: name.clone(),
        }
    }

    /// Changes the name of a [TuringState]
    pub fn rename(&mut self, name: &str)
    {
        self.name = name.to_string();
    }

    /// Adds a new transition to the state
    pub fn add_transition(&mut self, transition: TuringTransitionMultRibbons) -> Result<(), TuringError> 
    {
        // Check that the number of ribbon from a transition is the same for all added transitions
        if ! self.transitions.is_empty() && self.transitions.first().unwrap().get_number_of_affected_ribbons() != transition.get_number_of_affected_ribbons() {
            return Err(TuringError::ArgsSizeTransitionError);
        }

        Ok(self.transitions.push(transition))
    }

    /// Removes the transition ***at*** the given index and returns it if it was correctly returned
    pub fn remove_transition_with_index(&mut self, transition_index: u8) -> Result<TuringTransitionMultRibbons, TuringError>
    {
        if self.transitions.len() <= transition_index as usize {
            return Err(TuringError::OutOfRangeTransitionError { accessed_index: transition_index as usize, states_len: self.transitions.len() });
        }
        Ok(self.transitions.remove(transition_index.into()))
    }

    /// Removes all the transitions matching the given parameter. Beware that the `index_to_state` field will also be part of the evaluation.
    /// 
    /// If the transition wasn't part of this state, nothing will happen.
    pub fn remove_transition(&mut self, transition: &TuringTransitionMultRibbons)
    {
        let mut res = vec!();

        for t in &self.transitions {
            if t != transition || t.index_to_state != transition.index_to_state {
                res.push(t.clone());
            }
        }

        self.transitions = res;
        
    }

    /// Removes all the transitions from this state ***that are pointing*** at the given index
    pub fn remove_transitions(&mut self, to_index: u8)
    {
        let mut transitions = vec!();
        for t in &self.transitions 
        {
            if let Some(index_to_state) = t.index_to_state {
                // If it is pointing at the given index, we remove it
                if index_to_state == to_index 
                {
                    continue;
                }
            }
            transitions.push(t.clone());
        }
        self.transitions = transitions;
    }

    /// Updates the transition index to a new one
    pub fn update_transitions(&mut self, to_index_curr: u8, to_index_new: u8)
    {
        for t in &mut self.transitions
        {
            if let Some(index_to_state) = t.index_to_state {
                // If it was pointing to the old index, update it
                if index_to_state == to_index_curr 
                {
                    t.index_to_state = Some(to_index_new);
                    println!("changing it : from {} to {}", index_to_state, to_index_new);    
                }
            }
        }
    }

    /// Checks for all transitions that can be taken when reading a char in this state
    pub fn get_valid_transitions(&self, chars_read: Vec<char>) -> Vec<&TuringTransitionMultRibbons> {
        let mut res = vec![];
        for t in &self.transitions {
            if chars_read.eq(&t.chars_read) {
                res.push(t);
            }
        }
        return res;
    }

    /// Gets all the transitions that can be taken to reach the given index.
    pub fn get_transitions_to(&self, to_index: u8) -> Vec<&TuringTransitionMultRibbons> {
        let mut res = vec!();

        for t in &self.transitions {
            if let Some(i) = t.index_to_state {
                if i == to_index {
                    res.push(t);
                }
            }
        }

        return res;
    }
}

impl Debug for TuringState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringState")
            .field("name", &self.name)
            .field("is_final", &self.is_final)
            .field("transitions", &self.transitions)
            .finish()
    }
}

impl Clone for TuringState {
    fn clone(&self) -> Self {
        Self { is_final: self.is_final.clone(), transitions: self.transitions.clone(), name: self.name.clone() }
    }
}


/// Represents the direction of a movement that the pointer of a ribbon can take after reading/writing a character
pub enum TuringDirection {
    Left,
    Right,
    None,
}

impl TuringDirection {
    /// Return the integer value of the direction.
    ///
    /// Left values are negatives, right values are positives and none is represented by zero.
    pub fn get_value(&self) -> i8 {
        match self {
            Self::Left => -1,
            Self::Right => 1,
            Self::None => 0,
        }
    }
}

impl Debug for TuringDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "Left"),
            Self::Right => write!(f, "Right"),
            Self::None => write!(f, "None"),
        }
    }
}
impl Display for TuringDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Left => "L",
            Self::Right => "R",
            Self::None => "N",
        })
    }
}

impl Clone for TuringDirection {
    fn clone(&self) -> Self {
        match self {
            Self::Left => Self::Left,
            Self::Right => Self::Right,
            Self::None => Self::None,
        }
    }
}

impl PartialEq for TuringDirection {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}


/// A struct representing a transition for a turing machine that has strictly more than **1 ribbon** : 
/// * `a_0, a_1, ..., a_{n-1} -> D_0, b_1, D_1, b_2, D_2, ..., b_{n-1}, D_{n-1}`
/// - With :
///     * `a_i` : The character *i* being read.
///     * `D_i` : Direction to take by taking this transition, see [TuringDirection] for more information.
///     * `b_i` : The character to replace the character *i* with.
pub struct TuringTransitionMultRibbons {
    /// The chars that have to be read in order apply the rest of the transition : `a_0,..., a_{n-1}`
    pub chars_read: Vec<char>,
    /// The move to take after writing/reading the character : `D_0`
    pub move_read: TuringDirection,
    /// The character to replace the character just read : `(b_0, D_0),..., (b_{n-1}, D_{n-1})`
    pub chars_write: Vec<(char, TuringDirection)>,
    /// The index of the state to go to after passing through this state.
    pub index_to_state: Option<u8>,
}

impl TuringTransitionMultRibbons {
    /// Creates a new [TuringTransition].
    pub fn new(
        char_read: Vec<char>,
        move_read: TuringDirection,
        chars_read_write: Vec<(char, TuringDirection)>,
    ) -> Self {
        Self {
            chars_read: char_read,
            move_read,
            chars_write: chars_read_write,
            index_to_state: None,
        }
    }

    /// Simplifies the creation of a new [TuringTransition] of the form : 
    /// * `a_0, a_1, ..., a_{n-1} -> D_0, b_1, D_1, b_2, D_2, ..., b_{n-1}, D_{n-1}`
    /// 
    /// ## Args :
    /// * **chars_read** : The characters that have to be read in order to take this transition : `a_0,..., a_{n-1}`
    /// * **chars_write** : The characters to replace the characters read : `b_0, ..., b_{n-1}` 
    /// * **directions** : The directions to move the pointers of the ribbons : `D_0, ..., D_{n-1}`
    pub fn create(chars_read: Vec<char>, chars_write: Vec<char>, directions: Vec<TuringDirection>) -> Result<Self, TuringError>
    {
        let mut chars_write_dir: Vec<(char, TuringDirection)> = vec!();
        let move_read = directions.get(0);
        
        if let None = move_read {
            return Err(TuringError::ArgsSizeTransitionError);
        }
        let move_read = move_read.unwrap().clone();

        if chars_write.len() + 1 != directions.len(){
            return Err(TuringError::ArgsSizeTransitionError);
        }
        if chars_read.len() != directions.len() {
            return Err(TuringError::ArgsSizeTransitionError);
        }
        for i in 1..directions.len() 
        {
            chars_write_dir.push((*chars_write.get(i-1).unwrap(), directions.get(i).unwrap().clone()));        
        }
        Ok(
            Self {
                chars_read,
                move_read,
                chars_write: chars_write_dir,
                index_to_state : None,
            }
        )
    }
    
    /// Returns the number of ribbons that are going to be affected by this transition.
    pub fn get_number_of_affected_ribbons(&self) -> usize
    {
        return self.chars_write.len() + 1;
    }
}


impl Clone for TuringTransitionMultRibbons {
    fn clone(&self) -> Self {
        Self { chars_read: self.chars_read.clone(), move_read: self.move_read.clone(), chars_write: self.chars_write.clone(), index_to_state: self.index_to_state.clone() }
    }
}

impl Debug for TuringTransitionMultRibbons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringTransition")
            .field("char_read", &self.chars_read)
            .field("move_read", &self.move_read)
            .field("char_write", &self.chars_write)
            .field("next", &self.index_to_state)
            .finish()
    }
}

impl Display for TuringTransitionMultRibbons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        let mut char_read = String::from(self.chars_read[0]);
        for i in 1..self.chars_read.len()
        {
            char_read.push_str(format!(", {}", self.chars_read[i]).as_str());
        }

        let mut char_written = format!("{}", self.move_read);

        for (c, dir) in &self.chars_write {
            char_written.push_str(format!(", {}, {}", c, dir).as_str());
        }



        write!(f, "[{} -> {}]", char_read, char_written)
    }
}

impl PartialEq for TuringTransitionMultRibbons {
    /// Checks if two [TuringTransitionMultRibbons] are equivalent. Note that the `index_to_state` field is not part of this comparison.
    fn eq(&self, other: &Self) -> bool {
        self.chars_read == other.chars_read && self.move_read == other.move_read && self.chars_write == other.chars_write
    }
}