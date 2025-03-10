use pest::{iterators::Pair, Parser};
use std::fs;
use pest_derive::Parser;

use crate::{turing_errors::TuringError, turing_machine::TuringMachine, turing_state::TuringTransition};

#[derive(Parser)]
#[grammar = "turing_machine.pest"]
pub struct TuringGrammar;



pub fn parse_turing_machine(file_path: String) -> Result<TuringMachine, TuringError>
{
    
    let unparsed_file = fs::read_to_string(&file_path).expect("cannot read file");

    let file: Pair<'_, Rule> = TuringGrammar::parse(Rule::turing_machine, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    
    let turing_mac: Option<TuringMachine> = None;

    for turing_machine in file.into_inner() {
        println!("______");
        match turing_machine.as_rule() {
            Rule::rule => 
            {
                let mut var1 = String::new();
                let mut var2 = String::new();

                for rule in turing_machine.into_inner() {
                    println!("\t_=_=_=_=_");
                    match rule.as_rule() {
                        // Get var1 & var2
                        Rule::var =>
                        {
                            if var1.eq("")
                            {
                                var1 = rule.into_inner().as_str().to_string();
                            }
                            else if var2.eq("") {
                                var2 = rule.into_inner().as_str().to_string();
                            }
                        },
                        Rule::transition => {},
                        _ => unreachable!(), 
                    }
                }
                println!("q1: {}, q2: {}", var1, var2);
                
            },
            _ => unreachable!(),
        }
    }
    return Ok(turing_mac.unwrap());
}