use std::{
    char,
    fmt::{Debug, Display},
};

/// Represents a state of a turing machine
pub struct TuringState {
    is_final: bool,
    transitions: Vec<TuringTransition>,
    name: Option<String>,
}

impl TuringState {
    /// Creates a new [TuringState]
    pub fn new(is_final: bool) -> Self {
        Self {
            is_final,
            transitions: vec![],
            name: None,
        }
    }

    /// Adds a new transition to the state
    pub fn add_transition(&mut self, transition: TuringTransition) {
        self.transitions.push(transition);
    }

    /// Checks for all transitions that can be taken when reading a char in this state
    pub fn get_valid_transitions(&self, chars_read: Vec<char>) -> Vec<&TuringTransition> {
        let mut res = vec![];
        for t in &self.transitions {
            if chars_read.len() != t.chars_read.len() 
            {
                return res;    
            }
            for i in 0..chars_read.len() 
            {
                // If one of the characters wasn't read, stop, and do not add this transition as valid
                if t.chars_read[i] != chars_read[i] 
                {
                    break;
                }
                res.push(t);
                
            }
        }
        return res;
    }
}

impl Debug for TuringState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringState")
            .field("is_final", &self.is_final)
            .field("transitions", &self.transitions)
            .field("name", &self.name)
            .finish()
    }
}

/// Represents the direction of a movement a ribbon can take after reading/writing a character
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

/// A struct representing a turing transition
pub struct TuringTransition {
    /// The chars that have to be read in order apply the rest of the transition.
    pub chars_read: Vec<char>,
    /// The move to take after writing/reading the character.
    pub move_read: TuringDirection,
    /// The character to replace the character just read.
    pub chars_write: Vec<(char, TuringDirection)>,
    /// The index of the state to go to after passing through this state.
    pub index_to_state: u8,
}

impl TuringTransition {
    /// Creates a new [TuringTransition]
    pub fn new(
        char_read: Vec<char>,
        move_read: TuringDirection,
        chars_read_write: Vec<(char, TuringDirection)>,
    ) -> Self {
        Self {
            chars_read: char_read,
            move_read,
            chars_write: chars_read_write,
            index_to_state: 0,
        }
    }
}

impl Debug for TuringTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringTransition")
            .field("char_read", &self.chars_read)
            .field("move_read", &self.move_read)
            .field("char_write", &self.chars_write)
            .finish()
    }
}

impl Display for TuringTransition {
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