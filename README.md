# nakala

A programming language that I built based on [azrg's Eldiro blog posts](https://arzg.github.io/lang/). For the time being, this language is practically identical to it, hence the name "nakala" (which is Punjabi for _mimic_).
However, I plan to add more features and built on top of it over time, especially the CLI tool.

This is just a hobby project to expand my rust programming abilities as well as my knowledge about programming languages. 

There are a fair amount of moving parts, and just like azrg did, I have also split up all the components into separate crates. Below is a dependency graph to visualize how it all links together:
<p align="center" style="width: 90%; margin: auto; margin-top: 20px">
  <img src="./assets/graph.svg"/>
</p>

## Features
As nakala is in its very early stages, the language does not have many features. However, it does have:

#### Binary Expressions
Prefix, Infix, and Postfix binary expression support
```
1 + 4 * 10 + -4
```

#### Comments
You can have comments placed within expressions
```
1 
+ 123 # add a medium number
+ 5512312 # add a large number
```


## Usage
Nakala comes with a REPL CLI tool located in `/crates/nakala`. You can clone the project and run the following to use it:

```bash
$ cargo run
```


