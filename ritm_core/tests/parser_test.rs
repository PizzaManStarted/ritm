use ritm_core::{parser::{parse_transition_string, parse_turing_graph_string}, turing_machine::TuringMachines};


#[test]
fn test_parse()
{

    let machine = String::from("q_i {ç, ç -> R, ç, R} q_1;
                                        q1 {0, _ -> R, a, R 
                                          | 1, _ -> R, a, R} q1;
                                        
                                        q_1 {1, _ -> R, _, L} q_2;
                                        
                                        q_2 { 0, a -> R, a, L
                                            | 1, a -> R, a, L} q_2;
                                        
                                        q_2 {$, ç -> N, ç, N} q_a;");

    let res = parse_turing_graph_string(machine);


    let mut t = TuringMachines::new(res.unwrap(), String::from("0100"), ritm_core::turing_machine::Mode::StopAfter(2000)).unwrap();

    // println!("{:?}", res);
    for steps in &mut t {
        println!("{}", steps);   
    }
}

#[test]
fn test_parse_transition()
{
    let transition_str = String::from("q_2 { 0, a -> R, a, L
                                            | 1, a -> R, a, L} q_2");


    println!("{:?}", parse_transition_string(transition_str));
}