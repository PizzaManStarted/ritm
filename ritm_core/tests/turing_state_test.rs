use ritm_core::{turing_errors::TuringError, turing_state::*};


// ________________________________________ Transitions tests ______________________________
#[test]
fn transition_creation_test() 
{
    let t1 =  TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap();

    expect_wrong_args_error(TuringTransitionMultRibbons::create(vec!('ç', 'ç'),
                                                         vec!(),
                                                         vec!(TuringDirection::Left)));

    expect_wrong_args_error(TuringTransitionMultRibbons::create(vec!(),
                                                         vec!('ç'),
                                                         vec!(TuringDirection::Left)));

    expect_wrong_args_error(TuringTransitionMultRibbons::create(vec!('ç'),
                                                         vec!('ç'),
                                                         vec!()));

    if let Some(_) = t1.index_to_state {
        panic!("A none value was expected here");
    }

    assert_eq!(t1.chars_read, vec!('ç','ç'));
    assert_eq!(t1.move_read, TuringDirection::Left);
    assert_eq!(t1.chars_write, vec!(('ç',TuringDirection::Right)));

    assert_eq!(t1.get_number_of_affected_ribbons(), 2)
}

#[test]
fn transition_eq()
{
    let t1 =  TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap();

    assert_ne!(t1, TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Right, TuringDirection::Right)).unwrap());

    assert_ne!(t1, TuringTransitionMultRibbons::create(vec!('ç', 'v'), vec!('ç'), vec!(TuringDirection::Right, TuringDirection::Right)).unwrap());

    assert_ne!(t1, TuringTransitionMultRibbons::create(vec!('ç', 'v'), vec!('t'), vec!(TuringDirection::Right, TuringDirection::Right)).unwrap());

    assert_ne!(t1, TuringTransitionMultRibbons::create(vec!('ç', 'v', 'p'), vec!('t', 'x'), vec!(TuringDirection::Right, TuringDirection::Right, TuringDirection::Left)).unwrap());

    assert_eq!(t1, TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap());
}



// ________________________________________ States tests ______________________________

#[test]
fn create_state() {
    let s = TuringState::new(false,  &String::from("test"));
    // name should be test
    assert_eq!(s.name, String::from("test"));

    // It should be false
    assert!(!s.is_final);

    let s = TuringState::new(true,  &String::from("test2"));

    // It should be true
    assert!(s.is_final);

    // There should be no transitions
    assert!(s.transitions.is_empty());
}

#[test]
fn rename_state() {
    let mut s = TuringState::new(false,  &String::from("test"));
    s.rename("test2");
    // name should be test2
    assert_eq!(s.name, String::from("test2"));
}



#[test]
fn add_transitions() {
    let mut s = TuringState::new(false,  &String::from("test"));
    let transition =  TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap();
    s.add_transition(transition);

    // Check that the transition was added
    assert_eq!(s.transitions.first().unwrap(), &TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap());
}

#[test]
fn remove_transitions() {
    let mut s = TuringState::new(false,  &String::from("test"));
    // add transitions
    s.add_transition(TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap());
    s.add_transition(TuringTransitionMultRibbons::create(vec!('ç', '_'), vec!('0'), vec!(TuringDirection::Left, TuringDirection::Left)).unwrap());

    // Remove both of them
    s.remove_transition(0).unwrap();
    s.remove_transition(0).unwrap();

    assert!(s.transitions.is_empty());

    // Add them back
    s.add_transition(TuringTransitionMultRibbons::create(vec!('ç', 'ç'), vec!('ç'), vec!(TuringDirection::Left, TuringDirection::Right)).unwrap());
    s.add_transition(TuringTransitionMultRibbons::create(vec!('ç', '_'), vec!('0'), vec!(TuringDirection::Left, TuringDirection::Left)).unwrap());


    expect_out_of_range_transition_error(s.remove_transition(2));
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

fn expect_out_of_range_transition_error<O>(res : Result<O, TuringError>)
{
    if let Err(e) = res {
        match e {
            TuringError::OutOfRangeTransitionError { accessed_index, states_len } => (),
            _ => panic!("Wrong error was returned"),
        }
    }
    else {
        panic!("Should have thrown an error")
    }
}
