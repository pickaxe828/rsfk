# rsfk 🦀
Brainfuck "compiler", written with Rust, macros and a splash of ❤️.

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