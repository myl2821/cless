extern crate ncurses;

use ncurses as nc;
use std::char;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Default)]
struct Context {
    pub lines: Vec<String>,
    pub scr_width: i32,
    pub scr_height: i32,
    pub buf_length: i32,
    pub x_offset: i32,
    pub y_offset: i32,
}

fn read_lines() -> Vec<String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\t{} <file>", args[0]);
        std::process::exit(1)
    }

    let mut lines: Vec<String> = vec![];
    let file = fs::File::open(Path::new(&args[1])).expect("unable to open file");
    let buffer = BufReader::new(file).lines();
    for line in buffer {
        if let Ok(l) = line {
            lines.push(l);
        }
    }

    lines
}

fn add_line(ctx: &mut Context, i: i32) -> bool {
    if ctx.y_offset + i >= ctx.lines.len() as i32 {
        return false;
    }
    let s = &ctx.lines[(ctx.y_offset + i) as usize];
    for c in s.chars() {
        let mut cur_x = 0;
        let mut cur_y = 0;
        nc::getyx(nc::stdscr(), &mut cur_y, &mut cur_x);

        if cur_y == ctx.scr_height {
            return false;
        }

        nc::addch(c as nc::chtype);
    }

    true
}

fn fresh_screen(ctx: &mut Context) {
    nc::clear();
    for i in 0..ctx.scr_height {
        if add_line(ctx, i) {
            // nc::printw(s);
            nc::addch('\n' as nc::chtype);
        } else {
            return;
        }
    }
    nc::mv(0, 0);
}

fn main() {
    let lines = read_lines();

    nc::initscr();
    nc::keypad(nc::stdscr(), true);
    nc::noecho();
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut ctx = Context::default();
    ctx.buf_length = lines.len() as i32;
    ctx.lines = lines;
    nc::getmaxyx(nc::stdscr(), &mut ctx.scr_height, &mut ctx.scr_width);

    loop {
        fresh_screen(&mut ctx);
        match nc::getch() {
            // j, up
            0x6a | nc::KEY_UP => {
                if ctx.y_offset < ctx.buf_length - 1 {
                    ctx.y_offset += 1;
                }
            }
            // k, down
            0x6b | nc::KEY_DOWN => {
                if ctx.y_offset > 0 {
                    ctx.y_offset -= 1;
                }
            }
            // q
            0x71 => break,
            _ => (),
        }
    }

    nc::endwin();
}