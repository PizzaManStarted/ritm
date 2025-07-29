use ritm_core::{
    parser::*, turing_graph::TuringMachineGraph, turing_machine::{Mode, TuringMachines}, turing_state::{TuringDirection, TuringTransitionMultRibbons}
};

fn main() {
    let machine = String::from("q_i {ç, ç -> R, ç, R} q_1;
                                        q1 {0, _ -> R, a, R 
                                          | 1, _ -> R, a, R} q1;
                                        
                                        q_1 {1, _ -> R, _, L} q_2;
                                        
                                        q_2 { 0, a -> R, a, L
                                            | 1, a -> R, a, L} q_2;
                                        
                                        q_2 {$, ç -> N, ç, N} q_a;");

    let res = parse_turing_graph_string(machine);


    let mut t = TuringMachines::new(res.unwrap(), String::from("01101"), ritm_core::turing_machine::Mode::StopAfter(20000)).unwrap();

    // println!("{:?}", res);
    for steps in &mut t {
        println!("{}", steps);   
    }

}
