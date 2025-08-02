
use colored::Colorize;
use ritm_repl::choice_modes::{collect_enum_values, print_help, ExecuteTuringMode, ModeEvent, ModifyTuringMode, StartingMode};
use ritm_repl::ripl_error::{print_error_help, RiplError};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};





fn main() -> rustyline::Result<()> 
{
    let mut rl = DefaultEditor::new()?;
    rl.clear_screen().unwrap();
    
    // Collect all possible modes
    let starting_modes = collect_enum_values::<StartingMode>();

    // Clear terminal
    rl.clear_screen().unwrap();

    loop {
        let mut argument = String::new();
        let mut need_help = false;

        print_help::<StartingMode>();
        // Print possible commands
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let line = line.trim();


                // Adds the given string to the history for convenience
                rl.add_history_entry(line.to_string())?;

                // Split line
                let line_vec : Vec<&str> = line.split(" ").collect();
                

                // if the line starts with : "h " or "help " then the user is requesting help first
                if line.starts_with("h ") || line.starts_with("help ") {
                    if line_vec.len() > 2 {
                        print_error_help(RiplError::ArgsNumberError { received: line_vec.len() - 1, expected: 1 });
                        continue;
                    }
                    argument = line_vec.get(1).unwrap().to_string();
                    need_help = true;
                }
                else if line.eq("q") || line.eq("quit") || line.eq("exit") || line.eq("leave") {
                    break;
                }
                else if line.eq("cl") || line.eq("clear") {
                    rl.clear_screen().unwrap();
                }
                else {
                    if line_vec.len() > 1 {
                        print_error_help(RiplError::UnknownCommandError { command: line.to_string() });
                        continue;
                    }
                    argument = line_vec.get(0).unwrap().to_string();
                }

                // Read requested nb
                let index_res = argument.parse();
                if let Err(_) = &index_res {
                    print_error_help(RiplError::CouldNotParseStringError { value: argument });
                    continue;
                }

                let index = index_res.unwrap();

                if index >= starting_modes.len() {
                    print_error_help(RiplError::OutOfRangeIndexError { index });
                    continue;
                }
                // println!("Got index : {}", index.to_string().purple());
                if need_help {
                    starting_modes.get(index).unwrap().print_help();
                }
                else {
                    starting_modes.get(index).unwrap().choose_option();
                }

                // println!("Line: {}", line);
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


fn parse_string_to_int(value: String) -> Result<usize, RiplError>
{
    let index = value.parse();
    if let Err(_) = &index {
        return Err(RiplError::CouldNotParseStringError { value: value.to_string() });
    }
    return Ok(index.unwrap());
}

