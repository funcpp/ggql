use std::fmt::Display;

use base64::{engine::general_purpose::STANDARD as base64, Engine as _};

pub fn parse_goid(goid: &async_graphql::ID) -> Result<(String, i32), String> {
    let decoded = base64.decode(goid.as_str());
    if decoded.is_err() {
        return Err(format!("Failed to decode id: {:?}", goid).into());
    }

    let goid = String::from_utf8(decoded.unwrap()).unwrap();

    let parts: Vec<&str> = goid.split('#').collect();
    if parts.len() != 2 {
        return Err(format!("Failed to parse id: {:?}, parts: {:?}", goid, parts).into());
    }

    let ty = parts[0];
    let id = parts[1].parse::<i32>().unwrap();

    Ok((ty.to_string(), id))
}

pub fn generate_goid<T: Display>(ty: &str, id: T) -> async_graphql::ID {
    async_graphql::ID::from(base64.encode(format!("{}#{}", ty, id)))
}
