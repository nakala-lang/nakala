# nakala

<p align="center">
  <img src="./assets/logo.png" width="150"/>
</p>

[![CI](https://github.com/nakala-lang/nakala/actions/workflows/CI.yml/badge.svg)](https://github.com/nakala-lang/nakala/actions/workflows/CI.yml)
![PRs Welcome](https://img.shields.io/badge/PRs-welcomed-green.svg)
[![GitHub license](https://img.shields.io/github/license/nakala-lang/nakala.svg)](https://github.com/nakala-lang/nakala/blob/master/LICENSE)

A programming language that I built based on [arzg's Eldiro blog posts](https://arzg.github.io/lang/). 
The core architecture (especially the Parser) is practically identical to Eldiro, hence the name of the project: nakala (Punjabi for _mimic_).
If you haven't read through his blog posts, I highly recommend you do because it is an unmatched learning resource.

### Why make nakala?
I was so inspired by his blog posts that I have decided to continue the implementation that I made while reading his posts. Since then, nakala has turned into its own unique project. 

The most notable and interesting things I have implemented since continuing my implementation is a runtime engine that computes the parsed `HIR` representation and the countless additional language features.

### Design Philosophy

Nakala only has two core values:

1. The syntax should feel familiar, while remaining unique
2. Using (and working on) nakala should always be fun

## Features
Even though nakala is still in active development, many features are supported so far:

#### Binary Expressions
<p align="center">
  <img src="./assets/exprs.png"/>
</p>

#### Boolean Expressions
<p align="center">
  <img src="./assets/booleans.png" />
</p>

#### Comments
<p align="center">
  <img src="./assets/comments.png" />
</p>

#### Error Handling

<p align="center">
  <img src="./assets/errors_cli.png" />
</p>

#### Variable Declaration and References
<p align="center">
  <img src="./assets/variables.png" />
</p>

#### Code Blocks
<p align="center">
  <img src="./assets/blocks.png" />
</p>

#### Strings
<p align="center">
  <img src="./assets/strings.png" />
</p>

#### Functions
<p align="center">
  <img src="./assets/functions.png" />
</p>


#### If/Else Statements
<p align="center">
  <img src="./assets/ifs.png" />
</p>

#### Loops
<p align="center">
  <img src="./assets/for_loops.png" />
</p>

#### `.nak` File Format

You can store a nakala program in a `.nak` file and run it using the CLI tool. For example:

<p align="center">
  <img src="https://i.gyazo.com/1a44b53e530b2d2bb9396390e290ce5c.gif" />
</p>

## Project Layout
There are a fair amount of moving parts, and just like arzg, I have also split up all the components into separate crates. Below is a dependency graph to visualize how it all links together:
<p align="center" style="width: 100%; margin: auto; margin-top: 20px">
  <img src="./assets/graph.svg"/>
</p>

## Usage
Nakala comes with a REPL CLI tool located in `/crates/nakala`. You can clone the project and run the following to use it:

```bash
$ cargo run
```

There are various flags for improving the developing experience. You can view all of the flags by running
```bash
$ cargo run -- help
```

---

### Contributing
I am always welcoming PRs and would love to work on the project with other people if they are interested. There are no rules, and I will accept any PR as long as it aligns with the projects core values as described above.

### License
`nakala` uses the MIT License

#### Attributions
<div>Icons made by <a href="https://www.freepik.com" title="Freepik">Freepik</a> from <a href="https://www.flaticon.com/" title="Flaticon">www.flaticon.com</a></div>
