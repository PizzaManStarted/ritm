use ritm_core::{turing_graph::TuringMachineGraph, turing_machine::{reset, TuringIterator, TuringMachine}, turing_state::{TuringDirection, TuringTransitionMultRibbons}};


#[test]
fn execution_non_deter()
{
    let tm_graph = get_test_non_deter_graph();
    //println!("{:?}", tm_graph);

    let mut turing_machine = TuringMachine::new(tm_graph, String::from("00100")).unwrap();

    for tmp in turing_machine.as_iter() {
        println!("_______________\nExec. step ::\n{}", tmp)
    }


    

}



/// Gets a non determinist Turing machine graph.
/// Return a turing machine that checks if a given inputed value is of the form `x1y` with `|x| = |y|`.
fn get_test_non_deter_graph() -> TuringMachineGraph
{
    let q1 = &String::from("q1");
    let q2 = &String::from("q2");
    let mut graph = TuringMachineGraph::new(1).unwrap();

    graph.add_state(&q1);
    graph.add_state(&q2);


    // q_0 -> {ç, ç, => R, ç, R} -> q_1
    let mut transition = TuringTransitionMultRibbons::create(vec!('ç','ç'), vec!('ç'), vec!(TuringDirection::Right, TuringDirection::Right)).unwrap();
    graph.append_rule_state_by_name(&String::from("i"), transition.clone(), &q1).unwrap();

    // q_1 -> {0, _ => R, a, R} -> q_1
    transition = TuringTransitionMultRibbons::create(vec!('0','_'), vec!('a'), vec!(TuringDirection::Right, TuringDirection::Right)).unwrap();
    graph.append_rule_state_by_name(&q1, transition.clone(), &q1).unwrap();
    // q_1 -> {1, _ => R, a, R} -> q_1
    transition = TuringTransitionMultRibbons::create(vec!('1','_'), vec!('a'), vec!(TuringDirection::Right, TuringDirection::Right)).unwrap();
    graph.append_rule_state_by_name(&q1, transition.clone(), &q1).unwrap();

    // q_1 -> {1, _ => R, _, L} -> q_2
    transition = TuringTransitionMultRibbons::create(vec!('1','_'), vec!('_'), vec!(TuringDirection::Right, TuringDirection::Left)).unwrap();
    graph.append_rule_state_by_name(&q1, transition.clone(), &q2).unwrap();

    // q_2 -> {0, a => R, a, L} -> q_2
    transition = TuringTransitionMultRibbons::create(vec!('0','a'), vec!('a'), vec!(TuringDirection::Right, TuringDirection::Left)).unwrap();
    graph.append_rule_state_by_name(&q2, transition.clone(), &q2).unwrap();

    // q_2 -> {1, a => R, a, L} -> q_2
    transition = TuringTransitionMultRibbons::create(vec!('1','a'), vec!('a'), vec!(TuringDirection::Right, TuringDirection::Left)).unwrap();
    graph.append_rule_state_by_name(&q2, transition.clone(), &q2).unwrap();

    // q_2 -> {$, ç => N, ç, N} -> a
    transition = TuringTransitionMultRibbons::create(vec!('$','ç'), vec!('ç'), vec!(TuringDirection::None, TuringDirection::None)).unwrap();
    graph.append_rule_state_by_name(&q2, transition.clone(), &String::from("a")).unwrap();

    return graph;
    
}