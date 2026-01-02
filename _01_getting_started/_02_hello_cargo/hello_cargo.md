Cargo is Rust's build system and package manager

## Install
```
❯ brew install cargo
```
## Create project using cargo
```
❯ cargo new hello_cargo
```

## Building and Running a Cargo Project
```
❯ cargo build 
   Compiling hello_cargo v0.1.0 (~/projects/Rustlings/_01_getting_started/_02_hello_cargo/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.64s
```
- This command creates an executable file in `target/debug/hello_cargo`, now to run 
```
❯ ./target/debug/hello_cargo
Hello, Rust 🦀 with Cargo ⛴️ !!
```

## Use `cargo run` command to build and run at once
```
❯ cargo run 
   Compiling hello_cargo v0.1.0 (~/projects/Rustlings/_01_getting_started/_02_hello_cargo/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/hello_cargo`
Hello, Rust 🦀 with Cargo ⛴️ !!
```