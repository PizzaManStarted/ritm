use turingrs::{
    turing_machine::{TuringMachine, TuringMachineExecutor},
    turing_state::{TuringDirection, TuringTransition},
};

fn main() {
    println!("Hello, world!");

    let mut turing_mach: TuringMachine = TuringMachine::new(2);

    turing_mach
        .add_rule_state(
            "q_0".to_string(),
            TuringTransition::new(
                vec!('ç', 'ç', 'ç'),
                TuringDirection::Right,
                vec![('ç', TuringDirection::Right), ('ç', TuringDirection::Right)],
            ),
            "q_1".to_string(),
        )
        .expect("h");

    let tm_exec =
        TuringMachineExecutor::new(&turing_mach, "Hi I am glad to meet you all".to_string()).unwrap();

    for () in tm_exec {
        println!("hi")
    }
    //println!("{:?}", turing_mach);
}
