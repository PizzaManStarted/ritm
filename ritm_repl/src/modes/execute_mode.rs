use std::fmt::Display;

use colored::Colorize;
use strum_macros::EnumIter;

use crate::modes::choice_modes::ModeEvent;



#[derive(EnumIter)]
pub enum ExecuteTuringMode {
    NextStep,
    SkipSteps,
    AutoPlay,
    Finish,
    Stop,
    Reset,
    FeedWord,
    Summary,
}


impl Display for ExecuteTuringMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ExecuteTuringMode::NextStep => "Move to next step",
            ExecuteTuringMode::SkipSteps => "Skip multiple steps",
            ExecuteTuringMode::AutoPlay => "Execute at a given speed the TM",
            ExecuteTuringMode::Finish => "Finish the execution (can loop forever)",
            ExecuteTuringMode::Stop => "Stop the execution",
            ExecuteTuringMode::Reset => "Reset the execution",
            ExecuteTuringMode::FeedWord => "Feed a new word and reset",
            ExecuteTuringMode::Summary => "Print a summary of the execution",
        })
    }
}


impl ModeEvent for ExecuteTuringMode {
    fn print_help(&self) {
        todo!()
    }

    fn choose_option(&self, rl: &mut rustyline::Editor<(), rustyline::history::FileHistory>, storage: &mut crate::DataStorage) -> super::choice_modes::Modes {
        todo!()
    }
    
    fn get_help_color(str : colored::ColoredString) -> colored::ColoredString {
        str.yellow()
    }
}