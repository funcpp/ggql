use async_graphql::*;
use std::borrow::Cow;

#[derive(SimpleObject, InputObject)]
#[graphql(input_name = "PageArgumentsInput")]
pub struct PageArguments {
    pub offset: i32,
    pub limit: i32,
}

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

#[Object(name_type)]
impl<T: OutputType> Page<T> {
    async fn items(&self) -> &[T] {
        &self.items
    }

    async fn total(&self) -> i32 {
        self.total
    }

    async fn arguments(&self) -> Option<&PageArguments> {
        self.arguments.as_ref()
    }
}
