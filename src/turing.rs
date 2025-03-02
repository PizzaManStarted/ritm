pub struct TuringMachine
{
    states: Vec<TuringState>,
    reading_rubon: TuringRubons,
    write_rubons: TuringRubons,
}

struct TuringState
{
    is_final: bool,
    transitions: Vec<TuringTransition>,
    name: Option<String>
}

enum TuringDirection {
    Left,
    Right,
    None
}

struct TuringTransition
{
    char_read: char,
    move_read: TuringDirection,
    // When *first* char is read, replace it with the second *char* and move to direction 
    char_write: (char, Option<char>, TuringDirection)
}

struct TuringRubons
{
    chars_vec: Vec<char>
}