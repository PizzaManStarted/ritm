

use std::env;
use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use colored::Colorize;
use ritm_repl::modes::choice_modes::{collect_enum_values, print_help, ModeEvent, Modes};
use ritm_repl::modes::execute_mode::ExecuteTuringMode;
use ritm_repl::modes::modify_mode::ModifyTuringMode;
use ritm_repl::modes::starting_modes::StartingMode;
use ritm_repl::ripl_error::{print_error_help, RiplError};
use ritm_repl::DataStorage;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{DefaultEditor, Editor};
use strum::IntoEnumIterator;






fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let mut rl = DefaultEditor::new()?;
    
    // Clear screen
    rl.clear_screen().unwrap();

    // Adding a CTRL C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    
    ctrlc::set_handler(move || {
        // Sets the value to false, to signal to any running execution to stop
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Creates the data storage
    let mut storage = DataStorage {
        graph: None, 
        iterator: None, 
        is_running: running, 
        curr_path: {
            match env::current_dir() {
                Ok(dir) => {
                    Some(dir)
                },
                Err(e) => {
                    println!("{} {}", "Could not open the current directory: ".red(), e);
                    None
                },
            } 
        },
        clear_after_step: false,
        exec_mode: ritm_core::turing_machine::Mode::SaveAll
    };
    
    // Choose the first mode
    let mut curr_mode = Modes::Start;
    let mut prev_mode = Modes::Execute;

    loop {
        let status = match curr_mode {
            Modes::Start => {
                eval_loop::<StartingMode>(&mut rl, &mut curr_mode,  &mut prev_mode, &mut storage).unwrap()
            },
            Modes::Modify => {
                eval_loop::<ModifyTuringMode>(&mut rl, &mut curr_mode, &mut prev_mode, &mut storage).unwrap()
                
            },
            Modes::Execute => {
                eval_loop::<ExecuteTuringMode>(&mut rl, &mut curr_mode, &mut prev_mode, &mut storage).unwrap()
            },
        };

        if !status {
            break;
        }
    }
    Ok(())
}


fn eval_loop<E>(rl: &mut Editor<(), FileHistory>, 
                current_mode: &mut Modes, 
                previous_mode: &mut Modes,
                storage: &mut DataStorage) 
-> rustyline::Result<bool> where E : ModeEvent + IntoEnumIterator + Display
{
    let argument ;
    let mut need_help = false;

    
    // get possible commands
    let commands= collect_enum_values::<E>();
    
    if *previous_mode != *current_mode {
        print_help::<E>();
        *previous_mode = current_mode.clone();
    }

    let readline = rl.readline(">> ");
    
    // rl.clear_screen().unwrap();

    match readline {
        Ok(line) => {
            let line = line.trim();

            if line.is_empty() {
                return Ok(true);
            }

            // Adds the given string to the history for convenience
            rl.add_history_entry(line.to_string())?;

            // Split line
            let line_vec : Vec<&str> = line.split(" ").collect();

            // if the line starts with : "h " or "help " then the user is requesting help first
            if line.starts_with("h") || line.starts_with("help") {
                if line_vec.len() == 1 {
                    print_help::<E>();
                    return Ok(true);
                }
                else if line_vec.len() > 2 {
                    print_error_help(RiplError::ArgsNumberError { received: line_vec.len() - 1, expected: 1 });
                    return Ok(true);
                }
                argument = line_vec.get(1).unwrap().to_string();
                need_help = true;
            }
            else if line.eq("q") || line.eq("quit") || line.eq("exit") || line.eq("leave") {
                return Ok(false);
            }
            else if line.eq("cl") || line.eq("clear") {
                rl.clear_screen().unwrap();
                return Ok(true);
            }
            else {
                if line_vec.len() > 1 {
                    print_error_help(RiplError::UnknownCommandError { command: line_vec.get(0).unwrap().to_string() });
                    return Ok(true);
                }
                argument = line_vec.get(0).unwrap().to_string();
            }

            // Read requested nb
            let index_res = argument.parse();
            if let Err(_) = &index_res {
                print_error_help(RiplError::CouldNotParseStringIntError { value: argument });
                return Ok(true);
            }

            let index = index_res.unwrap();

            if index >= commands.len() {
                print_error_help(RiplError::OutOfRangeIndexError { index });
                return Ok(true);
            }
            // println!("Got index : {}", index.to_string().purple());
            if need_help {
                // rl.clear_screen().unwrap();
                commands.get(index).unwrap().print_help();
            }
            else {
                *current_mode = commands.get(index).unwrap().choose_option(rl, storage);
                // println!("{:?}", current_mode);
            }
            return Ok(true);
        },
        Err(ReadlineError::Interrupted) => {
            println!("CTRL-C");
            return Ok(false);
        },
        Err(ReadlineError::Eof) => {
            println!("CTRL-D");
            return Ok(false);
        },
        Err(err) => {
            println!("Error: {:?}", err);
            return Ok(false);
        }
    }
}