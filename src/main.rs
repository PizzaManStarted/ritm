use turingrs::turing_machine::TuringMachine;

fn main() 
{
    println!("Hello, world!");
    

    let turing_mach: TuringMachine = TuringMachine::new();

    println!("{:?}", turing_mach);
}
