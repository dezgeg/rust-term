extern crate termutils;
use termutils::hexes::Term;
use termutils::hexes::{KeyCharacter};

fn main () {
    termutils::ios::preserve(|| {
        let mut term = Term::new();
        loop {
            let k = match term.read() {
                Some(KeyCharacter('q')) => break,
                Some(key) => key,
                None      => break,
            };
            println!("Got key: {}", k);
        }
    });
}
