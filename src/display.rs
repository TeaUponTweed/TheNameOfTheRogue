use std;
use futures::Future;
use std::char;
use ncurses::*;
use libc::{ c_char, c_int, c_short, c_uint, c_ulong, c_void };

fn get_curses_input(max_delay: i32) -> Future<Item=(i32, i32), Error=std::io::Error> {
    use std::io;
    // use *;
    let ch = wget_wch(stdscr);
    match ch {
        Some(WchResult::KeyCode(KEY_MOUSE)) => {
            let mut mevent = MEVENT (0 as c_short, 0 as c_int, 0 as c_int, 0 as c_int, 0 as u64);
            let err_code = getmouse(&mut mevent);
            if !err_code {
                return Err(io::Error::new(io::ErrorKind::Other, "Couldn't get mouse event"));
            } else {
                return (mevent.x, mevent.y);
            }
        },
        _ => {},
    }
    Err(io::Error::new(io::ErrorKind::Other, "No input"))
}

fn ncurses_setup() {
    let locale_conf = LcCategory::all;
    setlocale(locale_conf, "en_US.UTF-8");
    initscr();
    mousemask(ALL_MOUSE_EVENTS as u64, None);
    keypad(stdscr, true);
    noecho();
}
