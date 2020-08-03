#[macro_use]
extern crate lazy_static;
extern crate ncurses;

mod language;
mod row;

use crate::language::Language;
use crate::row::Row;
use nc::{attr_t, COLOR_PAIR};
use ncurses as nc;

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
static COLOR_PAIR_PUNCTUATION: i16 = 2;

#[derive(Debug, Default)]
struct Context {
    pub lang: Option<&'static Language>,
    pub rows: Vec<Row>,
    pub scr_width: i32,
    pub scr_height: i32,
    pub buf_length: i32,
    pub x_offset: i32,
    pub y_offset: i32,
    in_multi_comment: bool,
    in_single_comment: bool,
    in_string: bool,
    in_char: bool,
}

impl Context {
    fn highlight_word(&mut self, word: &str) -> attr_t {
        if !self.lang.is_some() {
            return 0;
        }

        let lang = self.lang.unwrap();

        /* Multi-line Comments. */
        if let Some((start, end)) = lang.multi_line_comment {
            if self.in_multi_comment && !word.contains(end) {
                return COLOR_PAIR(COLOR_PAIR_COMMENT);
            } else if self.in_multi_comment && word.contains(end) {
                self.in_multi_comment = false;
                return COLOR_PAIR(COLOR_PAIR_COMMENT);
            } else if !self.in_string
                && !self.in_multi_comment
                && word.contains(start)
                && !word.contains(&format!("\"{}", start))
            {
                self.in_multi_comment = true;
                return COLOR_PAIR(COLOR_PAIR_COMMENT);
            }
        }

        /* Single-line Comments. */
        if let Some(slc) = lang.single_line_comment {
            if self.in_single_comment {
                return COLOR_PAIR(COLOR_PAIR_COMMENT);
            } else if !self.in_string
                && !self.in_single_comment
                && word.contains(slc)
                && !word.contains(&format!("\"{}", slc))
            {
                self.in_single_comment = true;
                return COLOR_PAIR(COLOR_PAIR_COMMENT);
            }
        }

        /* Strings. */
        if !self.in_char {
            if self.in_string && !word.contains("\"") {
                return COLOR_PAIR(COLOR_PAIR_STRING);
            } else if self.in_string && word.contains("\"") {
                self.in_string = false;
                return COLOR_PAIR(COLOR_PAIR_STRING);
            } else if !self.in_string && word.contains("\"") {
                /* If the same quote is found from either direction
                 * then it's the only quote in the string. */
                if word.find('\"') == word.rfind('\"') {
                    self.in_string = true;
                }
                return COLOR_PAIR(COLOR_PAIR_STRING);
            }
        }

        /* Chars. */
        if self.in_char && !word.contains("\'") {
            return COLOR_PAIR(COLOR_PAIR_CHAR);
        } else if self.in_char && word.contains("\'") {
            self.in_char = false;
            return COLOR_PAIR(COLOR_PAIR_CHAR);
        } else if !self.in_char && word.contains("\'") && !word.contains("static") {
            /* If the same quote is found from either direction
             * then it's the only quote in the string. */
            if word.find('\'') == word.rfind('\'') {
                self.in_char = true;
            }
            return COLOR_PAIR(COLOR_PAIR_CHAR);
        }

        if word.is_empty() {
            return 0;
        }

        /* If it starts with a number, it is a number. */
        if word.as_bytes()[0] >= '0' as u8 && word.as_bytes()[0] <= '9' as u8 {
            return COLOR_PAIR(COLOR_PAIR_NUMBER);
        }

        if lang.keywords.contains(&word) {
            return COLOR_PAIR(COLOR_PAIR_KEYWORD);
        }

        if lang.types.contains(&word) {
            return COLOR_PAIR(COLOR_PAIR_TYPE);
        }

        if lang.storages.contains(&word) {
            return COLOR_PAIR(COLOR_PAIR_STORAGE);
        }

        match word {
            /* Punctuation. */
            "+" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "-" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "*" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "/" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "%" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "^" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "!" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "&" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "|" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "&&" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "||" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "<<" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            ">>+" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "+=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "-=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "*=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "/=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "%=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "^=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "&=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "|=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "<<=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            ">>=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "==" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "!=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            ">" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "<" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            ">=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "<=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "@" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "_" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "." => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            ".." => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "..." => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "..=" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "," => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            ";" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "::" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "->" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "=>" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "#" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "$" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),
            "?" => COLOR_PAIR(COLOR_PAIR_PUNCTUATION),

            /* Not something we need to highlight. */
            _ => 0,
        }
    }

    pub fn parse(&mut self) {
        self.in_multi_comment = false;
        self.in_string = false;
        self.in_char = false;

        let mut rows = self.rows.clone();
        for row in &mut rows {
            self.in_single_comment = false;
            let words = row.split();
            for (word, delimiter) in words {
                let attr = self.highlight_word(word.as_ref());
                let delim = format!("{}", delimiter);
                let delim_attr = self.highlight_word(&delim);
                row.tokens.push((word, attr));
                row.tokens.push((delim, delim_attr));
            }
        }

        self.rows = rows;
    }

