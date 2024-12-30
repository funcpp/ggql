use std::fmt::Display;

use base64::{engine::general_purpose::STANDARD as base64, Engine as _};

pub fn parse_goid(goid: &async_graphql::ID) -> Result<(&str, i32), String> {
    let parts: Vec<&str> = goid.as_str().split('#').collect();
    if parts.len() != 2 {
        return Err(format!("Failed to parse id: {:?}, parts: {:?}", goid, parts).into());
    }

    let ty = parts[0];
    let id = parts[1].parse::<i32>().unwrap();

    Ok((ty, id))
}

pub fn generate_goid<T: Display>(ty: &str, id: T) -> async_graphql::ID {
    async_graphql::ID::from(base64.encode(format!("{}#{}", ty, id)))
}
