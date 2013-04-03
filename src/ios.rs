use core::libc::{c_int,c_uint,c_void};
use util::guard;

pub fn cooked () -> int {
    unsafe { c::cooked() as int }
}

pub fn cbreak () -> int {
    unsafe { c::cbreak() as int }
}

pub fn raw () -> int {
    unsafe { c::raw() as int }
}

pub fn echo (enable: bool) -> int {
    unsafe { c::echo(enable as c_int) as int }
}

pub fn preserve<T> (body: &fn () -> T) -> T {
    let orig = unsafe { c::get() };
    do guard(|| { unsafe { c::set(orig) } }) {
        body()
    }
}

pub fn size() -> (uint, uint) {
    let cols: c_uint = 0;
    let rows: c_uint = 0;
    unsafe {
        c::size(&cols, &rows)
    }
    (cols as uint, rows as uint)
}

#[link_name = "termios_wrapper"]
extern mod c {
    fn cooked () -> c_int;
    fn cbreak () -> c_int;
    fn raw () -> c_int;
    fn echo (enable: c_int) -> c_int;

    fn get() -> *c_void;
    fn set(t: *c_void);

    fn size(cols: *c_uint, rows: *c_uint);
}
