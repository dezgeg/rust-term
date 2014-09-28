extern crate termutils;

fn main () {
    let (cols, rows) = termutils::ios::size();
    println!("tty: {:d} {:d}", cols as int, rows as int);
}
