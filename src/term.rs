#[link(name = "term",
       vers = "0.0.1",
       uuid = "55ed8b92-1054-4286-95b2-8e967f4fd51b",
       url  = "https://github.com/doy/rust-term")];

#[crate_type = "lib"];

use core::libc::c_int;

pub use ios::{cooked,cbreak,raw,echo,size};
use info::{escape,escape1,escape2};
use trie::Trie;

enum Keypress {
    KeyCharacter(char),
    KeyBackspace,
    KeyReturn,
    KeyTab,
    KeyCtrl(char),
    KeyF(int),
    KeyUp,
    KeyDown,
    KeyLeft,
    KeyRight,
    KeyHome,
    KeyEnd,
    KeyInsert,
    KeyDelete,
    KeyEscape,
}

enum Color {
    ColorBlack = 0,
    ColorRed,
    ColorGreen,
    ColorYellow,
    ColorBlue,
    ColorMagenta,
    ColorCyan,
    ColorWhite,
}

struct Term {
    priv r: Reader,
    priv w: Writer,
}

pub fn Term (cleanup: bool) -> Term {
    info::init();
    Term { r: Reader(cleanup), w: Writer(cleanup) }
}

impl Term {
    pub fn init_term_app (&mut self) {
        cbreak();
        echo(false);
        self.write(escape("smkx"));
        self.alternate_screen(true);
        self.clear();
        self.flush();
    }

    pub fn clear (&mut self) {
        self.w.clear();
    }

    pub fn move (&mut self, col: uint, row: uint) {
        self.w.move(col, row);
    }

    pub fn fg_color (&mut self, color: Color) {
        self.w.fg_color(color);
    }

    pub fn bg_color (&mut self, color: Color) {
        self.w.bg_color(color);
    }

    pub fn reset_color (&mut self) {
        self.w.reset_color();
    }

    pub fn cursor (&mut self, enabled: bool) {
        self.w.cursor(enabled);
    }

    pub fn alternate_screen (&mut self, enabled: bool) {
        self.w.alternate_screen(enabled);
    }

    pub fn write (&mut self, text: &str) {
        self.w.write(text);
    }

    pub fn flush (&mut self) {
        self.w.flush();
    }

    pub fn read (&mut self) -> Option<Keypress> {
        self.w.flush();
        self.r.read()
    }
}

struct Writer {
    priv buf: ~str,
    priv cleanup: bool,
}

fn Writer (cleanup: bool) -> Writer {
    Writer { buf: ~"", cleanup: cleanup }
}

impl Writer {
    fn clear (&mut self) {
        self.buf.push_str(escape("clear"));
    }

    fn move (&mut self, col: uint, row: uint) {
        if col == 0u && row == 0u {
            self.buf.push_str(escape("home"));
        }
        else {
            self.buf.push_str(escape2("cup", row as int, col as int));
        }
    }

    fn fg_color (&mut self, color: Color) {
        self.buf.push_str(escape1("setaf", color as int));
    }

    fn bg_color (&mut self, color: Color) {
        self.buf.push_str(escape1("setab", color as int));
    }

    fn reset_color (&mut self) {
        self.buf.push_str(escape("op"));
    }

    fn cursor (&mut self, enabled: bool) {
        if enabled {
            self.buf.push_str(escape("civis"));
        }
        else {
            self.buf.push_str(escape("cnorm"));
        }
    }

    fn alternate_screen (&mut self, enable: bool) {
        if enable {
            self.buf.push_str(escape("smcup"));
        }
        else {
            self.buf.push_str(escape("rmcup"));
        }
    }

    fn write (&mut self, text: &str) {
        self.buf.push_str(text);
    }

    fn flush (&mut self) {
        print(self.buf);
        io::stdout().flush();
        self.buf = ~"";
    }
}

impl Drop for Writer {
    fn finalize (&self) {
        if self.cleanup {
            print(escape("rmcup"));
            print(escape("sgr0"));
            print(escape("cnorm"));
        }
    }
}

struct Reader {
    priv escapes: ~Trie<Keypress>,
    priv buf: ~str,
    priv cleanup: bool,
}

pub fn Reader (cleanup: bool) -> Reader {
    Reader { escapes: build_escapes_trie(), buf: ~"", cleanup: cleanup }
}

impl Reader {
    fn read (&mut self) -> Option<Keypress> {
        if self.buf.len() > 0 {
            return Some(self.next_key());
        }

        let first = util::timed_read(-1);
        if first.is_none() {
            return None;
        }

        let mut buf = str::from_char(*first.get_ref());
        loop {
            if !self.escapes.has_prefix(buf) {
                /* XXX i think this is a borrow check bug, should look into
                 * it at some point */
                //return match self.escapes.find(buf) {
                //    &Some(k) => { Some(k) }
                //    &None    => {
                //        self.unget(buf);
                //        self.read()
                //    }
                //}
                {
                    let k = self.escapes.find(buf);
                    if k.is_some() {
                        return *k;
                    }
                }
                self.unget(buf);
                return self.read();
            }

            match util::timed_read(1000000) {
                Some(next) => { buf.push_char(next) }
                None       => {
                    self.unget(buf);
                    return self.read();
                }
            }
        }
    }

    fn unget (&mut self, buf: &str) {
        self.buf.push_str(buf);
    }

    fn next_key (&mut self) -> Keypress {
        assert!(self.buf.len() > 0);
        for uint::range_rev(self.buf.len(), 0) |i| {
            match self.escapes.find(self.buf.slice(0, i)) {
                &Some(k) => {
                    for uint::range(0, i) |_| {
                        str::shift_char(&mut self.buf);
                    }
                    return k
                }
                &None    => { }
            }
        }
        let next = str::shift_char(&mut self.buf);
        return KeyCharacter(next);
    }
}

impl Drop for Reader {
    fn finalize (&self) {
        if self.cleanup {
            print(escape("rmkx"));
        }
    }
}

// XXX this whole thing needs to be able to deal with caps that don't exist
fn build_escapes_trie () -> ~Trie<Keypress> {
    let mut trie = ~Trie();

    trie.insert(escape("kbs"), KeyBackspace);
    trie.insert(escape("cr"),  KeyReturn);
    trie.insert(escape("ht"),  KeyTab);

    trie.insert(escape("kcuu1"), KeyUp);
    trie.insert(escape("kcud1"), KeyDown);
    trie.insert(escape("kcub1"), KeyLeft);
    trie.insert(escape("kcuf1"), KeyRight);

    trie.insert(escape("khome"), KeyHome);
    trie.insert(escape("kend"),  KeyEnd);
    trie.insert(escape("kich1"), KeyInsert);
    trie.insert(escape("kdch1"), KeyDelete);

    for uint::range(1, 12) |i| {
        trie.insert(escape(fmt!("kf%d", i as int)), KeyF(i as int));
    }

    for uint::range(1, 26) |i| {
        let s = str::from_char(i as char);
        if (trie.find(s).is_none()) {
            trie.insert(s, KeyCtrl(i as char));
        }
    }

    trie.insert(str::from_char(27 as char), KeyEscape);

    trie
}

pub fn isatty() -> bool {
    unsafe { c_isatty(0) as bool }
}

pub mod ios;
pub mod info;
mod util;
mod trie;

extern {
    #[link_name = "isatty"]
    fn c_isatty(fd: c_int) -> c_int;
}
