use std::fs::read_dir;
use gitignore::File;

use crate::language::determine_language;

///# Walk
///- path: String
///Used to perform an action for all non-directory
///paths within a given directory and its children.
pub fn walk(path: String, ignore: &File) {
    let rd_result = read_dir(&path);

    if let Ok(contents) = rd_result {
        for result in contents {
            if let Ok(entry) = result {
                let p = entry.path();
                let route = p.as_path();
                let ig = ignore.is_excluded(&route);
                if let Ok(i) = ig {
                    if i {
                        let raw_meta = entry.metadata();
                        if let Ok(meta) = raw_meta {
                            if meta.is_file() {
                                let lang = determine_language(&route.to_string_lossy().to_string());
                                println!("File {} is of type {}", route.to_string_lossy().to_string(), lang);
                            } else if meta.is_dir() {
                                walk(route.to_string_lossy().to_string(), &ignore)
                            } else {
                                println!("Symlink located: {}", route.to_string_lossy().to_string());
                                println!("Not following")
                            }
                        }
                    } else {
                        println!("Ignoring: {}", route.to_string_lossy().to_string())
                    }
                } else {
                    dbg!(ig.unwrap_err());
                }
            } else {
                println!("Encountered an error with result");
            }
        }
    }
}