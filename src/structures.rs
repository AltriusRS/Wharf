use crate::structures::langs::Supported;

pub mod langs;


pub enum CommentType {
    METHOD = 0,
    TYPE = 1,
    STRUCT = 2, // Structure OR Class
    ENUM = 3,
    VARIABLE = 4,
    NONE = 5
}

pub struct Comment {
    pub c_lang: Supported,
    pub c_type: CommentType,
    pub c_parent: Option<String>,
    pub c_arguments: Option<Vec<Args>>
}

pub struct Args {
    pub a_type: Option<String>, // Not used in dynamically typed languages
    pub a_name: Option<String>,
    pub a_sample: Option<String>,
    pub can_null: bool,
    pub can_undefined: bool,
}