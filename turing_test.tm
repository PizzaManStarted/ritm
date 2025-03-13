q_0 { ç, ç -> R, _, R } q_1;

q_1 { 0, _ -> L, 0, R
    | 1, _ -> N, 0, R } q_a;


_____________________________________

int = { ASCII_DIGIT+ }
str = { (ASCII_ALPHA| "_")+ }

char = { ("ç" | ASCII_DIGIT|ASCII_ALPHA| "_") }

move = _{ dir_left | dir_right | dir_none }
	dir_left	= { "L" }
    dir_right	= { "R" }
    dir_none	= { "N" }

turing_machine = { ( rule ~ ";")+ }

rule = { (var) ~ ("{" ~ transition ~ ("|" ~ transition)* ~ "}")+ ~ (var)}
var = { "q_" ~ (int|str) }

transition = { to_read ~ "->" ~  to_write_move }

to_read = { char ~ ("," ~ char)* } 
to_write_move = { move ~ ("," ~ char ~ "," ~ move)* } 

WHITESPACE = _{ " " | "\t" | "\n" }
