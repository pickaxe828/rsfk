use input_macro::input;
use num::Integer;

#[derive(Debug)]
pub struct BrainfuckState<T: Integer> {
    pub memory: Vec<T>,
    pub pointer: usize,
}

#[macro_export]
macro_rules! impl_brainfuck_for {
    ($t:ty) => {
        impl BrainfuckState<$t> {
            pub fn new() -> BrainfuckState<u8> {
                BrainfuckState::<u8> {
                    memory: vec![0],
                    pointer: 0usize,
                }
            }

            pub fn mv_left(&mut self, offset: usize) {
                self.pointer -= offset;
            }

            pub fn mv_right(&mut self, offset: usize) {
                self.pointer += offset;
                for _ in self.memory.len()..(self.pointer + offset + 1) {
                    self.memory.push(0);
                }
            }

            pub fn add(&mut self, number: $t) {
                self.memory[self.pointer] += number;
            }

            pub fn sub(&mut self, number: $t) {
                self.memory[self.pointer] -= number;
            }

            pub fn out(&mut self) {
                print!("{}", char::from(self.memory[self.pointer] as u8));
            }

            pub fn inp(&mut self) {
                self.memory[self.pointer] = input!("").parse::<$t>().unwrap();
            }
        }
    };
}

impl_brainfuck_for!(u8);
impl_brainfuck_for!(u16);
impl_brainfuck_for!(u32);
impl_brainfuck_for!(u64);
impl_brainfuck_for!(u128);
impl_brainfuck_for!(usize);