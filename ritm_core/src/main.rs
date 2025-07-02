use ritm_core::{
    parser::*, turing_graph::TuringMachineGraph, turing_machine::{TuringIterator, TuringMachineWithRef}, turing_state::{TuringDirection, TuringTransition}
};

fn main() {
    let mut mt = parse_turing_machine_file("resources/turing2.tm".to_string()).unwrap();
    // let mut mt  = parse_turing_machine("".to_string()).unwrap();
    
    // FIX ME This is not okay !
    //mt.remove_state(&"a".to_string());
    //println!("{:?}", mt);
    // FIXME : Fix the bug of index being wrong when removing something like a !
    let mut exec= TuringMachineWithRef::new(&mt, "0010011".to_string()).unwrap();
    
    println!("{:?}", exec.get_writting_ribbons());

    //&et mut exec = TuringMachineExecutor::new(mt, "1010101010".to_string()).unwrap();
    for tmp in exec.as_iter() {
        println!("_______________\nExec. step ::\n{}", tmp)
    }
    
    println!("{:?}", mt);

    println!("{:?}", exec.get_state_pointer());
}
