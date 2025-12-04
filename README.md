This whole project is generated using cargo.
Cargo is the rust package manager it handles packages called dependencies.
To generate a new cargo project:
> cargo new project_name

This generates the following:

### Cargo.lock
Holds all the dependencies of your dependencies.

### Cargo.toml 
Holds your dependencies and basic project configuration.

### target
Holds code generated and used by cargo e.g the executable.
Target is automaticly added to the .gitignore.

### src 
Holds application code.
By default it generates with a main.rs that holds the main function that is triggered on `cargo run`.

---

Using cargo lets u use cargo commands:
> cargo run

compiles the project into an executable file which then gets executed.

### Code standard
Rust used snake_case for the following:
- `.rs` files
- folders
- project names
- functions
- variables

Constants use SCREAMING_SNAKE_CASE.

## Concepts
### Rust variables
Rust variables are immutable by default, meaning once a value is assigned, the value cannot change.
To make a variable mutable use the `mut` keyword.
? What is the difference between a constant

```
let mut immutable_variable = 0; // This value will hold 1 forever.
let mutable variable = 1; // This value can be changed.
```

### Types
`String::new` means calls the new function on the String type from the standard library.

Using `::` calls a function on a type or module.

### Modules
Modules can be imported using the `use` keyword.

```
use std::io;

io::stdin().readline(&mut input)
```
which will make the `io` module usable.

However to use modules they don't have to be imported, functions on modules can also be called directly on the module.
```
std::io::stdin().readline(&mut input)
```

### Enums
Enums are types that can have multiple states, each state is called a variant.

`std::io::readline()` returns the enum type `Result` which has two variants:
- `Ok`, contains the generated value.
- `Err`, contains information about the error.

The `Result` type also contains functions that can be called, `expect()` will check if `Err` is returned, if so it will display the string passed in its paramater. If `Ok` is passed it will return the generated value back letting u use that. If don't call expect on a function that can return the `Err` variant it will display a warning during compilation:
```
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
10 |     let _ = io::stdin().read_line(&mut guess);
   |     +++++++

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.59s

```

### References 
References are created with the `&` symbol, and are immutable by default.

For some reason it is good to change the input to a mutable reference, this has to do with performance i don't know why tho.

