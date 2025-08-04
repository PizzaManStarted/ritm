use std::fmt::Display;
use rustyline::{history::FileHistory, Editor};
use strum::IntoEnumIterator;
use colored::Colorize;

use crate::DataStorage;



pub trait ModeEvent {
    fn print_help(&self);
    fn choose_option(&self, rl: &mut Editor<(), FileHistory>, storage: &mut DataStorage) -> Modes;
}


#[derive(Debug)]
pub enum Modes {
    Start,
    Modify,
    Execute
}


// _________________________________________________________________________

pub fn print_help<E>() where E:IntoEnumIterator + Display
{
    for (i, mode) in E::iter().enumerate() {
            println!("{}: {}", i.to_string().blue().bold(), mode.to_string().italic());
    }
}

pub fn print_help_gen<E>(vec: &Vec<E>) where E:IntoEnumIterator + Display
{
    for (i, mode) in vec.iter().enumerate() {
            println!("{}: {}", i.to_string().blue().bold(), mode.to_string().italic());
    }
}


pub fn collect_enum_values<E>() -> Vec<E> where E:IntoEnumIterator + Display + ModeEvent
{
    E::iter().collect()
}
