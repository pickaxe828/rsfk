# rsfk-core ♋
Brainfuck state machine. Dependency of `rsfk`

# How to use this project?
  ## Prerequisite
  - [Rust](https://www.rust-lang.org/)
  ## Command
  - `cargo run` to execute `main.rs`
  ## Usage
  ** This library contains no implementation for loop instructions `[` and `]` ** </br>
  (In `rsfk`, the loop of Rust `loop {}` is used when expanding the macro)
  ### Struct
  `BrainfuckState::new() -> BrainfuckState::<u8>`
  ### Methods
  `BrainfuckState::mv_left(usize) -> ()`</br>
  `BrainfuckState::mv_right(usize) -> ()`</br>
  `BrainfuckState<T>::add(T) -> ()`</br>
  `BrainfuckState<T>::sub(T) -> ()`</br>
  `BrainfuckState::out(usize) -> ()`</br>
  `BrainfuckState::inp(usize) -> ()`</br>