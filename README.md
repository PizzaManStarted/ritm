# *R*ibbon *I*nteractive *T*uring *M*achine


The goal of this crate is to allow users to experiment with **Turing Machines**. Especially the ones that 

## Installation

## Usage

There are multiple ways to use this crate depending on your needs or preferences.

### Using the code directly



### Using the GUI

### Using the REPL (Read-Eval-Print Loop)

The REPL has multiple modes, each with a different purpuse.




## Parser

A parser was made to ease the usage of this program.


### Example

This *non-deterministic* machine accepts the following language :
$$L = \{ xx \,|\, x \in \Sigma^*_{bool} \}$$
```
q_i { ç, ç -> R, ç, R } q_1;

q_1 { 0, _ -> R, 0, R 
    | 1, _ -> R, 1, R } q_1;

q_1 { 0, _ -> N, _, L 
    | 1, _ -> N, _, L } q_2;

q_2 { 0, 0 -> N, 0, L 
    | 0, 1 -> N, 1, L 
    | 1, 0 -> N, 0, L 
    | 1, 1 -> N, 1, L } q_2;

q_2 { 0, ç -> N, ç, R 
    | 1, ç -> N, ç, R } q_3;


q_3 { $, _ -> N, _, N } q_a;

q_3 { 0, 0 -> R, 0, R 
    | 1, 1 -> R, 1, R } q_3;
```


## Acknowledgments

This project was realised by two umons students :
* [Adrien Zianne](https://github.com/AdrienZianne)
* [Axel Foucart](https://github.com/PizzaManStarted)