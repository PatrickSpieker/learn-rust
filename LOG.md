# Log of things about Rust! 
Commands / notes / etc. 

## 2022-11-29 lunch

### Chapter 3 (will come back to 2)
Variables immutable by default
mutable with:
```
let mut x = 5;
```
constants vs. immutable vars:
- constants only set with constant expression, not result of value that could only be computed at run time
example const
```
const THREE_HOURS_SECS: u32 = 60 * 60 * 3;
```

can reuse let? so like this is valid
```
let y = 5;
let y = y + 1;
```
lmao so much for immutability

values have data types
two kinds of types: scalar and compound
scalar: ints, floats, booleans, characters
ints: 8, 16, ... 128 bit signed (i8) and unsigned (u128)
    - literals: decimal 98_222, hex 0xff, binary 0b1111_0000, ...
floats: f64 and f32
boolean: bool, true - false
character: unicode scalar value, declared with char
- range from: U+0000 - U+10FFFF

compound: tuples and arrays
tuples: 
```
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
```
- tuple with no values is called the Unit ()
arrays: 
```
let a: [i32; 5] = [1,2,3,4,5];
```
allocated on the stack
have a fixed number of elements
accessed with:
```
a[0]
a[1]
```
functions:
- 2 kinds of instructions in rust: statements and expressions
- statement: performs action; no return value: `let y = 5;`
- expression: has a return value, `x + 1`, doesn't include semi colon

### Chapter 4 - ownership / references / borrowing
no garbage collector
Rules
- each value has an owner
- only one owner at a time
- when owner goes out of scope, value is dropped

when heap-allocated var goes out of scope, "drop" is called
```
// won't compile
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);
```
^^ we say that s1 has been moved to s2
Rust never automatically creates deep copies of data
Copy trait is a thing; primitives basically use this
- all int types
- boolean
- floats
- tuples if their contained types implement copy

can't modify something that you're borrowing
```
fn calc_len(st: &String) -> usize {
    // st doesn't have ownership over the string it points to
    st.len()
}
```
^^ so `calc_len` can't mutate `st` 

you can pass mutable ref with:
```
let mut s = String::from("hello");
let r = &mut s;
```
but you can't have _any other references to the value_ - mutable or immutable
- basically advantage of this behavior is no data races, one thing can mutate at a time
enforced by the compiler:
- either: 1 mut ref, or any number of immut
- refs must all be valid (not dangling)

slices
- ref to contiguous seq of elements in a collection

### Chapter 5 - structs!

### Chapter 6 - enums / pattern matching

### Chapter 7 - packages / crates / modules

### Chapter 8 - collecions!

### Chapter 9 - error handling

### Chapter 10 - generics / traits / lifetimes

### Chapter 11 - tests

### Chapter 12 - I/O

### Chapter 13 - functional stuff: iterators / closures

### (skip for now) Chapter 14 - Cargo / Crates.io

### (skip for now) Chapter 15 - smart pointers

### (skip for now) Chapter 16 - concurrency

### (skip for now) Chapter 17 - OOP

### (skip for now) Chapter 18 - more pattern matching

### (skip for now) Chapter 19 - advanced features

### (skip for now) Chapter 20 - final project!

## 2022-11-29
Starting here
https://doc.rust-lang.org/book/ch01-01-installation.html

Installing the "rustup" tool
```
 curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

Also need a "linker" (do i already have the compiler?)

source "$HOME/.cargo/env"

$ rustc --version

bro rust is about empowerment :)

https://doc.rust-lang.org/book/ch00-00-introduction.html

Book source code: https://github.com/rust-lang/book/tree/main/src

Run `rustup doc` to open the local documentation in your browser.

running hello world:
```
rustc main.rs
./main
```

`rustfmt` is the canonical code formatter / linker. 

from a folder with Cargo.toml, run:
```
cargo build
cargo build --release
```
or 
```
cargo run
```
and also for typechecking
```
cargo check
```

