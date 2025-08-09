use std::{fmt::Debug, path::PathBuf, str::FromStr, sync::{atomic::AtomicBool, Arc}};

use ritm_core::{turing_graph::TuringMachineGraph, turing_machine::TuringMachines, turing_parser::parse_transition_string, turing_state::TuringTransitionMultRibbons};
use rustyline::{history::FileHistory, Editor};

use crate::ripl_error::RiplError;

pub mod modes;

pub mod ripl_error;


pub struct DataStorage {
    pub graph : Option<TuringMachineGraph>,
    pub iterator : Option<TuringMachines>,
    pub is_running : Arc<AtomicBool>,
    pub curr_path : Option<PathBuf>
}


pub fn query_usize(rl: &mut Editor<(), FileHistory>, query: String) -> Result<usize, RiplError>
{
    println!("{}", query);
    loop {
        let readline = rl.readline("==> ");
        match readline {
            Ok(l) => {
                let l = l.trim().to_string();
                if l.is_empty() {
                    continue;
                }

                rl.add_history_entry(l.to_string()).unwrap();

                // Read requested nb
                let index_res = l.parse();
                if let Err(_) = &index_res {
                    return Err(RiplError::CouldNotParseStringIntError { value: l });
                }
                return Ok(index_res.unwrap());
            },
            Err(e) => return Err(RiplError::CouldNotParseStringError { value: e.to_string() }),
        }
    }
}

pub fn query_prim<E: FromStr>(rl: &mut Editor<(), FileHistory>, query: String) -> Result<E, RiplError> where <E as FromStr>::Err: Debug
{
    // FIXME: fix the error returned !!!
    println!("{}", query);
    loop {
        let readline = rl.readline("==> ");
        match readline {
            Ok(l) => {
                let l = l.trim().to_string();
                if l.is_empty() {
                    continue;
                }

                rl.add_history_entry(l.to_string()).unwrap();

                // Read requested nb
                let index_res = l.parse();
                if let Err(_) = &index_res {
                    return Err(RiplError::CouldNotParseStringIntError { value: l });
                }
                return Ok(index_res.unwrap());
            },
            Err(e) => return Err(RiplError::CouldNotParseStringError { value: e.to_string() }),
        }
    }
}

pub fn query_float(rl: &mut Editor<(), FileHistory>, query: String) -> Result<f32, RiplError>
{
    println!("{}", query);
    loop {
        let readline = rl.readline("==> ");
        match readline {
            Ok(l) => {
                let l = l.trim().to_string();
                if l.is_empty() {
                    continue;
                }

                rl.add_history_entry(l.to_string()).unwrap();

                // Read requested nb
                let index_res = l.parse();
                if let Err(_) = &index_res {
                    return Err(RiplError::CouldNotParseStringIntError { value: l });
                }
                return Ok(index_res.unwrap());
            },
            Err(e) => return Err(RiplError::CouldNotParseStringError { value: e.to_string() }),
        }
    }
}


pub fn query_string(rl: &mut Editor<(), FileHistory>, query: String) -> Result<String, RiplError>
{
    println!("{}", query);
    loop {
        let readline = rl.readline("==> ");
        match readline {
            Ok(l) => {
                let l = l.trim().to_string();
                if l.is_empty() {
                    continue;
                }
                rl.add_history_entry(l.to_string()).unwrap();
                return Ok(l.trim().to_string());
            },
            Err(e) => return Err(RiplError::CouldNotParseStringError { value: e.to_string() }),
        }
    }
}