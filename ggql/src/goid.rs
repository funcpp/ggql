pub fn parse_goid(goid: &async_graphql::ID) -> Result<(&str, i32), String> {
    let parts: Vec<&str> = goid.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("Failed to parse id: {:?}", goid).into());
    }

    let ty = parts[0];
    let id = parts[1].parse::<i32>().unwrap();

    Ok((ty, id))
}
