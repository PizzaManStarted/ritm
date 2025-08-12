# $\texttt{RITM}$ : *R*ead *E*val *P*rint *L*oop

## 1) Load/Create Turing Machine Graph

In this mode you can either create a new empty turing machine graph or load an existing one.

### Create

Creating a blank graph is as easy as specifying the number of working ribbons that you will be working with.

This is because every transitions will have to respect this value when you will later add them.


A blank graph consists of three default states :
* $q_i$ : The **initial** state where every execution will start from.
* $q_a$ : The **accepting** state where an execution should end to *accept* an input.
* $q_r$ : The **rejecting** state. This state can be use to directly *reject* an input.

### Load

To load a graph you already made, simply specify the path to this machine. This path can be **absolute** or even **relative** to where the REPL was executed from.


## 2) Modify Turing Machine Graph

### Print a summary of the Turing Machine

This command will simply display a *summary* of the current graph that you are editing. 

This summary includes : 
* The states present in the machine and their type
* All the transitions contained in the machine per pair of states.

The latter can be copied to an external text file in order to *save* this graph for later uses.

### Add a state

Adds another state to the graph.

### Add one or multiple transition

Use this command to append one or multiple transition between two states. 

If one of the given state does not currently exists, then it will be **automatically** added.  

The inputs must respect the parser rules for transitions. 

For example :
* Adding **one** transition between $q_i$ and $q_1$ : `q_i {ç, ç -> R, ç, R} q_1`
* Adding **four** transitions between $q_2$ and $q_2$ : `q_2 { 0, 0 -> N, 0, L | 0, 1 -> N, 1, L  | 1, 0 -> N, 0, L  | 1, 1 -> N, 1, L } q_2;`

### Remove one or multiple transition
### Remove a state

Removes the given state and **any** mention of it inside the graph.


This means that every transitions leaving or entering it will also be removed.

It is not possible to remove any of the default states of the graph like $q_i$, $q_a$ or even $q_r$.

### Save this TM as a file



### Feed a word and start executing this Turing Machine
### Unload the current Turing Machin




## 3) Execute Turing Machine


### Move to next step
### Skip multiple steps
### Execute at a given speed the TM
### Finish the execution (can loop forever)
### Reset the execution
### Feed a new word and reset
### Toggle on/off clearing after each step
### Sets the execution mode
### Print a summary of the graph
### Print a summary of the execution
### Iterate over the correct path, if any (can loop forever)
### Stop the execution