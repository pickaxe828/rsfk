# rsfk 🦀
Brainfuck "compiler", written with Rust, macros and a splash of ❤️

# How to use this project?
  ## Prerequisite
  - [Rust](https://www.rust-lang.org/)
  ## Command
  - `cargo run` to execute `main.rs`
  ## Usage
  ### Macro
  `inline_bf! ( (Instance of rsfk_core::BrainfuckState) {BF instruction} )`
  ```rs
  use rsfk;

  fn main() {
    let mut bf = rsfk_core::BrainfuckState::<u32>::new();
    rsfk::inline_bf!( (bf), { ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++. } );
    println!("{:?}", &bf);
  }

  // Hello World!
  ```

# Roadmap, probably
- [x] Basic code
- [x] Cargo upload
- [ ] Input fix (`ParseIntError { kind: InvalidDigit }`)
- [ ] Test cases
- [ ] CI/CD on GitHub Action
  - [ ] Separate Cargo/ GitHub branch
  - [ ] Auto release onto Cargo when PR is merged to the branch
- [ ] Optimizations
  - [ ] Repeated instructions into one function call
- [ ] Standalone bf compiler?
- [ ] Detailed control over the state of the machine?
  - [ ] Preload memory
  - [ ] Step execution (Possible?)