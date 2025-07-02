use ritm_core::{turing_errors::TuringError, turing_graph::TuringMachineGraph, turing_state::{TuringDirection::{self}, TuringTransitionMultRibbons}};


#[test]
fn create_graph_test() 
{
    let graph = TuringMachineGraph::new(2).unwrap();
    
    assert_eq!(*graph.name_index_hashmap.get("i").unwrap(), 0 as u8);
    assert_eq!(*graph.name_index_hashmap.get("a").unwrap(), 1 as u8);
    assert_eq!(*graph.name_index_hashmap.get("r").unwrap(), 2 as u8);

    expect_illegal_action_error(TuringMachineGraph::new(0));
    // Check the final states
    assert!(graph.get_state_from_name(&"a".to_string()).unwrap().is_final);
    assert!(graph.get_state_from_name(&"r".to_string()).unwrap().is_final);
    assert!(!graph.get_state_from_name(&"i".to_string()).unwrap().is_final);

    TuringMachineGraph::new(1).unwrap();

}

#[test]
fn delete_init_nodes_test() 
{
    let mut graph = TuringMachineGraph::new(2).unwrap();
       
    expect_illegal_action_error(graph.remove_state(&"i".to_string()));
    expect_illegal_action_error(graph.remove_state(&"a".to_string()));
    expect_illegal_action_error(graph.remove_state(&"r".to_string()));
}

#[test]
fn add_nodes() {
    let mut graph = TuringMachineGraph::new(1).unwrap();
    
    // Check they already exists
    assert_eq!(graph.add_state(&"i".to_string()), 0);
    assert_eq!(graph.add_state(&"a".to_string()), 1);
    assert_eq!(graph.add_state(&"r".to_string()), 2);

    // Add new ones
    assert_eq!(graph.add_state(&"b".to_string()), 3);
    assert_eq!(graph.add_state(&"c".to_string()), 4);
    assert_eq!(graph.add_state(&"d".to_string()), 5);
    // Check they got the correct index
    assert_eq!(graph.add_state(&"b".to_string()), 3);
    assert_eq!(graph.add_state(&"c".to_string()), 4);
    assert_eq!(graph.add_state(&"d".to_string()), 5);
}

#[test]
fn get_nodes_test() {
    let mut graph = TuringMachineGraph::new(1).unwrap();
    // Add new nodes
    assert_eq!(graph.add_state(&"b".to_string()), 3);
    assert_eq!(graph.add_state(&"c".to_string()), 4);
    assert_eq!(graph.add_state(&"d".to_string()), 5);

    // check they get be obtained
    assert_eq!(graph.get_state(3).unwrap().name.clone(), "b".to_string());
    assert_eq!(graph.get_state(4).unwrap().name.clone(), "c".to_string());
    assert_eq!(graph.get_state(5).unwrap().name.clone(), "d".to_string());

    // check they get be obtained
    assert_eq!(graph.get_state_from_name(&"b".to_string()).unwrap().name.clone(), "b".to_string());
    assert_eq!(graph.get_state_from_name(&"c".to_string()).unwrap().name.clone(), "c".to_string());
    assert_eq!(graph.get_state_from_name(&"d".to_string()).unwrap().name.clone(), "d".to_string());

    // Check they aren't final
    assert!(!graph.get_state_from_name(&"b".to_string()).unwrap().is_final);
    assert!(!graph.get_state_from_name(&"c".to_string()).unwrap().is_final);
    assert!(!graph.get_state_from_name(&"d".to_string()).unwrap().is_final);
}


#[test]
fn add_transition()
{
    let mut graph = TuringMachineGraph::new(2).unwrap();

    graph.append_rule_state_by_name(String::from("i"), 
                                TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap(), 
                                String::from("a")).expect("no errors were expected");

    // e, is not part of the graph
    expect_unk_name_error(graph.append_rule_state_by_name(String::from("e"), 
                                TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap(), 
                                String::from("a")));
    // o, is not part of the graph
    expect_unk_name_error(graph.append_rule_state_by_name(String::from("a"), 
                            TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap(), 
                            String::from("o")));
    // add e and o to the graph
    graph.add_state(&String::from("e"));
    graph.add_state(&String::from("o"));

    // Check that the transition didn't already exists
    // Check that the transition was really added
    if !graph.get_transition_indexes_by_name(&String::from("e"), &String::from("o"))
                    .expect("a value was expected here").is_empty() {
                        panic!("No values were expected");
    }
    // add transition
    graph.append_rule_state_by_name(String::from("e"), 
                                TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap(), 
                                String::from("o")).expect("no errors were expected");

    // Check that the transition was really added
    if graph.get_transition_indexes_by_name(&String::from("e"), &String::from("o"))
                    .expect("a value was expected here").is_empty() {
                        panic!("A value should be here");
    }

    // Add the same transition again
    // TODO check this
}


#[test]
fn add_transition_2_or_more_ribbon()
{
    let mut graph = TuringMachineGraph::new(2).unwrap();

    // well 

    
    let mut graph = TuringMachineGraph::new(3).unwrap();

    
}

fn expect_illegal_action_error<O>(res : Result<O, TuringError>)
{
    if let Err(e) = res {
        match e {
            TuringError::IllegalActionError { cause } => (),
            _ => panic!("Wrong error was returned"),
        }
    }
    else {
        panic!("Should have thrown an error")
    }
}


fn expect_unk_name_error<O>(res : Result<O, TuringError>)
{
    if let Err(e) = res {
        match e {
            TuringError::UnknownStateError { state_name } => (),
            _ => panic!("Wrong error was returned"),
        }
    }
    else {
        panic!("Should have thrown an error")
    }
}