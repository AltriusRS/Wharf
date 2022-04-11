use std::path::Path;

mod fs;

mod language;

fn main() {
    let root = std::env::current_dir().unwrap().to_string_lossy().to_string();
    let fmt = format!("{}/.wharfignore", root);
    let path = Path::new(fmt.as_str());
    let ignored = if let Ok(_) = std::fs::OpenOptions::new().create(false).read(true).open(path) {
        let f = gitignore::File::new(path);
        if let Ok(file) = f {
            file
        } else {
            println!("Failed to parse .wharfignore file.");
            dbg!(f.unwrap_err());
            std::process::exit(1);
        }
    } else {
        let f = std::fs::OpenOptions::new().create(true).write(true).open(path);
        if let Ok(_) = f {
            let f = gitignore::File::new(path);
            if let Ok(file) = f {
                file
            } else {
                println!("Failed to parse .wharfignore file.");
                dbg!(f.unwrap_err());
                std::process::exit(1);
            }
        } else {
            println!("Failed to generate .wharfignore file.\nPlease create one before continuing.");
            std::process::exit(1);
        }
    };

    println!(".wharfignore file loaded successfully");

    fs::walk("".to_string(), &ignored)
}


