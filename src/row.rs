use crate::language::{Language, RUST};
use ncurses as nc;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

/* Word delimiters. */
static WORD_LIMITS: &'static [char] = &[
    ' ', '(', ')', ':', ';', '&', '+', '-', ',', '.', '@', '~', '\\', '\n', '\r', '\0', !0 as char,
];

#[derive(Clone, Debug, Default)]
pub struct Row {
    pub raw: String,
    pub tokens: Vec<(String, nc::attr_t)>,
}

impl Row {
    pub fn split(&self) -> Vec<(String, char)> {
        let mut words = vec![];
        let mut word = vec![];
        for c in self.raw.chars() {
            if !WORD_LIMITS.contains(&c) {
                word.push(c);
            } else {
                words.push((word.into_iter().collect(), c));
                word = vec![];
            }
        }

        words.push((word.into_iter().collect(), ' '));
        words
    }
}

pub fn read_rows() -> (Vec<Row>, Option<&'static Language>) {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\t{} <file>", args[0]);
        std::process::exit(1)
    }

    let mut rows: Vec<Row> = vec![];
    let file = fs::File::open(Path::new(&args[1])).expect("unable to open file");
    let buffer = BufReader::new(file).lines();
    for line in buffer {
        if let Ok(l) = line {
            rows.push(Row {
                raw: l,
                tokens: vec![],
            });
        }
    }

    let lang: Option<&'static Language> = match Path::new(&args[1]).extension() {
        Some(os_str) => match os_str.to_str() {
            Some("rs") => Some(&RUST),
            _ => None,
        },
        _ => None,
    };

    (rows, lang)
}
