# nakala

A programming language that I built based on [azrg's Eldiro blog posts](https://arzg.github.io/lang/). 
The core design and architecture are all practically identical to Eldiro, hence the name of the project: nakala (Punjabi for _mimic_). 
If you haven't read through his blog posts, I highly recommend you do because it is an unmatched learning resource.

### Why make nakala?
I was so inspired by his blog posts that I have decided to continue the implementation that I made while reading his posts. 

The most notable of which being:
- Creating a runtime engine that computes the parsed `HIR` representation
- Adding many more language features (such as variable references, code blocks, string literals, functions)
- CLI Runtime/REPL. 

## Features
As nakala is in its very early stages, the language does not have many features. However, it does have:

#### Binary Expressions
Prefix, Infix, and Postfix binary expression support
```
1 + 4 * 10 + -4

200 + (5 * (100 + 4))
```

#### Comments
You can have comments placed within expressions
```
1 
+ 123 # add a medium number
+ 5512312 # add a large number
```

#### Variable Declaration and References
```
let a = 200 + (5 * (100 + 4))

let b = a
```

#### Code Blocks
```
let x = {
  # first let's declare a variable
  let temp = 100

  # now lets create another variable
  let temp2 = 500

  # add them together. The final statement in a block is the value returned
  temp + temp2
}
```

#### Strings
```
let x = "Hello, World!"
```

#### Functions
```
fn test(x,y) { x + y }

call test(10, 5) # output is 15

# since code blocks are expressions, you can have crazy things like this totally work fine :)
fn get_const() { 10 }
fn add(num1, num2) { num1 + num2 }

let sum = call add (
  {
    let someOtherVariable = 10
    let factor1 = 12341

    someOtherVariable * factor1

  },
  {
    let delta = -10
    
    delta * (-5)
  } * call get_const()
)

sum # output is 12391
```

#### `.nak` File Format

You can store a nakala program in a `.nak` file and run it using the CLI tool. For example:

```
# my_program.nak

let x = 100

let y = x + 5

x + y
```

You can then run this program with the following command

```
$ nakala my_program.nak

110
```

## Project Layout
There are a fair amount of moving parts, and just like azrg, I have also split up all the components into separate crates. Below is a dependency graph to visualize how it all links together:
<p align="center" style="width: 100%; margin: auto; margin-top: 20px">
  <img src="./assets/graph2.svg"/>
</p>


## Usage
Nakala comes with a REPL CLI tool located in `/crates/nakala`. You can clone the project and run the following to use it:

```bash
$ cargo run
```


