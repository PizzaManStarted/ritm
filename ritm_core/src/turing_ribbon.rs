use std::fmt::{Debug, Display};

use crate::{turing_errors::TuringError, turing_state::TuringDirection};


/// A trait used to implement Turing ribbons
pub trait TuringRibbon : Display + Clone
{
    /// Creates a new [TuringRibbon]
    fn new() -> Self;

    /// Tries to apply the transition to the given ribbon.
    /// 
    /// The transition is applied if the character being pointed by the head of this ribbon is the same as the given `if_read` character.
    /// 
    /// ## Returns
    /// A [bool] if everything went correctly :
    /// * `Some(true)` if the transition went smoothly.
    /// * `Some(false)` if the transition could not be taken.
    /// 
    /// A [TuringError] if an error happened, like for example, it was not possible to move at the given direction.
    fn try_apply_transition(&mut self, if_read: char, replace_by: char, move_to: &TuringDirection) -> Result<bool, TuringError>;

    /// Returns the current character being read by the ribbon
    fn read_curr_char(&self) -> char;

    /// Returns the vector of char as stored by the ribbon 
    fn get_contents(&self) -> &Vec<char>;

    /// Returns the index of the char being pointed by the ribbon
    fn get_pointer(&self) -> usize;
}



/// Represents a ribbon made to write and read characters.
pub struct TuringWriteRibbon
{
    chars_vec: Vec<char>,
    pointer: usize
}

/// Represents a ribbon made to store and only read a word.
pub struct TuringReadRibbon
{
    chars_vec: Vec<char>,
    pointer: usize
}

impl TuringRibbon for TuringWriteRibbon 
{
    /// Creates a new [TuringWriteRibbon]
    fn new() -> Self
    {
        Self 
        { 
            chars_vec: vec!('ç', '_'),
            pointer: 0
        }
    }
    
    fn try_apply_transition(&mut self, if_read: char, replace_by: char, move_to: &TuringDirection) -> Result<bool, TuringError>
    {
        // if the correct symbol was read
        if self.chars_vec[self.pointer] == if_read 
        {
            let new_pointer = (self.pointer as isize) + (move_to.get_value() as isize);
            
            if new_pointer < 0
            {
                return Err(TuringError::OutofRangeRibbonError { accessed_index: new_pointer as usize, ribbon_size: self.chars_vec.len() });
            }
            // In a write ribbon, we have an *infinite size*, so we can simulate this by adding when needed a new empty char
            if new_pointer >= self.chars_vec.len() as isize {
                self.chars_vec.push('_');
                
            }
            // Replace the current char read
            self.chars_vec[self.pointer] = replace_by;
            
            // Move to the new position
            self.pointer = new_pointer as usize;
            return Ok(true);
        }
        return Ok(false);
    }
    
    fn read_curr_char(&self) -> char {
        return self.chars_vec[self.pointer];
    }
    
    fn get_contents(&self) -> &Vec<char> {
        &self.chars_vec
    }
    
    fn get_pointer(&self) -> usize {
        self.pointer
    }
}


impl TuringRibbon for TuringReadRibbon
{
    /// Creates a new [TuringReadRibbon]
    fn new() -> Self 
    {
        Self 
        { 
            chars_vec: vec!('ç', '$'),
            pointer: 0
        }
    }
    
    fn try_apply_transition(&mut self, if_read: char, _: char, move_to: &TuringDirection) -> Result<bool, TuringError>
    {
        // if the correct symbol was read
        if self.chars_vec[self.pointer] == if_read 
        {
            // Move to the new position
            let new_pointer = (self.pointer as isize) + (move_to.get_value() as isize);
            
            // If the pointer points out of the bounds of the reading ribbon
            if new_pointer < 0 || new_pointer >= self.chars_vec.len() as isize
            {
                return Err(TuringError::OutofRangeRibbonError { accessed_index: new_pointer as usize, ribbon_size: self.chars_vec.len() });
            }
            self.pointer = new_pointer as usize;
            return Ok(true);
        }
        return Ok(false);
    }
    
    fn read_curr_char(&self) -> char {
        return self.chars_vec[self.pointer];
    }

    fn get_contents(&self) -> &Vec<char> {
        &self.chars_vec
    }

