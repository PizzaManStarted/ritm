use pest::Parser;
use std::fs;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "turing_machine.pest"]
pub struct TuringGrammar;



pub fn test()
{
    
    let unparsed_file = fs::read_to_string("resources/turing2.tm").expect("cannot read file");

    let file = TuringGrammar::parse(Rule::turing_machine, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    

    for rule in file.into_inner() {
        println!("______");
        match rule.as_rule() {
            Rule::rule => 
            {
                for rule in rule.into_inner() {
                    println!("\t_=_=_=_=_");
                    match rule.as_rule() {
                        Rule::var => todo!("it me"),
                        Rule::transition => todo!("it me !!"),
                        _ => unreachable!(), 
                    }
                }
            },
            _ => unreachable!(),
        }
    }
}