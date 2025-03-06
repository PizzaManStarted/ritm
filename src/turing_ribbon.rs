use std::{fmt::{Debug, Display}, u16};

use crate::turing_state::TuringDirection;

pub trait TuringRibbon : Display
{
    
    fn new() -> Self;

    fn transition_state(&mut self, if_read: char, replace_by: char, move_to: TuringDirection) -> bool;

    fn read_curr_char(&self) -> char;

}

pub struct TuringWriteRibbon
{
    chars_vec: Vec<char>,
    pointer: usize
}


pub struct TuringReadRibbon
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
    
    fn transition_state(&mut self, _: char, replace_by: char, move_to: TuringDirection) -> bool
    {
        // Replace the current char read
        self.chars_vec[self.pointer] = replace_by;
        self.chars_vec.push('_');
        // Move to the new position
        let new_pointer = (self.pointer as isize) + (move_to.get_value() as isize);
        
        self.pointer = new_pointer as usize;
        return true;
    }
    
    fn read_curr_char(&self) -> char {
        return self.chars_vec[self.pointer];
    }
}


impl TuringRibbon for TuringReadRibbon
{
    fn new() -> Self 
    {
        Self 
        { 
            chars_vec: vec!('ç', '$'),
            pointer: 0
        }
    }
    
    fn transition_state(&mut self, if_read: char, _: char, move_to: TuringDirection) -> bool
    {
        // if the correct symbol was read
        if self.chars_vec[self.pointer] == if_read 
        {
            // Replace the current char read
            self.chars_vec.push('_');
            // Move to the new position
            let new_pointer = (self.pointer as isize) + (move_to.get_value() as isize);
            
            self.pointer = new_pointer as usize;
            return true;
        }
        return false;
    }
    
    fn read_curr_char(&self) -> char {
        return self.chars_vec[self.pointer];
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
        write!(f, "{}", ribbon_to_string(&self.chars_vec, self.pointer))
    }
}

impl Display for TuringReadRibbon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}", ribbon_to_string(&self.chars_vec, self.pointer))
    }
}

/// Turns a character vector into an easy string to read for humans, displaying with an arrow the current char being pointed
fn ribbon_to_string(chars_vec: &Vec<char>, pointer: usize) -> String
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
    // [ç,a,a,a,a]
    res.pop();
    res += "]\n";
    res.push_str(&pointing);
    return res;
}