    fn get_pointer(&self) -> usize {
        self.pointer
    }
} 


impl TuringReadRibbon 
{
    /// Feed a word into the read ribbon, and also adds 'ç' and '$' to the extremities of it
    pub fn feed_word(&mut self, word: String)
    {
        self.chars_vec.clear();
        self.chars_vec.push('ç');
        for ch in word.chars() {
            self.chars_vec.push(ch);
        }
        self.chars_vec.push('$');
    }
}



impl Display for TuringReadRibbon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}", ribbon_to_string(&self.chars_vec, self.pointer, false))
    }
}

/// Turns a character vector into an easy string to read for humans, displaying with an arrow the current char being pointed
fn ribbon_to_string(chars_vec: &Vec<char>, pointer: usize, is_inf: bool) -> String
{
    let mut res: String = String::from("[");
    let mut pointing: String = String::from(" ");
    let mut count = 0;
    for c in chars_vec 
    {
        res.push_str(&format!("{c},"));
        if count == pointer
        {
            pointing.push_str("↑ ");
        }
        else {
            pointing.push_str("  ");
        }
        count += 1;
    }
    
    res.pop();
    if is_inf 
    {
        res += ",...";    
    }
    res +=  "]\n";
    
    res.push_str(&pointing);
    return res;
}



impl Debug for TuringWriteRibbon 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TuringRibbon").field("chars_vec", &self.chars_vec).finish()
    }
}

impl Display for TuringWriteRibbon 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}", ribbon_to_string(&self.chars_vec, self.pointer, true))
    }
}


impl Clone for TuringReadRibbon {
    fn clone(&self) -> Self {
        Self { chars_vec: self.chars_vec.clone(), pointer: self.pointer.clone() }
    }
}


impl Clone for TuringWriteRibbon {
    fn clone(&self) -> Self {
        Self { chars_vec: self.chars_vec.clone(), pointer: self.pointer.clone() }
    }
}


// Keeping the unit test here because we need to access private fields
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_new_read_ribbon()
    {
        let ribbon = TuringReadRibbon::new();

        assert_eq!(ribbon.pointer, 0);
        assert_eq!(ribbon.chars_vec, vec!('ç', '$'));
        let ribbon = TuringWriteRibbon::new();
    
        assert_eq!(ribbon.pointer, 0);
        assert_eq!(ribbon.chars_vec, vec!('ç', '_'));
    }

    #[test]
    fn test_feed_word_ribbon()
    {
        let mut ribbon = TuringReadRibbon::new();
        
        ribbon.feed_word("test".to_string());

        assert_eq!(ribbon.chars_vec, vec!('ç', 't', 'e', 's', 't', '$'));
    }


    #[test]
    fn test_transition_read_ribbon()
    {
        let mut ribbon = TuringReadRibbon::new();
        
        ribbon.feed_word("test".to_string());

        ribbon.try_apply_transition('ç', 'ç', &TuringDirection::Right).unwrap();
        assert_eq!(ribbon.pointer, 1);
        ribbon.try_apply_transition('t', 'p', &TuringDirection::Left).unwrap();
        assert_eq!(ribbon.pointer, 0);
        ribbon.try_apply_transition('ç', 'ç', &TuringDirection::None).unwrap();
        assert_eq!(ribbon.pointer, 0);
        
        assert_eq!(ribbon.chars_vec, vec!('ç', 't', 'e', 's', 't', '$'));


        ribbon.try_apply_transition('ç', '_', &TuringDirection::Right).unwrap();
        ribbon.try_apply_transition('t', '_', &TuringDirection::Right).unwrap();
        ribbon.try_apply_transition('e', '_', &TuringDirection::Right).unwrap();
        ribbon.try_apply_transition('s', '_', &TuringDirection::Right).unwrap();
        ribbon.try_apply_transition('t', '_', &TuringDirection::Right).unwrap();

        match ribbon.try_apply_transition('$', '_', &TuringDirection::Right) {
            Ok(b) => {panic!("Transition should have returned a TuringError and not : {}", b)},
            Err(e) => {
                match e {
                    TuringError::OutofRangeRibbonError { accessed_index: _, ribbon_size: _ } => (),
                    _ => panic!("OutofRangeRibbonError was expected")
                }
            },
        }

    }

    
}