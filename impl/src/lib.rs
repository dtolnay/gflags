#![allow(
    clippy::module_name_repetitions,
    clippy::option_if_let_else,
    clippy::single_match_else
)]

extern crate proc_macro;

mod define;
mod error;
mod infer;
mod name;
mod parse;
mod placeholder;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn define_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as parse::Flag);
    let expanded = define::expand(input);
    TokenStream::from(expanded)
}
