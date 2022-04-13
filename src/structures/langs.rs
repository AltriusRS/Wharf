//#
//#
pub enum Supported {
    RUST,        // Rust                                 https://en.wikipedia.org/wiki/Rust_(programming_language)
    JAVASCRIPT,  // Javascript                           https://en.wikipedia.org/wiki/JavaScript
    TYPESCRIPT,  // Typescript                           https://en.wikipedia.org/wiki/TypeScript
    PYTHON,      // Python                               https://en.wikipedia.org/wiki/Python_(programming_language)
    BASIC,       // BASIC                                https://en.wikipedia.org/wiki/BASIC
    BATCH,       // Batch scripts (Windows Command Line) https://en.wikipedia.org/wiki/Batch_file
    UNKNOWN,
}

pub fn select_lang(language: Supported) -> CommentSplitters {
    match language {
        Supported::RUST => RUST,
        Supported::JAVASCRIPT => JAVASCRIPT,
        Supported::TYPESCRIPT => TYPESCRIPT,
        _ => UNKNOWN
    }
}

pub struct CommentSplitters {
    pub wharf_id: String,
    pub supports_comments: bool,
    pub supports_block: bool,
    pub typed: bool,
    pub eol: String,
    pub block_start: String,
    pub block_end: String,
}


/*#
    @Name: RUST
    @Type: CommentSplitters
*/
pub const RUST: CommentSplitters = CommentSplitters {
    wharf_id: "#".to_string(),
    supports_comments: true,
    supports_block: true,
    typed: true,
    eol: "//".to_string(),
    block_start: "/*".to_string(),
    block_end: "*/".to_string(),
};



/*#
    @Name: TYPESCRIPT
    @Type: CommentSplitters
 */
pub const TYPESCRIPT: CommentSplitters = RUST;



/*#
    @Name: JAVASCRIPT
    @Type: CommentSplitters
*/
pub const JAVASCRIPT: CommentSplitters = CommentSplitters {
    wharf_id: "#".to_string(),
    supports_comments: true,
    supports_block: true,
    typed: false,
    eol: "//".to_string(),
    block_start: "/*".to_string(),
    block_end: "*/".to_string(),
};

/*#
    @Name: PYTHON
    @Type: CommentSplitters
*/
pub const PYTHON: CommentSplitters = CommentSplitters {
    wharf_id: "#".to_string(),
    supports_comments: true,
    supports_block: false,
    typed: false,
    eol: "#".to_string(),
    block_start: "".to_string(),
    block_end: "".to_string(),
};

/*#
    @Name: BASIC
    @Type: CommentSplitters
 */
pub const BASIC: CommentSplitters = CommentSplitters {
    wharf_id: "".to_string(),
    supports_comments: true,
    supports_block: false,
    typed: false,
    eol: "REM".to_string(),
    block_start: "".to_string(),
    block_end: "".to_string(),
};


/*#
    @Name: UNKNOWN
    @Type: CommentSplitters
*/
pub const UNKNOWN: CommentSplitters = CommentSplitters {
    wharf_id: "".to_string(),
    supports_comments: false,
    supports_block: false,
    typed: false,
    eol: "".to_string(),
    block_start: "".to_string(),
    block_end: "".to_string(),
};