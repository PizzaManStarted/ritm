use ritm_core::{turing_graph::TuringMachineGraph, turing_machine::TuringMachines};
use rustyline::{history::FileHistory, Editor};

use crate::ripl_error::RiplError;

pub mod modes;

pub mod ripl_error;


pub struct DataStorage {
    pub graph : Option<TuringMachineGraph>,
    pub iterator : Option<TuringMachines>
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
                return Ok(l);
            },
            Err(e) => return Err(RiplError::CouldNotParseStringError { value: e.to_string() }),
        }
    }
}