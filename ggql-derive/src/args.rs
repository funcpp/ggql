use darling::{ast::Data, util::Ignored, FromDeriveInput, FromField};
use syn::Ident;

#[derive(FromField)]
#[darling(attributes(ggql))]
pub struct NodeField {
    pub ident: Option<Ident>,

    #[darling(default)]
    pub id: bool,
}

#[derive(FromDeriveInput)]
#[darling(attributes(ggql))]
pub struct Node {
    pub ident: Ident,
    pub data: Data<Ignored, NodeField>,

    #[darling(default)]
    pub name: String,
}
