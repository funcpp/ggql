use darling::ast::Data;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Error;

use crate::{args, utils::GeneratorResult};

pub fn generate(node_args: &args::Node) -> GeneratorResult<TokenStream> {
    let ident = &node_args.ident;
    let node_name = &node_args.name;

    if node_name.is_empty() {
        return Err(syn::Error::new(ident.span(), "Node name is required").into());
    }

    let s = match &node_args.data {
        Data::Struct(s) => s,
        _ => return Err(Error::new_spanned(ident, "Node can only be applied to a struct").into()),
    };

    let id_field = s.fields.iter().find(|f| f.id);
    let id_field_name = match id_field {
        Some(f) => f.ident.as_ref().unwrap(),
        None => &format_ident!("id"),
    };

    let expanded = quote! {
        impl ggql::NodeTrait for #ident {

            const NODE_NAME: &'static str = #node_name;

            fn node_name(&self) -> &str {
                Self::NODE_NAME
            }

            fn node_id(&self) -> async_graphql::ID {
                //use base64::{engine::general_purpose::STANDARD as base64, Engine as _};
                //async_graphql::ID::from(base64.encode(format!("{}#{}", self.node_name(), self.#id_field_name)))

                ggql::goid::generate_goid(Self::NODE_NAME, &self.#id_field_name)
            }
        }
    };

    Ok(expanded.into())
}
