# Log of things about Rust! 
Commands / notes / etc. 

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
