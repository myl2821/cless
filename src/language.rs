#[derive(Debug)]
pub struct Language {
    pub single_line_comment: Option<&'static str>,
    pub multi_line_comment: Option<(&'static str, &'static str)>,
    pub keywords: Vec<&'static str>,
    pub types: Vec<&'static str>,
    pub storages: Vec<&'static str>,
}

lazy_static! {
    pub static ref RUST: Language = Language {
        single_line_comment: Some("//"),
        multi_line_comment: Some(("/*", "*/")),
        keywords: vec![
            "as", "async", "await", "break", "continue", "crate", "do", "dyn", "else", "enum",
            "extern", "false", "fn", "for", "in", "if", "impl", "let", "log", "loop", "match",
            "mod", "move", "once", "priv", "pub", "return", "struct", "super", "trait", "type",
            "unsafe", "use", "union", "while", "where",
        ],
        types: vec![
            "int", "uint", "char", "bool", "u8", "u16", "u32", "u64", "i16", "i32", "i64", "f32",
            "f64", "str", "self", "Self", "true",
        ],
        storages: vec!["const", "ref", "mut", "static"],
    };
    pub static ref C: Language = Language {
        single_line_comment: Some("//"),
        multi_line_comment: Some(("/*", "*/")),
        keywords: vec![
            "switch", "if", "while", "for", "break", "continue", "return", "else", "struct",
            "union", "typedef", "static", "enum", "class", "case",
        ],
        types: vec!["int", "long", "double", "float", "char", "unsigned", "signed", "void"],
        storages: vec!["const", "static"],
    };
}
