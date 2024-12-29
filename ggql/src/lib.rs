pub mod goid;

pub use ggql_derive::*;

pub trait NodeTrait {
    const NODE_NAME: &'static str;

    fn node_name(&self) -> &str;
    fn node_id(&self) -> async_graphql::ID;
}

#[macro_export]
macro_rules! node_interface {
(
        $enum_name:ident {
            $(
                $variant:ident => $getter:expr
            ),* $(,)?
        }
    ) => {
        #[derive(async_graphql::Interface)]
        #[graphql(field(name = "id", ty = "ID"))]
        pub enum $enum_name {
            $(
                $variant($variant),
            )*
        }

        impl $enum_name {
            pub async fn get_node_by_goid(ctx: &async_graphql::Context<'_>, goid: &async_graphql::ID) -> async_graphql::Result<Self> {
                let (ty, id) = match ggql::goid::parse_goid(goid) {
                    Ok(v) => v,
                    Err(e) => return Err(e.into()),
                };

                match ty {
                    $(
                        $variant::NODE_NAME => {
                            let item = $getter(ctx, id);
                            Ok($enum_name::$variant(item))
                        },
                    )*
                    _ => Err(format!("Failed to get node by id (ty = {}, id = {})", ty, id).into()),
                }
            }
        }
    };
}

pub trait ResultExt<T, E> {
    fn map_into<U, F>(self) -> Result<U, F>
    where
        T: Into<U>,
        E: Into<F>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn map_into<U, F>(self) -> Result<U, F>
    where
        T: Into<U>,
        E: Into<F>,
    {
        self.map(Into::into).map_err(Into::into)
    }
}
