extern crate termutils;
use std::io;

fn main () {
    print!("Enter password: ");
    termutils::ios::echo(false);
    let mut reader = io::stdin();
    let pass = reader.read_line().unwrap_or("nothing".to_string());
    termutils::ios::echo(true);
    println!("\nYour password is: {:s}", pass);
}
