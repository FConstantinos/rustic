# Rustic

## Introduction
This is a parser for a small subset of the Rust programming language. The subset it parses are arithmetic operations on u8 literals and variables. On command, it also performs constant folding and constant propagation.

## Build

To build it, run

```
cargo build --release
```

for debug builds (less optimizations than release builds), use

```
cargo build
```

This will build the executables under target/debug/rustic and target/release/rustic

## Run

To run the program:

```
rustic <input file>
```

where \<input file\> is any rust source code that is admited by the grammar under src/rust.pest

to run with constant propagation (also see relevant section below)

```
rustic <input file> --constprop
```

To run the tests do this on the source directory:

```
cargo test
```

## Constant Propagation
You can apply constant propagation on your AST. This performs constant folding and
propagates variables that are constant.

The requirement is that the AST produced is equivalent to the original one.

To use it:

```
rustic <input file> --constprop
```

here is an example of constant folding. This file:

```
fn main() {
    let a = 1u8 + 1u8;
}
```

translates to:


```
rustic before.rs --constprop

Unparsed file:
"fn main() {\n    let a = 1u8 + 1u8;\n}\n"

Resulting program:

fn main() {
    let a = 2u8;
}
```

here is an example of constant propagation. This file:

```
fn main() {
    let a = 10u8;
    let b = 5u8;
    let c = a + b;
    let d = b * 2u8;
}
```

translates to:

```

Unparsed file:
"fn main() {\n    let a = 10u8;\n    let b = 5u8;\n    let c = a + b;\n    let d = b * 2u8;\n}"

Resulting program:

fn main() {
    let a = 10u8;
    let b = 5u8;
    let c = 15u8;
    let d = 10u8;
}
```

Here is a more complex example:

```
fn main() {
    let x = 2u8/(1u8 + 2u8/(1u8 + 1u8));
    let y = x * (2u8 + 3u8 * (4u8 + 5u8));
}
```

translates to:

```
rustic nested_3.rs --constprop
Unparsed file:
"fn main() {\n    let x = 2u8/(1u8 + 2u8/(1u8 + 1u8));\n    let y = 1u8 * (2u8 + 3u8 * (4u8 + 5u8));\n}"

Resulting program:

fn main() {
    let x = 1u8;
    let y = 29u8;
}
```

### Assumptions:
- Binary operators return u8
    - therefore we can't have negative numbers, rationals or anything else besides non-negative integers up to 255

### Limitations:

#### Interaction with Left Associativity
Consider this program

```
fn main(x: u8) {
    let m = x + 5u8 - 2u8;
	let n = 5u8 - 2u8 + x;
}
```

A user might expect that applying constant propagation would simplify this as such:

```
fn main(x: u8) {
    let m = x + 3u8;
	let n = 3u8 + x;
}
```

However, this is not the case and the actual result is this:

```
fn main(x: u8) {
    let m = x + 5u8 - 2u8;
	let n = 3u8 + x;
}
```

where constants are not propagated in the first expression but are propagated in the second.

This is because constant folding works by visting the AST and transforming binary operations to integers if both arguments are integers.

Since expressions evaluate from left to right, the order of evaluation for the first initializing expression would be this:

```
let m = (x + 5u8) - 2u8;
```

the visiting order of the binary operations would be (x + 5u8) which doesn't transform since x is not a constant
The next node to be visited would be the subtraction and becaue the left operand is not an integer, the transformation will do nothing.

A more involved implementation could perhaps work like this:

- Expand the expression (e.g (a + b) * ( c + d) would become a*c +a*d + b*c + b*d )
- Move all constants and literals to the left. 
    - That would probably need the language to define unary operators e.g in cases like x - 1u8 - 2u8
- terms with division would have to be handled separately (e.g perform the above steps on the nominator and denominator separately).


#### Immediate Failure on non u8 Intermediate Results

Another limitation is that the folding will fail if constant evaluation creates a value that does not fit the type of u8. For example, this:

```
let m = 252u8 + 5u8 - 2u8;
```

will fail when visiting the first binary operation (252 + 5) because it would overflow the u8 type, even though the value of the whole of the initializing expression is 255.

This is in line with the assumption that binary operators can only return u8 results.

A more involved implementation could propagate intermediate results through annotating the AST to the level of the assignment and compute the final results there (and check for errors)

## Known bugs:
AST printing displays extra parenthesis. The output AST is still equivalent to the original code. This problem can be solved by passing an attribute to the AST nodes of the expressions that actually have parenthesi and not printing parenthesi otherwise


