fn main() {
  let mut bf = rsfk_core::BrainfuckState::<u32>::new();
  rsfk::inline_bf!( (bf), { ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++. } );
  println!("{:?}", &bf);
}