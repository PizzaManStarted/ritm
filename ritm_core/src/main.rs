use ritm_core::{
    parser::*, turing_graph::TuringMachineGraph, turing_machine::{Mode, TuringMachines}, turing_state::{TuringDirection, TuringTransitionMultRibbons}
};

fn main() {
    let mut mt = parse_turing_machine_file("resources/turing2.tm".to_string()).unwrap();
    // let mut mt  = parse_turing_machine("".to_string()).unwrap();
    
    // FIX ME This is not okay !
    //mt.remove_state(&"a".to_string());
    //println!("{:?}", mt);
    // FIXME : Fix the bug of index being wrong when removing something like a !
    let mut exec= TuringMachines::new(mt, "0010011".to_string(), Mode::SaveAll).unwrap();

    //&et mut exec = TuringMachineExecutor::new(mt, "1010101010".to_string()).unwrap();
    for tmp in &mut exec {
        println!("_______________\nExec. step ::\n{}", tmp)
    }
    
    println!("{:?}", exec.get_turing_machine_graph());

}
