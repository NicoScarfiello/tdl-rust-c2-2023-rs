pub fn add_extension(name: &str, extension: &str) -> String {
    let mut parts: Vec<&str> = name.split('.').collect();
    if parts.len() > 1 {
        parts.pop();
    }
    parts.push(extension);
    parts.join(".")
}
