extern crate termutils;
use std::os;

fn main () {
    if os::args().len() < 2 {
        fail!("usage: tput <terminfo capability>");
    }

    termutils::info::init();
    let ref attr = os::args()[1];
    let escape = termutils::info::escape(attr.as_slice()).expect(
        format!("{:s} is not supported on this terminal", *attr).as_slice()
    );
    println!("{}", escape);
}
