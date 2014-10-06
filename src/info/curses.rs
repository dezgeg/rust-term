use std::str;
use std::ptr;
use std::c_str;
use std::libc::{c_char,c_int,c_long};

mod c {
use std::libc::{c_char,c_int,c_long};
#[link(name = "curses")]
    extern {
        pub fn setupterm (term: *mut c_char, fd: c_int, errret: *mut c_int) -> c_int;
        pub fn tigetstr (s: *mut c_char) -> *mut c_char;
        pub fn tparm (s: *mut c_char,
                a1: c_long, a2: c_long, a3: c_long,
                a4: c_long, a5: c_long, a6: c_long,
                a7: c_long, a8: c_long, a9: c_long) -> *mut c_char;
    }
}


/// The default colors available on a terminal emulator.
#[deriving(Eq)]
pub enum Color {
    ColorBlack = 0,
    ColorRed,
    ColorGreen,
    ColorYellow,
    ColorBlue,
    ColorMagenta,
    ColorCyan,
    ColorWhite,
}

/**
 * Initialize the terminfo database.
 *
 * This must be called before any functions from this module are used. The
 * current terminal is determined by looking at the `TERM` environment
 * variable.
 */
pub fn init () {
    unsafe { c::setupterm(ptr::null(), 1, ptr::null()) };
}

macro_rules! def_escape(
    ($name:ident -> $escape:expr) => (
        pub fn $name () -> ~str {
            let attr = $escape;
            match escape(attr) {
                Some(e) => e,
                None    => fail!(format!("{:s} is not supported on this terminal",
                                      attr)),
            }
        }
    );
    ($name:ident -> $escape:expr, $ty1:ident) => (
        pub fn $name (p1: $ty1) -> ~str {
            let attr = $escape;
            match escape1(attr, p1 as int) {
                Some(e) => e,
                None    => fail!(format!("{:s} is not supported on this terminal",
                                      attr)),
            }
        }
    );
    ($name:ident -> $escape:expr, $ty1:ident, $ty2:ident) => (
        pub fn $name (p1: $ty1, p2: $ty2) -> ~str {
            let attr = $escape;
            match escape2(attr, p1 as int, p2 as int) {
                Some(e) => e,
                None    => fail!(format!("{:s} is not supported on this terminal",
                                      attr)),
            }
        }
    );
)

// XXX macros can't take attributes yet (including documentation), so change
// these to /// once that is fixed

// The terminal escape to clear the screen.
def_escape!(clear_screen         -> "clear")
// The terminal escape to set the foreground color to `p1`.
def_escape!(set_a_foreground     -> "setaf", Color)
// The terminal escape to set the background color to `p1`.
def_escape!(set_a_background     -> "setab", Color)
// The terminal escape to reset the foreground and background colors.
def_escape!(orig_pair            -> "op")
// The terminal escape to reset all attributes.
def_escape!(exit_attribute_mode  -> "sgr0")
// The terminal escape to move the cursor to the top left of the screen.
def_escape!(cursor_home          -> "home")
// The terminal escape to move the cursor to (`p1`, `p2`).
def_escape!(cursor_address       -> "cup", uint, uint)
// The terminal escape to scroll text up.
def_escape!(scroll_forward       -> "ind")
// The terminal escape to scroll text up multiple lines.
def_escape!(scroll_forward_multiple -> "indn", uint)
// The terminal escape to scroll text down.
def_escape!(scroll_reverse       -> "ri")
// The terminal escape to scroll text down multiple lines.
def_escape!(scroll_reverse_multiple -> "rin", uint)
// The terminal escape to enable underline mode.
def_escape!(enter_underline_mode -> "smul")
// The terminal escape to disable underline mode.
def_escape!(exit_underline_mode  -> "rmul")
// The terminal escape to enable standout mode.
def_escape!(enter_standout_mode  -> "smso")
// The terminal escape to disable standout mode.
def_escape!(exit_standout_mode   -> "rmso")
// The terminal escape to enable reverse video mode.
def_escape!(enter_reverse_mode   -> "rev")
// The terminal escape to enable bold mode.
def_escape!(enter_bold_mode      -> "bold")
// The terminal escape to enable blink mode.
def_escape!(enter_blink_mode     -> "blink")
// The terminal escape to make the cursor invisible.
def_escape!(cursor_invisible     -> "civis")
// The terminal escape to make the cursor visible.
def_escape!(cursor_normal        -> "cnorm")
// The terminal escape to enable the alternate screen.
def_escape!(enter_ca_mode        -> "smcup")
// The terminal escape to disable the alternate screen.
def_escape!(exit_ca_mode         -> "rmcup")
// The terminal escape to enter keypad mode.
def_escape!(keypad_xmit          -> "smkx")
// The terminal escape to leave keypad mode.
def_escape!(keypad_local         -> "rmkx")

