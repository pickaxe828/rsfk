#![feature(iter_map_windows)]
#![feature(proc_macro_quote)]

extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};


// Recursive parsing for loops...
fn bf_to_rust(bf_state_ts: TokenStream, bf_code_ts: TokenStream) -> TokenStream {
  // TokenStream -> Option<TokenTree>, with a trailing None for windowing
  let mut bf_code_ts_vec: Vec<Option<TokenTree>> = bf_code_ts.clone().into_iter().map(|x| Some(x)).collect();
  bf_code_ts_vec.push(None);

  let bf_code_tt = bf_code_ts_vec.into_iter().map_windows(|[curr, _next]| {
    // TODO: Optimization by merging same instructions
    match curr {
      Some(TokenTree::Punct(punct)) => { // If curr is punct
        let res = match punct.to_string().as_str() {
          "+" => { format!("{}.{}{}", &bf_state_ts, "add(1)", ";") },
          "-" => { format!("{}.{}{}", &bf_state_ts, "sub(1)", ";") },
          ">" => { format!("{}.{}{}", &bf_state_ts, "mv_right(1)", ";") },
          "<" => { format!("{}.{}{}", &bf_state_ts, "mv_left(1)", ";") },
          "." => { format!("{}.{}{}", &bf_state_ts, "out()", ";") },
          "," => { format!("{}.{}{}", &bf_state_ts, "inp()", ";") },
          token => { panic!("{} {} {}", "Invalid symbol", token, "found.") }
        };
        Some(res.parse::<TokenStream>().unwrap()) // -> Option<TokenStream>
      },
      // If curr is a group, possibly a loop or syntax error
      Some(TokenTree::Group(group)) => {
        let res = match group.delimiter() {
          proc_macro::Delimiter::Bracket => { // [] -> Loop
            format!("{}{}{}{}{}{}{}{}", 
              "loop { if (", 
                bf_state_ts.clone(),
                ".memory[",
                  bf_state_ts.clone(),
                  ".pointer]",
                " == 0) {break;}",
              bf_to_rust(bf_state_ts.clone(), group.stream()),
              "};"
            ).parse().unwrap() // -> TokenStream
          },
          _ => { // Other kinds of delimiters
            panic!("{}", "Invalid syntax. Do you meant to use [..] for loops instead? ")
          }
        };
        Some(res) // -> Option<TokenStream>
      },
      others => {
        panic!("{} {:?} {}", "Invalid syntax. Fragment", others, "is unexpected. ")
      }
    }
  }).collect::<Vec<Option<TokenStream>>>();

  // Sum up the generated tokens
  
  let res = bf_code_tt.into_iter().flatten().collect::<Vec<TokenStream>>();
  res.into_iter().flatten().collect::<TokenStream>()
}

#[proc_macro]
#[allow(unused_parens)]
pub fn inline_bf(_item: TokenStream) -> TokenStream {
  let tree = _item.clone().into_iter().map(|x| x).collect::<Vec<TokenTree>>();
  let bf_state_ts: Option<TokenStream>;
  let bf_code_ts: Option<TokenStream>;

  match &tree[0] {
    TokenTree::Group(bf) => {
      bf_state_ts = Some(bf.stream());
    },
    _ => {
      panic!("{}", "Format ( BrainfuckState<Integer> ), {..} expected.");
    }
  }
  
  match &tree[1] {
    TokenTree::Punct(punct) => {
      if punct.as_char() == ",".chars().nth(0).unwrap() {
        match &tree[2] {
          TokenTree::Group(group) => {
            bf_code_ts = Some(group.stream());
          },
          _ => {
            panic!("{}", "Format ( BrainfuckState<Integer> ), {..} expected.")
          }
        }
      } else {
        panic!("{}", "Format ( BrainfuckState<Integer> ), {..} expected.")
      }
    }
    _ => {
      panic!("{}", "Format ( BrainfuckState<Integer> ), {..} expected.")
    }
  }
  let mut post = bf_to_rust(bf_state_ts.unwrap(), bf_code_ts.unwrap());
  post = format!("{{{}}};", post).parse().unwrap();
  post
  // let mut res: Vec<TokenTree> = post.into_iter().map(|x| x).collect();
  // res.push(bf_state_ts.unwrap().into());
  // res
}