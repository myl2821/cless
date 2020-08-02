extern crate ncurses;

mod row;

use crate::row::Row;
use ncurses as nc;

#[derive(Debug, Default)]
struct Context {
    pub rows: Vec<Row>,
    pub scr_width: i32,
    pub scr_height: i32,
    pub buf_length: i32,
    pub x_offset: i32,
    pub y_offset: i32,
}

fn add_line(ctx: &mut Context, i: i32) -> bool {
    if ctx.y_offset + i >= ctx.rows.len() as i32 {
        return false;
    }
    let s = &ctx.rows[(ctx.y_offset + i) as usize].raw;
    for c in s.chars() {
        let mut cur_x = 0;
        let mut cur_y = 0;
        nc::getyx(nc::stdscr(), &mut cur_y, &mut cur_x);

        if cur_y == ctx.scr_height - 1 {
            return false;
        }

        nc::addch(c as nc::chtype);
    }

    true
}

fn refresh(ctx: &mut Context) {
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

fn prompt(ctx: &Context) {
    if ctx.y_offset == ctx.buf_length - 1 {
        nc::attron(nc::A_BOLD() | nc::A_REVERSE());
        nc::mv(ctx.scr_height - 1, 0);
        nc::printw("(END)");
        nc::attroff(nc::A_BOLD() | nc::A_REVERSE());
    }
}

fn main() {
    let lines = crate::row::read_rows();

    nc::initscr();
    nc::keypad(nc::stdscr(), true);
    nc::noecho();
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut ctx = Context::default();
    ctx.buf_length = lines.len() as i32;
    ctx.rows = lines;
    nc::getmaxyx(nc::stdscr(), &mut ctx.scr_height, &mut ctx.scr_width);

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
