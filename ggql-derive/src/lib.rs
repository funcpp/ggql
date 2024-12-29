extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod args;
mod node;
mod utils;

#[proc_macro_derive(Node, attributes(ggql))]
pub fn derive_node(input: TokenStream) -> TokenStream {
    let node_args = match args::Node::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(node_args) => node_args,
        Err(err) => return TokenStream::from(err.write_errors()),
    };
    match node::generate(&node_args) {
        Ok(expanded) => expanded,
        Err(err) => err.write_errors().into(),
    }
}
