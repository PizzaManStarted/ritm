use std::fmt::{Debug, Display};


pub struct TuringState
{
    is_final: bool,
    transitions: Vec<TuringTransition>,
    name: Option<String>
}

impl TuringState 
{
    pub fn new(is_final: bool) -> Self
    {
        Self {  is_final, 
                transitions: vec!(),
                name: None 
            }
    }    
}

impl Debug for TuringState 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringState").field("is_final", &self.is_final).field("transitions", &self.transitions).field("name", &self.name).finish()
    }
}


enum TuringDirection {
    Left,
    Right,
    None
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

struct TuringTransition
{
    char_read: char,
    move_read: TuringDirection,
    // When *first* char is read, replace it with the second *char* and move to direction 
    char_write: (char, Option<char>, TuringDirection)
}

impl Debug for TuringTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringTransition").field("char_read", &self.char_read).field("move_read", &self.move_read).field("char_write", &self.char_write).finish()
    }
}



pub struct TuringRubon
{
    chars_vec: Vec<char>
}


impl TuringRubon 
{
    pub fn new() -> Self
    {
        Self 
        { 
            chars_vec: vec!('ç'),
        }
    }
}


impl Debug for TuringRubon 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringRubon").field("chars_vec", &self.chars_vec).finish()
    }
}

impl Display for TuringRubon 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res: String = String::from("[");
        for c in &self.chars_vec 
        {
            res.push_str(&format!("{c},"));
        }
        res.pop();
        res += "]";
        write!(f, "({})", self)
    }
}