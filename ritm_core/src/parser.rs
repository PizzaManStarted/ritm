use pest::{iterators::Pair, Parser};
use std::fs;
use pest_derive::Parser;

use crate::{turing_errors::TuringError, turing_graph::TuringMachineGraph, turing_machine::TuringMachine, turing_state::{TuringDirection, TuringTransitionMultRibbons}};

#[derive(Parser)]
#[grammar = "turing_machine.pest"]
pub struct TuringGrammar;


pub fn parse_turing_machine_file(file_path: String) -> Result<TuringMachineGraph, TuringError>
{
    let unparsed_file = fs::read_to_string(&file_path).expect("cannot read file");
    return parse_turing_machine(unparsed_file);
}


pub fn parse_turing_machine(turing_mach: String) -> Result<TuringMachineGraph, TuringError>
{
    let file: Pair<'_, Rule> = TuringGrammar::parse(Rule::turing_machine, &turing_mach)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    
    let mut turing_machine: Option<TuringMachineGraph> = None;

    let mut to_from_vars : Vec<String>;
    let mut transition: TuringTransitionMultRibbons;
    let mut transitions: Vec<TuringTransitionMultRibbons>;

    let mut nb_of_ribbons: Option<usize> = None;

    for turing_machine_rule in file.into_inner() {
        match turing_machine_rule.as_rule() {
            Rule::rule => 
            {
                transitions = vec!();
                to_from_vars = vec!();
                let mut from_var = String::new();
                
                for rule in turing_machine_rule.into_inner() {
                    // println!("Showing : {}", rule.as_str());
                    match rule.as_rule() {
                        // Get var1 & var2
                        Rule::var =>
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
                            transition = parse_transition(rule);
                            if let Some(k) = nb_of_ribbons
                            {
                                if k != transition.get_number_of_affected_ribbons() 
                                {
                                    return Err(TuringError::ArgsSizeTransitionError);
                                }
                            }
                            else {
                                nb_of_ribbons = Some(transition.get_number_of_affected_ribbons());
                            }
                            transitions.push(transition);
                        },
                        _ => unreachable!(), 
                    }
                };
                /* Add the colected transitions to the MT */

                // If the MT doesn't already exists, create it
                if let None = turing_machine 
                {
                    // With the collected number of ribbons
                    turing_machine = Some(TuringMachineGraph::new(transitions.get(0).expect("At least one rule should be given in a transition").get_number_of_affected_ribbons() as u8).unwrap());    
                }
                // If the MT existed
                if let Some(mt) = &mut turing_machine 
                {
                    // Adds all the collected transitions
                    for transition in transitions  
                    {
                        if let Err(e) = mt.append_rule_state_by_name(to_from_vars.get(0).expect("A name state was expected").to_string(), 
                                            transition, 
                                            to_from_vars.get(1).expect("Two name states were expected").to_string())
                        {
                            return Err(e);
                        }
                    }
                }
            },
            // The file has ended, we can stop reading
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
        Rule::var | Rule::int | Rule::str => 
        {
            rule.into_inner().as_str().trim().to_string()
        },
        _ => unreachable!(),
    }
}

fn parse_transition(rule: Pair<Rule>) -> TuringTransitionMultRibbons
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
                        Rule::char =>
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

    TuringTransitionMultRibbons::create(chars_read, chars_written, directions).unwrap()
}