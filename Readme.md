## Summary Rust "The book"

#### Chapter 1
- Install Rust using rustup
    
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
```
- Update to a newer Rust version
```
rustup update
```
- Open locally installed documentation
```
rustup doc
```
- Write and run a “Hello, world!” program using rustc directly
```
$ rustc main.rs
$ ./main
Hello, world!
```
- Create and run a new project using the conventions of Cargo
```
$ cargo new hello_cargo
$ cd hello_cargo
cargo build - compila
cargo run - compila y ejecuta
cargo check - comprueba
```

# Por donde voy

file:///home/clonbg/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/book/ch02-00-guessing-game-tutorial.html#handling-invalid-input



# Errores y soluciones

- Blocking waiting for file lock on package cache

```
rm -rf ~/.cargo/.package-cache
```