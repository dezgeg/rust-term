extern crate termutils;
use std::io;

fn loop_chars () {
    let mut reader = io::stdin();
    loop {
        let ch = reader.read_char();
        match ch {
            Ok('q') => break,
            Ok(ch)  => { io::stdout().write_char(ch); }
            _       => break,
        }
    }
}

fn main () {
    println!("In raw mode (q to exit):");
    termutils::ios::preserve(|| {
        termutils::ios::raw();
        loop_chars();
    });

    println!("\nIn normal mode (q to exit):");
    loop_chars();
}