    fn initialize() {
        nc::start_color();
        nc::init_color(COLOR_BACKGROUND, 0, 0, 0);
        nc::init_color(COLOR_FOREGROUND, 142 * 4, 161 * 4, 161 * 4);
        nc::init_color(COLOR_KEYWORD, 130 * 4, 151 * 4, 0);
        nc::init_color(COLOR_TYPE, 197 * 4, 73 * 4, 27 * 4);
        nc::init_color(COLOR_STORAGE, 219 * 4, 51 * 4, 47 * 4);
        nc::init_color(COLOR_COMMENT, 33 * 4, 138 * 4, 206 * 4);
        nc::init_color(COLOR_STRING, 34 * 4, 154 * 4, 142 * 4);
        nc::init_color(COLOR_CHAR, 34 * 4, 154 * 4, 142 * 4);
        nc::init_color(COLOR_NUMBER, 236 * 4, 107 * 4, 83 * 4);

        nc::init_pair(COLOR_PAIR_DEFAULT, COLOR_FOREGROUND, COLOR_BACKGROUND);
        nc::init_pair(COLOR_PAIR_KEYWORD, COLOR_KEYWORD, COLOR_BACKGROUND);
        nc::init_pair(COLOR_PAIR_TYPE, COLOR_TYPE, COLOR_BACKGROUND);
        nc::init_pair(COLOR_PAIR_STORAGE, COLOR_STORAGE, COLOR_BACKGROUND);
        nc::init_pair(COLOR_PAIR_COMMENT, COLOR_COMMENT, COLOR_BACKGROUND);
        nc::init_pair(COLOR_PAIR_STRING, COLOR_STRING, COLOR_BACKGROUND);
        nc::init_pair(COLOR_PAIR_CHAR, COLOR_CHAR, COLOR_BACKGROUND);
        nc::init_pair(COLOR_PAIR_NUMBER, COLOR_NUMBER, COLOR_BACKGROUND);
    }
}

fn add_line(ctx: &mut Context, i: i32) -> bool {
    if ctx.y_offset + i >= ctx.rows.len() as i32 {
        return false;
    }
    let tokens = &ctx.rows[(ctx.y_offset + i) as usize].tokens;
    for (token, attr) in tokens {
        let mut cur_x = 0;
        let mut cur_y = 0;
        nc::getyx(nc::stdscr(), &mut cur_y, &mut cur_x);

        if cur_y == ctx.scr_height - 1 {
            return false;
        }

        nc::attr_on(*attr);
        nc::addstr(token);
        nc::attr_off(*attr);
    }

    true
}

fn refresh(ctx: &mut Context) {
    nc::clear();
    for i in 0..ctx.scr_height {
        if add_line(ctx, i) {
            nc::addch('\n' as nc::chtype);
        } else {
            return;
        }
    }
    nc::mv(0, 0);
}

fn prompt(ctx: &Context) {
    if ctx.y_offset == ctx.buf_length - 1 {
        nc::attron(nc::A_BOLD() | nc::A_REVERSE());
        nc::mv(ctx.scr_height - 1, 0);
        nc::addstr("(END)");
        nc::attroff(nc::A_BOLD() | nc::A_REVERSE());
    }
}

fn main() {
    let (lines, lang) = crate::row::read_rows();

    nc::initscr();
    nc::keypad(nc::stdscr(), true);
    nc::noecho();
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    Context::initialize();

    let mut ctx = Context::default();
    ctx.buf_length = lines.len() as i32;
    ctx.rows = lines;
    ctx.lang = lang;
    nc::getmaxyx(nc::stdscr(), &mut ctx.scr_height, &mut ctx.scr_width);
    ctx.parse();

    loop {
        refresh(&mut ctx);
        prompt(&ctx);
        match nc::getch() {
            // j, down
            0x6a | nc::KEY_DOWN => {
                if ctx.y_offset < ctx.buf_length - 1 {
                    ctx.y_offset += 1;
                }
            }
            // k, up
            0x6b | nc::KEY_UP => {
                if ctx.y_offset > 0 {
                    ctx.y_offset -= 1;
                }
            }
            // f, z
            0x66 | 0x7a => {
                if ctx.y_offset + ctx.scr_height < ctx.buf_length - 1 {
                    ctx.y_offset += ctx.scr_height;
                } else {
                    ctx.y_offset = ctx.buf_length - 1;
                }
            }
            // b, w
            0x62 | 0x77 => {
                if ctx.y_offset > ctx.scr_height {
                    ctx.y_offset -= ctx.scr_height;
                } else {
                    ctx.y_offset = 0;
                }
            }
            // q
            0x71 => break,
            _ => (),
        }
    }

    nc::endwin();
}
