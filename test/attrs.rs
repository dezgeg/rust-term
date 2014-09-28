extern crate termutils;
use std::io;

fn main () {
    termutils::info::init();
    println!("{}", termutils::info::exit_attribute_mode());
    let mut reader = io::stdin();
    loop {
        println!("Attribute?");
        let mut attr = reader.read_line().unwrap_or("".to_string());
        attr = attr.replace("\n", "");
        if attr.as_slice().starts_with("fg:") || attr.as_slice().starts_with("bg:") {
            let set = if attr.as_slice().starts_with("fg:") {
                |c| { println!("{}", termutils::info::set_a_foreground(c)) }
            }
            else {
                |c| { println!("{}", termutils::info::set_a_background(c)) }
            };

            match attr.as_slice().slice_from(3) {
                "black"   => set(termutils::info::ColorBlack),
                "red"     => set(termutils::info::ColorRed),
                "green"   => set(termutils::info::ColorGreen),
                "yellow"  => set(termutils::info::ColorYellow),
                "blue"    => set(termutils::info::ColorBlue),
                "magenta" => set(termutils::info::ColorMagenta),
                "cyan"    => set(termutils::info::ColorCyan),
                "white"   => set(termutils::info::ColorWhite),
                _          => (),
            }
        }
        else {
            match attr.as_slice() {
                "underline"   => println!("{}", termutils::info::enter_underline_mode()),
                "standout"    => println!("{}", termutils::info::enter_standout_mode()),
                "reverse"     => println!("{}", termutils::info::enter_reverse_mode()),
                "bold"        => println!("{}", termutils::info::enter_bold_mode()),
                "blink"       => println!("{}", termutils::info::enter_blink_mode()),
                "reset"       => println!("{}", termutils::info::exit_attribute_mode()),
                "reset_color" => println!("{}", termutils::info::orig_pair()),
                ""            => break,
                _              => (),
            }
        }
    }
    println!("{}", termutils::info::exit_attribute_mode());
}
