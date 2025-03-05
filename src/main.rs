use turingrs::{turing_machine::{TuringMachine, TuringMachineExecutor}, turing_state::{TuringDirection, TuringTransition}};

fn main() 
{
    println!("Hello, world!");
    

    let mut turing_mach: TuringMachine = TuringMachine::new(2);
    
    turing_mach.add_rule_state("q_0".to_string(), TuringTransition::new('ç', 
                                                                                TuringDirection::Right, 
                                                                         vec!(('ç', TuringDirection::Left), ('ç', TuringDirection::Left))), 
                                                                                          "q_a".to_string()).expect("h");
    
    let mut tm_exec = TuringMachineExecutor::new(mt, "Hi".to_string());
    
    println!("{:?}", turing_mach);
}
