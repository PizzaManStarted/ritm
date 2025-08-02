use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use colored::Colorize;


pub trait ModeEvent {
    fn print_help(&self);
    fn choose_option(&self);
}


#[derive(EnumIter)]
pub enum StartingMode {
    CreateTM,
    LoadTM
}

impl Display for StartingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            StartingMode::CreateTM => "Create new Turing Machine",
            StartingMode::LoadTM => "Load an existing Turing Machine",
        })
    }
}


#[derive(EnumIter)]
pub enum ModifyTuringMode {
    PrintSummary,
    AddState,
    AddTransitions,
    RemoveTransitions,
    RemoveState,
    SaveTM,
    FeedWord
}


impl Display for ModifyTuringMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ModifyTuringMode::PrintSummary => "Print a summary of the TM",
            ModifyTuringMode::AddState => "Add a state",
            ModifyTuringMode::AddTransitions => "Add one or multiple transition",
            ModifyTuringMode::RemoveTransitions => "Remove one or multiple transition",
            ModifyTuringMode::RemoveState => "Remove a state",
            ModifyTuringMode::SaveTM => "Save this TM as a file",
            ModifyTuringMode::FeedWord => "Feed a word and start executing this TM",
        })
    }
}




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



// _________________________________________________________________________

pub fn print_help<E>() where E:IntoEnumIterator + Display
{
    for (i, mode) in E::iter().enumerate() {
            println!("{}: {}", i.to_string().blue().bold(), mode.to_string().italic());
    }
}


pub fn collect_enum_values<E>() -> Vec<E> where E:IntoEnumIterator + Display + ModeEvent
{
    E::iter().collect()
}




impl ModeEvent for StartingMode {
    fn print_help(&self) {
        let tm_it = "Turing Machine".italic().bold();
        print!("-> ");
        match self {
            StartingMode::CreateTM => println!("Creates a new {tm_it} by specifying the {} and a {} (if any)", 
                                                "number of writting ribbons".purple(),
                                                "name".yellow() ),
            StartingMode::LoadTM => println!("Loads a new {tm_it} by specifying a {} to it from",
                                                "file path".purple()),
        }
        println!("")
    }

    fn choose_option(&self) {
        todo!()
    }
}


impl ModeEvent for ModifyTuringMode {
    fn print_help(&self) {
        let tm_it_bold = "Turing Machine".italic().bold();
        print!("-> ");
        match self {
            ModifyTuringMode::PrintSummary => println!("Prints a detailed overview of the current {tm_it_bold}"),
            ModifyTuringMode::AddState => println!("Adds a {} to the current {tm_it_bold}", "state".purple()),
            ModifyTuringMode::AddTransitions => println!("Adds one or multiple {} to the current {tm_it_bold}", "transitions".purple()),
            ModifyTuringMode::RemoveTransitions => println!("Removes one or multiple {} from the current {tm_it_bold}", "transitions".purple()),
            ModifyTuringMode::RemoveState => println!("Removes a {} from the current {tm_it_bold}", "state".purple()),
            ModifyTuringMode::SaveTM => println!("Saves the current {tm_it_bold} as a file"),
            ModifyTuringMode::FeedWord => println!("Feeds a word to the {tm_it_bold} and starts executing it"),
        }
        println!("")
    }

    fn choose_option(&self) {
        todo!()
    }
}