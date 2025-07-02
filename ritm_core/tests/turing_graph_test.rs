use ritm_core::{turing_errors::TuringError, turing_graph::TuringMachineGraph, turing_state::{TuringDirection::{self}, TuringTransition}};


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
    assert_eq!(graph.get_state(3).unwrap().name.clone().unwrap(), "b".to_string());
    assert_eq!(graph.get_state(4).unwrap().name.clone().unwrap(), "c".to_string());
    assert_eq!(graph.get_state(5).unwrap().name.clone().unwrap(), "d".to_string());

    // check they get be obtained
    assert_eq!(graph.get_state_from_name(&"b".to_string()).unwrap().name.clone().unwrap(), "b".to_string());
    assert_eq!(graph.get_state_from_name(&"c".to_string()).unwrap().name.clone().unwrap(), "c".to_string());
    assert_eq!(graph.get_state_from_name(&"d".to_string()).unwrap().name.clone().unwrap(), "d".to_string());

    // Check they aren't final
    assert!(!graph.get_state_from_name(&"b".to_string()).unwrap().is_final);
    assert!(!graph.get_state_from_name(&"c".to_string()).unwrap().is_final);
    assert!(!graph.get_state_from_name(&"d".to_string()).unwrap().is_final);
}


#[test]
fn transition_creation_test() 
{
    expect_wrong_args_error(TuringTransition::create(vec!('ç'),
                                                         vec!(),
                                                         vec!(TuringDirection::Left)));

    expect_wrong_args_error(TuringTransition::create(vec!(),
                                                         vec!('ç'),
                                                         vec!(TuringDirection::Left)));

    expect_wrong_args_error(TuringTransition::create(vec!('ç'),
                                                         vec!('ç'),
                                                         vec!()));
}

#[test]
fn add_transition_existing_node()
{
    let mut graph = TuringMachineGraph::new(1).unwrap();

    
}


#[test]
fn add_transition_not_existing_nodes() {
    let mut graph = TuringMachineGraph::new(1).unwrap();
    // Add new nodes
    assert_eq!(graph.add_state(&"b".to_string()), 3);
    assert_eq!(graph.add_state(&"c".to_string()), 4);

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

fn expect_wrong_args_error<O>(res : Result<O, TuringError>)
{
    if let Err(e) = res {
        match e {
            TuringError::ArgsSizeTransitionError => (),
            _ => panic!("Wrong error was returned"),
        }
    }
    else {
        panic!("Should have thrown an error")
    }
}