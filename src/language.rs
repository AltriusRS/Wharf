use crate::structures::langs::Supported;

pub fn determine_language(path: &String) -> Supported {
    let mut dots: Vec<&str> = path.split(".").collect();
    if dots.len() > 0 {
        match dots.pop().unwrap().to_lowercase().as_str() {
            "rs"  => Supported::RUST,
            "js"  => Supported::JAVASCRIPT,
            "ts"  => Supported::TYPESCRIPT,
            "py"  => Supported::PYTHON,
            "bat" => Supported::BATCH,
            "bas" => Supported::BASIC,
            _     => Supported::UNKNOWN
        }
    } else {
        Supported::UNKNOWN
    }
}