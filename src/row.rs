use ncurses as nc;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

/* Individual color handles. */
static COLOR_BACKGROUND: i16 = 16;
static COLOR_FOREGROUND: i16 = 17;
static COLOR_KEYWORD: i16 = 18;
static COLOR_TYPE: i16 = 19;
static COLOR_STORAGE: i16 = 20;
static COLOR_COMMENT: i16 = 21;
static COLOR_STRING: i16 = 22;
static COLOR_CHAR: i16 = 23;
static COLOR_NUMBER: i16 = 24;

/* Color pairs; foreground && background. */
static COLOR_PAIR_DEFAULT: i16 = 1;
static COLOR_PAIR_KEYWORD: i16 = 2;
static COLOR_PAIR_TYPE: i16 = 3;
static COLOR_PAIR_STORAGE: i16 = 4;
static COLOR_PAIR_COMMENT: i16 = 5;
static COLOR_PAIR_STRING: i16 = 6;
static COLOR_PAIR_CHAR: i16 = 7;
static COLOR_PAIR_NUMBER: i16 = 8;

/* Word delimiters. */
static WORD_LIMITS: &'static [u8] = &[
    ' ' as u8, '(' as u8, ')' as u8, ':' as u8, ';' as u8, '&' as u8, '+' as u8, '-' as u8,
    ',' as u8, '.' as u8, '@' as u8, '~' as u8, '\\' as u8, '\n' as u8, '\r' as u8, '\0' as u8,
    !0 as u8,
];

#[derive(Debug, Default)]
pub struct Row {
    pub raw: String,
    pub tokens: Vec<(String, nc::attr_t)>,
}

pub fn read_rows() -> Vec<Row> {
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

    rows
}
