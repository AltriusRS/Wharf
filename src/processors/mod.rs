use std::fs::read;
use std::path::Path;

pub mod rust;





pub fn parse_file(language: String, path: &Path) {
    let handle = read(path);
    if let Ok(file) = handle {
        let pretext = String::from_utf8(file);
        if let Ok(text) = pretext {
            let mut lines: Vec<&str> = text.split("\n").collect();
            let mut index = 0;
            for line in lines {
                if line.ends_with("\r") {
                    let ctx: Vec<&str> = line.split("\r").collect();
                    lines[index] = ctx[0];
                }
            }
            match language.as_str() {
                "Rust" => rust::parse(lines),
                _ => return
            }
        } else {
            println!("Failed to parse contents of file: {}", path.to_string_lossy())
        }
    } else  {
        println!("Failed to open file for processing: {}", path.to_string_lossy());
    }
}