use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use colored::Colorize;


pub trait ModeEvent {
    fn print_help(&self);
    fn choose_option(&self);
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


pub fn collect_enum_values<E>() -> Vec<impl ModeEvent + IntoEnumIterator + Display> where E:IntoEnumIterator + Display + ModeEvent
{
    E::iter().collect()
}
