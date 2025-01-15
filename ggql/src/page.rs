use std::borrow::Cow;

use async_graphql::{InputObject, OutputType, SimpleObject, TypeName};

#[derive(InputObject, SimpleObject)]
pub struct PageArguments {
    pub offset: i32,
    pub limit: i32,
}

#[derive(SimpleObject)]
pub struct Page<T: OutputType> {
    pub items: Vec<T>,
    pub total: i32,
    pub arguments: Option<PageArguments>,
}

impl<T: OutputType> Page<T> {
    pub fn new(items: Vec<T>, total: i32, arguments: Option<PageArguments>) -> Self {
        Self {
            items,
            total,
            arguments,
        }
    }
}

impl<T: OutputType> TypeName for Page<T> {
    fn type_name() -> Cow<'static, str> {
        format!("{}Page", T::type_name()).into()
    }
}