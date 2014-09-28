extern crate termutils;
use std::os;

fn main () {
    match os::args()[1].as_slice() {
        "echo"   => termutils::ios::echo(true),
        "noecho" => termutils::ios::echo(false),
        _         => fail!("unknown argument"),
    };
}
