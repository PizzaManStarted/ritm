
use ritm_repl::choice_modes::{print_help, ExecuteTuringMode, ModifyTuringMode, StartingMode};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};





fn main() -> Result<()> 
{
    let mut rl = DefaultEditor::new()?;
    rl.clear_screen().unwrap();
    // Collect all possible modes

    loop {
        print_help::<StartingMode>();


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



