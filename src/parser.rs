use pest::{iterators::Pair, Parser};
use std::fs;
use pest_derive::Parser;

use crate::{turing_errors::TuringError, turing_machine::TuringMachine, turing_state::{TuringDirection, TuringTransition}};

#[derive(Parser)]
#[grammar = "turing_machine.pest"]
pub struct TuringGrammar;



pub fn parse_turing_machine(file_path: String) -> Result<TuringMachine, TuringError>
{
    
    let unparsed_file = fs::read_to_string(&file_path).expect("cannot read file");

    let file: Pair<'_, Rule> = TuringGrammar::parse(Rule::turing_machine, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    
    let mut turing_machine: Option<TuringMachine> = None;

    let mut to_from_vars : Vec<String> = vec!();
    let mut transition: TuringTransition;
    let mut transitions: Vec<TuringTransition> = vec!();
    let mut nb_of_ribbons: Option<usize> = None;

    for turing_machine_rule in file.into_inner() {
        println!("______");
        
        match turing_machine_rule.as_rule() {
            Rule::rule => 
            {
                let mut from_var = String::new();
                
                for rule in turing_machine_rule.into_inner() {
                    println!("Showing : {}", rule.as_str());
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
                                if k != transition.get_number_of_ribbons() 
                                {
                                    panic!("Wrong nb of ribbons !"); // FIXME, wrap this in an error
                                }
                            }
                            else {
                                nb_of_ribbons = Some(transition.get_number_of_ribbons());
                            }
                            transitions.push(transition);
                        },
                        _ => unreachable!(), 
                    }
                    println!("\t_=_=_=_=_");
                }
            },
            // The file has ended, we can stop reading
            Rule::EOI => {},
            _ => unreachable!(),
        }
        
    }
    
    return Ok(turing_machine.unwrap());
}



fn parse_str_token(rule: Pair<Rule>) -> String
{
    match rule.as_rule() 
    {
        Rule::var | Rule::int | Rule::str => 
        {
            rule.into_inner().as_str().to_string()
        },
        _ => unreachable!(),
    }
}

fn parse_transition(rule: Pair<Rule>) -> TuringTransition
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
                    // FIXME AHHHHHHHHHH
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

    TuringTransition::create(chars_read, chars_written, directions)
}