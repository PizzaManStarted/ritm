use turingrs::{
    turing_machine::{TuringMachine, TuringMachineExecutor},
    turing_state::{TuringDirection, TuringTransition},
};

fn main() {
    println!("Hello, world!");

    let turing_mach: TuringMachine = TuringMachine::new(2)
        .add_rule_state(
            "q_0".to_string(),
            TuringTransition::new(
                vec!['ç', 'ç', 'ç'],
                TuringDirection::Right,
                vec![('ç', TuringDirection::Right), ('ç', TuringDirection::Right)],
            ),
            "q_1".to_string(),
        )
        .unwrap()
        .add_rule_state(
            "q_1".to_string(),
            TuringTransition::new(
                vec!['1', '_', '_'],
                TuringDirection::Right,
                vec![('1', TuringDirection::Right), ('0', TuringDirection::Right)],
            ),
            "q_a".to_string(),
        )
        .unwrap();

    let mut tm_exec =
        TuringMachineExecutor::new(&turing_mach, "10001110011101".to_string())
            .unwrap();

    let mut i = 0;
    println!("Before all : \n{}", tm_exec);

    for () in &mut tm_exec {
        println!("{i}");
        i+=1;
    }
 
    println!("After all : \n{}", tm_exec);
   
}
