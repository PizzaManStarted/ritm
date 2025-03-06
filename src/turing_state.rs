use std::fmt::{Debug, Display};

/// Represents a state of a turing machine
pub struct TuringState
{
    is_final: bool,
    transitions: Vec<TuringTransition>,
    name: Option<String>
}

impl TuringState 
{
    /// Creates a new [TuringState]
    pub fn new(is_final: bool) -> Self
    {
        Self {  is_final, 
                transitions: vec!(),
                name: None 
            }
    }

    /// Adds a new transition to the state
    pub fn add_transition(&mut self, transition: TuringTransition)
    {
        self.transitions.push(transition);
    }
}

impl Debug for TuringState 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringState").field("is_final", &self.is_final).field("transitions", &self.transitions).field("name", &self.name).finish()
    }
}


/// Represents the direction of a movement a ribbon can take after reading/writing a character
pub enum TuringDirection {
    Left,
    Right,
    None
}


impl TuringDirection {
    /// Return the integer value of the direction. 
    /// 
    /// Left values are negatives, right values are positives and none is represented by zero. 
    pub fn get_value(&self) -> i8
    {
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

/// A struct representing a turing transition 
pub struct TuringTransition
{
    /// The char that has to be read in order apply the rest of the transition.
    pub char_read: char,
    /// The move to take after writing/reading the character.
    pub move_read: TuringDirection,
    /// The character to replace the character just read.
    pub chars_write: Vec<(char, TuringDirection)>,
    /// The index of the state to go to after passing through this state.
    pub index_to_state: u8,
}

impl TuringTransition {
    /// Creates a new [TuringTransition]
    pub fn new(char_read: char, move_read: TuringDirection, chars_read_write: Vec<(char, TuringDirection)>) -> Self
    {
        Self { 
            char_read, move_read, chars_write: chars_read_write, index_to_state: 0 
        }
    }
}

impl Debug for TuringTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringTransition").field("char_read", &self.char_read).field("move_read", &self.move_read).field("char_write", &self.chars_write).finish()
    }
}



pub trait TuringRibbon 
{
    fn new() -> Self;

    fn transition_state(&mut self, if_read: char, replace_by: char, move_to: TuringDirection);
}

pub struct TuringWriteRibbon
{
    chars_vec: Vec<char>,
    pointer: usize
}


impl TuringRibbon for TuringWriteRibbon 
{
    fn new() -> Self
    {
        Self 
        { 
            chars_vec: vec!('ç'),
            pointer: 0
        }
    }

    fn transition_state(&mut self, if_read: char, replace_by: char, move_to: TuringDirection)
    {
        // If the current char being pointed at is the expected char
        if self.chars_vec[self.pointer] == if_read 
        {
            // Replace the current char read
            self.chars_vec[self.pointer] = replace_by;
            self.chars_vec.push('_');
            // Move to the new position
            let new_pointer = (self.pointer as isize) + (move_to.get_value() as isize);

            self.pointer = new_pointer as usize;
        }
    }
}


impl Debug for TuringWriteRibbon 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringRibbon").field("chars_vec", &self.chars_vec).finish()
    }
}

impl Display for TuringWriteRibbon 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = String::from("[");
        let mut pointing: String = String::from(" ");
        let mut count = 0;
        for c in &self.chars_vec 
        {
            res.push_str(&format!("{c},"));
            if count == self.pointer
            {
                pointing.push_str("↑ ");
            }
            else {
                pointing.push_str("  ");
            }
            count += 1;
        }
        // [ç,a,a,a,a]
        res.pop();
        res += "]\n";
        res.push_str(&pointing);
        write!(f, "{}", res)
    }
}