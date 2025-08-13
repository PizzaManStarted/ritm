# $\texttt{RITM}$ : *R*ead *E*val *P*rint *L*oop

## Default commands

The following commands are always accessible.

### Print help

There are two types of help.

By simply typing `help` or even `h`, you will be shown the list of available commands that you can access.

By typing the same as above but following it with a positive integer (like for example : `help 0` or `h 2`) you will get a more in depth help/explanation of what the command with the given index can do.


### Clear screen

By typing `clear` or even `cl`, you can *clear* the terminal. 

### Quit program

And finally typing `q`, `quit`, `exit` or even `leave`.

You can safely exit the program. But beware that nothing will be saved, including the eventual turing machine you are working on, so don't forget to save it before leaving !



## 1) Load/Create Turing Machine Graph

In this mode you can either create a new empty turing machine graph or load an existing one.

### Create

Creating a blank graph is as easy as specifying the number of working ribbons that you will be working with.

This is because every transitions will have to respect this value when you will add them later.

> [!NOTE]
> A blank graph consists of three default states :
> * $q_i$ : The **initial** state where every execution will start from.
> * $q_a$ : The **accepting** state where an execution should end to *accept* an input.
> * $q_r$ : The **rejecting** state. This state can be use to directly *reject* an input.

### Load

To load a graph that is already saved in a text file, simply specify the path to this machine. 

This path can be **absolute** or even **relative** to where the REPL was executed from.


## 2) Modify Turing Machine Graph

### Print a summary of the Turing Machine

This command will simply display a *summary* of the current graph that you are editing. 

> [!NOTE]
> This summary includes : 
> * The states present in the machine and their type
> * All the transitions contained in the machine per pair of states.

The latter can be copied to an external text file in order to *save* this graph for later uses.

### Add a state

Adds another **normal** state to the graph.

### Add one or multiple transition

Use this command to append one or multiple transition between two states. 

> [!TIP]
> If one of the given state does not currently exists, then it will be **automatically** added.  

The inputs must respect the parser rules for transitions. 

For example :
* Adding **one** transition between $q_i$ and $q_1$ : `q_i {ç, ç -> R, ç, R} q_1`
* Adding **four** transitions between $q_2$ and $q_2$ : `q_2 { 0, 0 -> N, 0, L | 0, 1 -> N, 1, L  | 1, 0 -> N, 0, L  | 1, 1 -> N, 1, L } q_2`

### Remove one or multiple transition
### Remove a state

Removes the given state and **any** mention of it inside the graph.


This means that every transitions leaving or entering it will also be removed.

> [!WARNING]
> It is not possible to remove any of the default states of the graph like $q_i$, $q_a$ or even $q_r$.

### Save this TM as a file

Use this command to save the turing machine you are working with as a text file (by default it will add a `.tm` extension but you can change it as you desire).

The program will propose you a default path. This path will be current current one, but you can change it to fit your needs.


### Feed a word and start executing this Turing Machine

Feed a string to the turing machine and start executing it.

> [!WARNING]
> The word cannot contain any special symbols like `$`, `ç` and `_`. And empty string are not accepted. 



### Unload the current Turing Machine




## 3) Execute Turing Machine


### Move to next step
### Skip multiple steps
### Execute at a given speed the TM
### Finish the execution (can loop forever)


> [!CAUTION]
> Please be aware that if 


### Reset the execution
### Feed a new word and reset
### Toggle on/off clearing after each step
### Sets the execution mode
### Print a summary of the graph
### Print a summary of the execution
### Iterate over the correct path, if any (can loop forever)
### Stop the execution