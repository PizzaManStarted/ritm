use std::{
    char,
    fmt::{Debug, Display},
};

/// Represents a state of a turing machine
pub struct TuringState {
    pub is_final: bool,
    transitions: Vec<TuringTransition>,
    pub name: Option<String>,
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

    /// Sets the name of a [TuringState]
    /// 
    /// Returns the given [TuringState]
    pub fn set_name(mut self, name: &str) -> Self
    {
        self.name = Some(name.to_string());
        return self;
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
                return res;    // FIXME add error here
            }
            //println!("Let me check for : {} | chars read: {:?} and tr.read : {:?}", t, chars_read, t.chars_read);
            if chars_read.eq(&t.chars_read) {
                //println!("\t*Adding it !");
                res.push(t);
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

impl Clone for TuringDirection {
    fn clone(&self) -> Self {
        match self {
            Self::Left => Self::Left,
            Self::Right => Self::Right,
            Self::None => Self::None,
        }
    }
}


/// A struct representing a turing transition of the form : 
/// * `a_0, a_1, ..., a_{n-1} -> D_0, b_0, D_1, b_1, D_2, ..., b_{n-1}, D_n`
pub struct TuringTransition {
    /// The chars that have to be read in order apply the rest of the transition : `a_0,..., a_{n-1}`
    pub chars_read: Vec<char>,
    /// The move to take after writing/reading the character : `D_0`
    pub move_read: TuringDirection,
    /// The character to replace the character just read : `(b_0, D_1),..., (b_{n-1}, D_n)`
    pub chars_write: Vec<(char, TuringDirection)>,
    /// The index of the state to go to after passing through this state.
    pub index_to_state: u8,
}

impl TuringTransition {
    /// Creates a new [TuringTransition] of the form : 
    /// * `a_0, a_1, ..., a_{n-1} -> D_0, b_0, D_1, b_1, D_2, ..., b_{n-1}, D_n`
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

    /// Simplifies the creationf of a new [TuringTransition] of the form : 
    /// * `a_0, a_1, ..., a_{n-1} -> D_0, b_0, D_1, b_1, D_2, ..., b_{n-1}, D_n`
    /// 
    /// ## Args :
    /// * **chars_read** : The characters that have to be read in order to take this transition : `a_0,..., a_{n-1}`
    /// * **chars_write** : The characters to replace the characters read : `b_0, ..., b_{n-1}` 
    /// * **directions** : The directions to move the pointers of the ribbons : `D_0, ..., D_n` 
    pub fn create(chars_read: Vec<char>, chars_write: Vec<char>, directions: Vec<TuringDirection>) -> Self
    {
        let mut chars_write_dir: Vec<(char, TuringDirection)> = vec!();
        let move_read: TuringDirection = directions.get(0).expect("There must be at least one direction").clone();

        // TODO : Replace panics with real errors
        if chars_write.len() != directions.len() -1 {
            panic!("chars_write must have the same size as (directions - 1) in order to create couples of (char, TuringDirection)");
        }
        if chars_read.len() != directions.len() {
            panic!("The number of chars to read must be equal to the number of (char, directions) to replace them with")
        }
        for i in 1..directions.len() 
        {
            chars_write_dir.push((*chars_write.get(i-1).unwrap(), directions.get(i).unwrap().clone()));        
        }
        Self {
            chars_read,
            move_read,
            chars_write: chars_write_dir,
            index_to_state : 0,
        }
    }

    /// Returns the number of ribbons that are going to be affected by this transition.
    pub fn get_number_of_ribbons(&self) -> usize
    {
        return self.chars_write.len();
    }
}


impl Clone for TuringTransition {
    fn clone(&self) -> Self {
        Self { chars_read: self.chars_read.clone(), move_read: self.move_read.clone(), chars_write: self.chars_write.clone(), index_to_state: self.index_to_state.clone() }
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