use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};


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

impl StartingMode {
    pub fn print_help()
    {
        for (i, mode) in StartingMode::iter().enumerate() {
            println!("{}: {}", i, mode);
        }
    }
}




fn main() -> Result<()> {
    println!("Hello, world!");
    let mut rl = DefaultEditor::new()?;
    loop {
        StartingMode::print_help();
        // Print possible commands
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // Adds the given string to the history for convenience
                rl.add_history_entry(line.as_str())?;
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    Ok(())
}



