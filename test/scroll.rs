#![feature(globs)]
extern crate termutils;
use termutils::ios;
use termutils::hexes::*;

fn main () {
    termutils::ios::preserve(|| {
        let (_, rows) = termutils::ios::size();
        let mut term = Term::new();

        for i in range(1, rows) {
            term.write(format!("{}\n", i).as_slice());
        }
        term.write(format!("{}", rows).as_slice());

        loop {
            match term.read() {
                Some(KeyCharacter('q')) => break,
                Some(KeyUp) => {
                    term.move_cursor(0, 0);
                    term.scroll_reverse(2);
                }
                Some(KeyDown) => {
                    term.move_cursor(0, rows - 1);
                    term.scroll_forward(2);
                }
                _ => (),
            };
        }
    });
}