// The terminal escape generated by the backspace key.
def_escape!(key_backspace   -> "kbs")
// The terminal escape generated by the return key.
def_escape!(carriage_return -> "cr")
// The terminal escape generated by the tab key.
def_escape!(tab             -> "ht")
// The terminal escape generated by the up arrow key.
def_escape!(key_up          -> "kcuu1")
// The terminal escape generated by the down arrow key.
def_escape!(key_down        -> "kcud1")
// The terminal escape generated by the left arrow key.
def_escape!(key_left        -> "kcub1")
// The terminal escape generated by the right arrow key.
def_escape!(key_right       -> "kcuf1")
// The terminal escape generated by the home key.
def_escape!(key_home        -> "khome")
// The terminal escape generated by the end key.
def_escape!(key_end         -> "kend")
// The terminal escape generated by the insert key.
def_escape!(key_ic          -> "kich1")
// The terminal escape generated by the delete key.
def_escape!(key_dc          -> "kdch1")
// The terminal escape generated by the F1 key.
def_escape!(key_f1          -> "kf1")
// The terminal escape generated by the F2 key.
def_escape!(key_f2          -> "kf2")
// The terminal escape generated by the F3 key.
def_escape!(key_f3          -> "kf3")
// The terminal escape generated by the F4 key.
def_escape!(key_f4          -> "kf4")
// The terminal escape generated by the F5 key.
def_escape!(key_f5          -> "kf5")
// The terminal escape generated by the F6 key.
def_escape!(key_f6          -> "kf6")
// The terminal escape generated by the F7 key.
def_escape!(key_f7          -> "kf7")
// The terminal escape generated by the F8 key.
def_escape!(key_f8          -> "kf8")
// The terminal escape generated by the F9 key.
def_escape!(key_f9          -> "kf9")
// The terminal escape generated by the F10 key.
def_escape!(key_f10         -> "kf10")
// The terminal escape generated by the F11 key.
def_escape!(key_f11         -> "kf11")
// The terminal escape generated by the F12 key.
def_escape!(key_f12         -> "kf12")

/// The terminal escape generated by the F<`n`> key.
pub fn key_f (n: uint) -> Box<str> {
    let attr = format!("kf{:?}", n);
    match escape(attr) {
        Some(e) => e,
        None    => fail!(format!("{:s} is not supported on this terminal", attr)),
    }
}

/// The terminal escape corresponding to the `name` terminfo capability.
pub fn escape (name: &str) -> Option<Box<str>> {
    unsafe {
        let c_name =  name.to_c_str();
        let e = tigetstr(c_name.unwrap());
        if e == ptr::null() {
            None
        }
        else {
            Some(str::raw::from_c_str(e))
        }
    }
}

/**
 * The terminal escape corresponding to the `name` terminfo capability.
 *
 * This capability must take one parameter, which should be passed as `p1`.
 */
pub fn escape1 (name: &str, p1: int) -> Option<Box<str>> {
    unsafe {
        let c_name =  name.to_c_str();
        let e = tigetstr(c_name.unwrap());
        if e == ptr::null() {
            None
        }
        else {
            Some(str::raw::from_c_str(tparm1(e, p1)))
        }
    }
}

/**
 * The terminal escape corresponding to the `name` terminfo capability.
 *
 * This capability must take two parameters, which should be passed as `p1`
 * and `p2`.
 */
pub fn escape2 (name: &str, p1: int, p2: int) -> Option<Box<str>> {
    unsafe {
        let c_name =  name.to_c_str();
        let e = tigetstr(c_name.unwrap());
        if e == ptr::null() {
            None
        }
        else {
            Some(str::raw::from_c_str(tparm2(e, p1, p2)))
        }
    }
}

unsafe fn tigetstr (name: *mut c_char) -> *mut c_char {
    let c_out = c::tigetstr(name);
    if c_out as int == -1 {
        fail!(format!("{:s} is not a terminal capability",
                   unsafe { str::raw::from_c_str(name) }));
    }
    c_out
}

unsafe fn tparm1 (name: *mut c_char, p1: int) -> *mut c_char {
    let ret = c::tparm(name, p1 as c_long, 0, 0, 0, 0, 0, 0, 0, 0);
    if ret == ptr::null() {
        fail!(format!("Couldn't assemble parameters with {:s} {:d}",
                   unsafe { str::raw::from_c_str(name) }, p1));
    }
    ret
}

unsafe fn tparm2 (name: *mut c_char, p1: int, p2: int) -> *mut c_char {
    let ret = c::tparm(name, p1 as c_long, p2 as c_long, 0, 0, 0, 0, 0, 0, 0);
    if ret == ptr::null() {
        fail!(format!("Couldn't assemble parameters with {:s} {:d} {:d}",
                   unsafe { str::raw::from_c_str(name) }, p1, p2));
    }
    ret
}
