# Grammar

The grammar specification for nakala, A C style language. Huge shoutout to Bob Nystrom from "Crafting Interpreters", as I used a lot of his language design for this (as I would be confident it would end up with a working language as I wanted in the end)

```
expr -> literal | unary | binary | grouping
literal -> NUMBER | STRING | "true" | "false" | "null"
grouping -> "(" expr ")"
unary -> ( "-" | "!" ) expr
binary -> expr op expr
op -> "==" | "!=" | "<" | "<=" | ">" | ">=" | "+" | "-" | "*" | "/"
```

### Types
```
bool
number
string
null
```

### Expressions

#### Arithmetic
```
add + foo
sub - bar
multiply * foo
divide / bar
- negateMe
```

#### Comparison & Equality
```
less < than
lessThan <= orEqual
greater > than
greaterThan >= Equal
1 == 2 // false
"cat" != "dog" // true
```

#### Logical Operators

```
!false // true
true and false // false
true or false // true
```

### Statements

#### Variables
```
let a = 123 // a is a number
let b // b is null
let c = 123.31 // c is a number
```

#### Blocks
```
{
  print "One statement.";
  print "Two statement.";
}
```

#### If
```
if (condition) {
  print "condition is true"
} else {
  print "condition is false"
}
```

#### Until
```
let a = 0;
until (a == 10) {
  print a;
  a = a + 1;
}
```

### Functions

Functions are first class in Nakala

```
func addTwoThings(a, b) {
  ret a + b;
}

func identity(a) {
  ret a;
}

print identity(addTwoThings)(1,2); // Prints "3"
```

#### Closures 
```
func returnFunction() {
  let outside = "outside";

  func inner() {
    print outside;
  }

  ret inner;
}

let fn = returnFunction();
fn();
```

### Classes
Nakala is object-oriented because it is something I am confident in building.

```
class Breakfast {
  cook() {
    print "Eggs are frying"
  }

  serve(who) {
    print "Enjoy your breakfast, " + who + ".";
  }
}
```

Classes are first class
```
let myClass = Breakfast;

let myInstanceOfSomeClass = somethingThatUsesAClass(Breakfast);
```

#### Instantiation

```
class Breakfast {
  init(meat, bread) {
    this.meat = meat;
    this.bread = bread;
  }

  serve (who) {
    print "Enjoy your " + this.meat + " and " + this.bread + ", " + who " .";
  }
}
```

#### Inheritance
```
class Brunch extends Breakfast {
  init(meat, bread, drink) {
    super.init(meat, bread);
    this.drink = drink;
  }
}
```
