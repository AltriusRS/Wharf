pub fn determine_language(path: &String) -> String {
    let ext = extract_extension(path);
    return match ext.as_str() {
        "rs" => "rust".to_string(),
        "toml" => "toml".to_string(),
        _ => {
            "Unknown".to_string()
        }
    }
}

fn extract_extension(path: &String) -> String {
    let mut dots: Vec<&str> = path.split(".").collect();
    if dots.len() > 0 {
        dots.pop().unwrap().to_string()
    } else {
        "Unknown".to_string()
    }
}