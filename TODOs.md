# TODOs

## Consider read-only visitors for immutable visits

## Allow comments before the function definition

e.g this won't currently parse:
```
// This is a comment
fn main() {
    let x = 1u8;
}
```

## statically check that all snippets under test/snippets are referenced by at least one test in tests.rs

# TODOs that could break existing user's code:

## Do not allow keywords in identifier names:
The current parser allows this example:

```
fn main() {
    let let = 1u8;
    let fn = 2u8;
    let u8 = 3u8;
}
```

## Parsing inputs could use a comma
e.g this is allowed to parse:

```
fn main(a: u8 b: u8) {
    let x = a;
}
```

it would be arguably more readable with commas, e.g this:

```
fn main(a: u8, b: u8) {
    let x = a;
}
```

This change should make sure that both the grammar and the print utilities for the AST nodes are aligned.

