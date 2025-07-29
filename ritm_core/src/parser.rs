use pest::{iterators::Pair, Parser};
use std::fs;
use pest_derive::Parser;

use crate::{turing_errors::TuringError, turing_graph::TuringMachineGraph, turing_state::{TuringDirection, TuringTransitionMultRibbons}};

#[derive(Parser)]
#[grammar = "turing_machine.pest"]
pub struct TuringGrammar;


pub fn parse_turing_machine_file(file_path: String) -> Result<TuringMachineGraph, TuringError>
{
    let unparsed_file = fs::read_to_string(&file_path).expect("cannot read file");
    return parse_turing_machine_string(unparsed_file);
}


pub fn parse_turing_machine_string(turing_mach: String) -> Result<TuringMachineGraph, TuringError>
{
    let file = TuringGrammar::parse(Rule::turing_machine, &turing_mach);
    if let Err(e) = file {
        // TODO Add help here
        return Err(TuringError::ParseError { reason: String::from("Couldn't parse") });
    }
    let file = file.unwrap().next().unwrap(); // get and unwrap the `file` rule; never fails
    
    let mut turing_machine: Option<TuringMachineGraph> = None;

    let mut to_from_vars : Vec<String>;
    let mut transitions: Vec<TuringTransitionMultRibbons>;


    for turing_machine_rule in file.into_inner() {
        // Inside the 'turing_machine' rule, only two things can be matched : rule, and EOI
        match turing_machine_rule.as_rule() {
            // For every rule matched :
            Rule::rule => 
            {
                transitions = vec!();
                to_from_vars = vec!();
                let mut from_var = String::new();
                
                for rule in turing_machine_rule.into_inner() {
                    // Inside a rule, there are :
                    // * two state names (var1 & var2)
                    // * one or more transitions btw them
                    match rule.as_rule() {
                        // Get var1 & var2
                        Rule::state_name =>
                        {
                            if from_var.eq("")
                            {
                                from_var = parse_str_token(rule);
                            }
                            else {
                                to_from_vars.push(from_var.clone());
                                to_from_vars.push(parse_str_token(rule));
                            }
                        },
                        // Read all transitions
                        Rule::transition => {
                            // Add the transition
                            let tr_res = parse_transition(rule);
                            if let Err(e) = tr_res {
                                return Err(e); // FIXME wrap with parsing error
                            }
                            transitions.push(tr_res.unwrap());
                        },
                        _ => unreachable!(), 
                    }
                };
                /* Add the colected transitions to the MT */

                // If the MT doesn't already exists, create it
                if let None = turing_machine 
                {
                    // With the collected number of ribbons
                    let tm = TuringMachineGraph::new(transitions.get(0).expect("At least one rule should be given in a transition").get_number_of_affected_ribbons() - 1);

                    if let Err(e) = tm {
                        // FIXME wrap with parsing error
                        return Err(e);
                    }
                    turing_machine = Some(tm.unwrap());
                }
                // If the MT existed
                if let Some(mt) = &mut turing_machine 
                {
                    // Add the states to the mt (if they didn't already exists)
                    // and get their index
                    let var1 = mt.add_state(&to_from_vars.get(0).unwrap().to_string());
                    let var2 = mt.add_state(&to_from_vars.get(1).unwrap().to_string());
                    // Adds all the collected transitions for these states
                    for transition in transitions  
                    {
                        if let Err(e) = mt.append_rule_state(var1, transition, var2) {
                            return Err(e);
                        }
                    }
                }
            },
            // The file has ended, we can stop reading/parsing
            Rule::EOI => {},
            _ => unreachable!(),
        }
    }
    match turing_machine {
        Some(t) => return Ok(t),
        // If no parse value was given, simply return a read only one
        None => return Ok(TuringMachineGraph::new(1).unwrap()),
    }
}



fn parse_str_token(rule: Pair<Rule>) -> String
{
    match rule.as_rule() 
    {
        Rule::state_name | Rule::str => 
        {
            rule.into_inner().as_str().trim().to_string()
        },
        _ => unreachable!(),
    }
}

fn parse_transition(rule: Pair<Rule>) -> Result<TuringTransitionMultRibbons, TuringError>
{
    let mut chars_read: Vec<char> = vec!();
    let mut directions: Vec<TuringDirection> = vec!();
    let mut chars_written: Vec<char> = vec!();

    // Parse all the informations
    for transition_rule in rule.into_inner()
    {
        match transition_rule.as_rule(){
            Rule::to_read => 
            {
                // Parse all the characters to read
                for chars_rule in transition_rule.into_inner() {
                    // turns the rule into a string, then gets the first (and only) char
                    chars_read.push(chars_rule.as_str().chars().next().unwrap());
                }
            },
            Rule::to_write_move => 
            {
                for write_move_rule in transition_rule.into_inner()
                {
                    match write_move_rule.as_rule() {
                        Rule::dir_left => 
                        {
                            directions.push(TuringDirection::Left);
                        },
                        Rule::dir_right => 
                        {
                            directions.push(TuringDirection::Right);
                        },
                        Rule::dir_none => 
                        {
                            directions.push(TuringDirection::None);
                        },
                        Rule::char | Rule::special_chars =>
                        {
                            chars_written.push(write_move_rule.as_str().chars().next().unwrap());
                        },
                        _ => unreachable!(),
                    };
                    
                }
            },
            _ => unreachable!(),
        }
    }

    TuringTransitionMultRibbons::create(chars_read, chars_written, directions)

    
}