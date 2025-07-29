use ritm_core::{parser::{parse_transition_string, parse_turing_graph_string}, turing_machine::TuringMachines, turing_state::{TuringDirection, TuringTransitionMultRibbons}};


#[test]
fn test_parse_mt_valid()
{

    let machine = String::from("q_i {ç, ç -> R, ç, R} q_1;
                                        q1 {0, _ -> R, a, R 
                                          | 1, _ -> R, a, R} q1;
                                        
                                        q_1 {1, _ -> R, _, L} q_2;
                                        
                                        q_2 { 0, a -> R, a, L
                                            | 1, a -> R, a, L} q_2;
                                        
                                        q_2 {$, ç -> N, ç, N} q_a;");

    let res = parse_turing_graph_string(machine);


    let mut t = TuringMachines::new(res.unwrap(), String::from("01100"), ritm_core::turing_machine::Mode::StopAfter(2000)).unwrap();

    // println!("{:?}", res);
    for steps in &mut t {
        println!("{}", steps);   
    }
}


#[test]
fn test_parse_transition_valid_mult()
{
    let transition_str = String::from("q_i { 0, a -> R, a, L
                                            | 1, b -> N, p, R} q_2");

    let (from, transitions, to) = parse_transition_string(transition_str).unwrap();

    assert_eq!(String::from("i"), from);
    assert_eq!(String::from("2"), to);

    assert_eq!(transitions.len(), 2);
    assert_eq!(transitions[0], TuringTransitionMultRibbons::new(vec!('0', 'a'), TuringDirection::Right, vec!(('a', TuringDirection::Left))));
    assert_eq!(transitions[1], TuringTransitionMultRibbons::new(vec!('1', 'b'), TuringDirection::None, vec!(('p', TuringDirection::Right))));
}

#[test]
fn test_parse_transition_valid_single()
{
    let transition_str = String::from("qi { 0, a -> R, a, L } q2");

    let (from, transitions, to) = parse_transition_string(transition_str).unwrap();

    assert_eq!(String::from("i"), from);
    assert_eq!(String::from("2"), to);

    assert_eq!(transitions.len(), 1);
    assert_eq!(transitions[0], TuringTransitionMultRibbons::new(vec!('0', 'a'), TuringDirection::Right, vec!(('a', TuringDirection::Left))));
}


#[test]
fn test_parse_transition_fail()
{
    let transition_str = String::from("q_2 { 0, a -> R, a, L
                                            | 1 -> R, a, L} q_2");

    if let Ok(t) = parse_transition_string(transition_str) {
        panic!("The parser should have returned an error not this value:  {:?}",t)
    }

    let transition_str = String::from("q_2 { 0, a -> R, a, L
                                            | 1, a-> R, a, L} q_2;");

    if let Ok(t) = parse_transition_string(transition_str) {
        panic!("The parser should have returned an error not this value:  {:?}",t)
    }

    let transition_str = String::from("");

    if let Ok(t) = parse_transition_string(transition_str) {
        panic!("The parser should have returned an error not this value:  {:?}",t)
    }
}