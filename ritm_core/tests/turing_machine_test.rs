use ritm_core::{turing_graph::TuringMachineGraph, turing_machine::{Mode, TuringExecutionSteps, TuringMachines}, turing_state::{TuringDirection, TuringStateType, TuringTransitionMultRibbons}};


#[test]
fn save_all_accept()
{
    let tm_graph = get_test_non_deter_graph();
    //println!("{:?}", tm_graph);

    // let mut turing_machine = TuringMachine::new(tm_graph, String::from("010"), Mode::SaveAll).unwrap();

    let mut turing_machine = TuringMachines::new(tm_graph, String::from("11111"), Mode::SaveAll).unwrap();

    // let mut turing_machine = TuringMachines::new(get_smaller_non_deter_graph(), String::from("0000000000000001"), Mode::SaveAll).unwrap();
    let mut saved_state = None;
    let mut counter = 0;
    for steps in &mut turing_machine {
        println!("_______________\nExec. step ::\n{}", steps);
        counter += 1;
        
        if counter == 1000 {
            return;
        }
        saved_state = Some(steps);
    }

    if let Some(state) = saved_state {
        match state {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons:_ } => panic!("Wrong outcome"),
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_ } => {
                assert_eq!(TuringStateType::Accepting, reached_state.state_type)
            },
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, read_ribbon:_, write_ribbons:_ } => panic!("Wrong outcome"),
        }
    }
}

#[test]
fn save_all_not_accept()
{
    let tm_graph = get_test_non_deter_graph();

    let mut turing_machine = TuringMachines::new(tm_graph, String::from("0100"), Mode::SaveAll).unwrap();

    let mut saved_state = None;
    for steps in &mut turing_machine {
        println!("_______________\nExec. step ::\n{}", steps);
        saved_state = Some(steps);
    }

    if let Some(state) = saved_state {
        match state {
            TuringExecutionSteps::FirstIteration { init_state:_, init_read_ribbon:_, init_write_ribbons:_ } => panic!("Wrong outcome"),
            TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_ } => {
                assert_ne!(TuringStateType::Accepting, reached_state.state_type)
            },
            TuringExecutionSteps::Backtracked { previous_state:_, reached_state:_, read_ribbon:_, write_ribbons:_ } => panic!("Wrong outcome"),
        }
    }
}

#[test]
fn stop_after()
{
   let mut turing_machine = get_small_inf_machine(Mode::StopAfter(10000));
    
    let mut counter = 0;
    for _steps in &mut turing_machine {
        // println!("_______________\nExec. step ::\n{}", steps);
        counter += 1;
    }

    assert_eq!(counter, 10000)
}

#[test]
fn stop_first_reject()
{
    let tm_graph = get_test_non_deter_graph();

    let mut turing_machine = TuringMachines::new(tm_graph, String::from("010"), Mode::StopFirstReject).unwrap();

    // let mut turing_machine = TuringMachines::new(get_smaller_non_deter_graph(), String::from("0000000000000001"), Mode::SaveAll).unwrap();
    let mut counter = 0;
    let mut last_step = None;
    for steps in &mut turing_machine {
        counter += 1;

        if counter == 1000 {
            return;
        }
        
        last_step = Some(steps);
        
    }

    if let Some(step) = last_step {
        if let TuringExecutionSteps::TransitionTaken { previous_state:_, reached_state, transition_index_taken:_, transition_taken:_, read_ribbon:_, write_ribbons:_ } = step {
            assert_ne!(reached_state.state_type, TuringStateType::Accepting);
            return;
            
        }
    }
    panic!("The iteration didn't stop like it was supposed to");
}



/// Gets a graph that forces one complete descent before doing one backtracking and finishing.
/// Feed it `0...0` in order for it to suceed
fn get_smaller_non_deter_graph() -> TuringMachineGraph 
{
    let q2 = &String::from("q2");
    let mut graph = TuringMachineGraph::new(1).unwrap();

    graph.add_state(&q2);

    let mut transition = TuringTransitionMultRibbons::create(vec!('ç','ç'), vec!('ç'), vec!(TuringDirection::Right, TuringDirection::Right)).unwrap();
    graph.append_rule_state_by_name(&String::from("i"), transition.clone(), &q2).unwrap();

    transition = TuringTransitionMultRibbons::create(vec!('0','_'), vec!('_'), vec!(TuringDirection::Right, TuringDirection::None)).unwrap();
    graph.append_rule_state_by_name(&q2, transition.clone(), &q2).unwrap();


    transition = TuringTransitionMultRibbons::create(vec!('0','_'), vec!('_'), vec!(TuringDirection::Right, TuringDirection::None)).unwrap();
    graph.append_rule_state_by_name(&q2, transition.clone(), &String::from("a")).unwrap();

    graph
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

fn get_small_inf_machine(mode: Mode) -> TuringMachines 
{
    let q1 = &String::from("q1");

    let mut graph = TuringMachineGraph::new(1).unwrap();
    graph.add_state(q1);
    
    // q_0 -> {ç, ç, => R, ç, R} -> q_1
    let transition = TuringTransitionMultRibbons::create(vec!('ç','ç'), vec!('ç'), vec!(TuringDirection::Right, TuringDirection::Right)).unwrap();
    graph.append_rule_state_by_name(&String::from("i"), transition.clone(), &q1).unwrap();

    // q_1 -> {1, _, => N, _, N} -> q_1
    let transition = TuringTransitionMultRibbons::create(vec!('1','_'), vec!('_'), vec!(TuringDirection::None, TuringDirection::None)).unwrap();
    graph.append_rule_state_by_name(&q1, transition.clone(), &q1).unwrap();
    
    TuringMachines::new(graph, String::from("1"), mode).unwrap()
}