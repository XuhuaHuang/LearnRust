# LearnRust
[![wakatime](https://wakatime.com/badge/github/XuhuaHuang/LearnRust.svg)](https://wakatime.com/badge/github/XuhuaHuang/LearnRust)  
This repository is dedicated to contain code written with Cargo while self-learning coding in Rust.  
All executable projects are prefixed with `learn_`, followed by topic number `[0-9]_[0-9]_` incrementally and topic covered in the project. Please use code responsibly.  

---

## Getting Started
To run all project in this repository, make sure you already installed `cargo` by running the `cargo --version` in the terminal.
```Bash
# check the version of cargo
$ cargo --version
# navigate to a specific project folder
$ cd .\learn_1_1_hello_world\
# compile
$ cargo build
# execute
$ cargo run
```

---

## Special Thanks
**Prof. Hussam AI-Hertani**  
Teacher in Electronics and Information Technology program  
Heritage College, Gatineau, Quebec, Canada  

---

## Language
**Rust**  
![An image for Rust](./rust.png "Red Rust Crab")
> The Rust Programming Language: https://doc.rust-lang.org/book/title-page.html  
> Rust by Example: https://doc.rust-lang.org/rust-by-example/  
> The Cargo Book: https://doc.rust-lang.org/cargo/index.html

---

### `todo!()` Macro in Rust
```rust
fn main() {
    // Display the message "Hello, world!"
    todo!("Display the message by using the todo!() macro");
}
```

### Classic `Hello, world` Program
```rust
fn main() {
    // Our main function does one task: show a message
    // println! displays the input "Hello, world!" to the screen
    println!("Hello, world!");
}
```

### Smiley Face
```rust
// Specify the data type "char"
let character_1: char = 'S';
let character_2: char = 'f';
   
// Compiler interprets a single item in quotations as the "char" data type
let smiley_face = '😃';

// Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
let string_1 = "miley ";

// Specify the data type "str" with the reference syntax "&str"
let string_2: &str = "ace";

println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
```

---

## Commonly Used Commnads in Rust
```Bash
# update rustup
$ rustup self update
# set nightly version to default for academic purpose
$ rustup default nightly
# Install Rust Language Server (RSL)
$ rustup component add rls --toolchain nightly
$ rustup component add rust-analysis --toolchain nightly
$ rustup component add rust-src --toolchain nightly
# see the basics of rustc compiler
$ rustc -h
# see the basics of cargo (package management tool)
$ cargo -h
```

---

## Interesting Facts
1. Rust manages code based on `crate` and `mod`; Think of `crate` as projects, each `crate` is a complete unit for compiling, delivering an `.exe` or `.lib` file.  
Inside each `crate`, `mod` is placed as namespaces.
2. By default, the `rustc` compiler introduces dependance to the `stdlib`; In addition to that, the compiler automatically adds an `use` statement for each `crate` developed by the user: `use std::prelude::*;` for commonly used `type`, `trait`, `function` and `macro`.  
The source code of the `prelude` module is placed in `/src/libstd/prelude/` directory.
