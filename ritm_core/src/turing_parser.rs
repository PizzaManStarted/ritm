use pest::{iterators::Pair, Parser};
use std::fs;
use pest_derive::Parser;

use crate::{turing_errors::TuringError, turing_graph::TuringMachineGraph, turing_state::{TuringDirection, TuringTransitionMultRibbons}};

#[derive(Parser)]
#[grammar = "turing_machine.pest"]
pub struct TuringGrammar;


/// Parses a turing machine graph.
/// 
/// Important to note that if the given string is empty, then an empty [TuringMachineGraph] with a *k* of 1 is returned.
pub fn parse_turing_graph_file(file_path: String) -> Result<TuringMachineGraph, TuringError>
{
    if file_path.trim().is_empty() {
        return TuringMachineGraph::new(1);
    }
    let unparsed_file = fs::read_to_string(&file_path).expect("cannot read file");
    return parse_turing_graph_string(unparsed_file);
}


pub fn parse_turing_graph_string(turing_mach: String) -> Result<TuringMachineGraph, TuringError>
{
    let file = TuringGrammar::parse(Rule::turing_machine, &turing_mach);
    if let Err(e) = file {
        // TODO Add help here
        return Err(TuringError::ParseError { reason: String::from("Couldn't parse") });
    }
    let file = file.unwrap().next().unwrap(); // get and unwrap the `file` rule; never fails
    
    let mut turing_machine: Option<TuringMachineGraph> = None;



    for turing_machine_rule in file.into_inner() {
        // Inside the 'turing_machine' rule, only two things can be matched : a transition (or multiple in one), and EOI
        match turing_machine_rule.as_rule() {
            // For every rule matched :
            Rule::transition => 
            {
                let (from_var, transitions, to_var) = parse_transition(turing_machine_rule).unwrap();

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
                    let var1 = mt.add_state(&from_var);
                    let var2 = mt.add_state(&to_var);
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





/// Parses a string containing a transition of the form : 
/// * `q_i { transition } q_j`
/// * Or even :  `q_i { transition_0 | ... | transition_n } q_j`
/// 
/// Where each `transition` follows the form :  `a_0, a_1, ..., a_{n-1} -> D_0, b_1, D_1, b_2, D_2, ..., b_{n-1}, D_{n-1}`.
/// For more information look at the documentation of the structure [TuringTransitionMultRibbons].
/// 
/// When giving multiple transitions, each one must affect the same number of ribbons or an error will be returned.
pub fn parse_transition_string(to_parse: String) -> Result<(String, Vec<TuringTransitionMultRibbons>, String), TuringError>
{
    let parsed = TuringGrammar::parse(Rule::transition_only, &to_parse);
    if let Err(e) = parsed {
        // TODO Add help here
        return Err(TuringError::ParseError { reason: String::from("Couldn't parse") });
    }
    parse_transition(parsed.unwrap().next().unwrap())
}



/// Parses a string containing the content of a transition of the form : `a_0, a_1, ..., a_{n-1} -> D_0, b_1, D_1, b_2, D_2, ..., b_{n-1}, D_{n-1}`
/// For more information look at the documentation of the structure [TuringTransitionMultRibbons]
pub fn parse_transition_content_string(transition: String) -> Result<TuringTransitionMultRibbons, TuringError>
{
    let parsed = TuringGrammar::parse(Rule::turing_machine, &transition);
    if let Err(e) = parsed {
        // TODO Add help here
        return Err(TuringError::ParseError { reason: String::from("Couldn't parse") });
    }
    todo!("test");
}





fn parse_transition(rule: Pair<Rule>) -> Result<(String, Vec<TuringTransitionMultRibbons>, String), TuringError>
{
    let mut transitions = vec!();
    let mut to_var = String::new();
    let mut from_var = String::new();
    
    for rule in rule.into_inner() {
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
                    to_var = parse_str_token(rule);
                }
            },
            // Read all transitions
            Rule::transition_content => {
                // Add the transition
                let tr_res = parse_transition_content(rule);
                if let Err(e) = tr_res {
                    return Err(e); // FIXME wrap with parsing error
                }
                transitions.push(tr_res.unwrap());
            },
            _ => unreachable!(), 
        }
    };

    Ok((from_var, transitions, to_var))
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



fn parse_transition_content(rule: Pair<Rule>) -> Result<TuringTransitionMultRibbons, TuringError>
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