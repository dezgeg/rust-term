use info;
use ios::{cooked,cbreak,echo};
use trie::Trie;
use std::{str, uint, iter, io};
use std::io::File;
use std::path::posix::Path;

use util;

/// Keys that can be returned by `Term::read`.
#[deriving(PartialEq, Show)]
pub enum Keypress {
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

pub struct Term {
    // XXX: either use accessors or rename
    pub r: TermReader,
    pub w: TermWriter,
}

impl Term {
    /**
    * Creates a new `Term` instance.
    *
    * This can be used to manipulate the terminal for full screen
    * applications.
    */
    pub fn new () -> Term {
        info::init();

        cbreak();
        echo(false);

        // XXX: this sounds like hack
        let mut stream = io::File::open_mode(&Path::new("/dev/tty"), io::Open,
                io::ReadWrite).ok().expect("Open tty");

        // XXX need to come up with a better way to handle optional caps
        // should be able to use something like has_keypad_xmit or something
        let terms = ["smkx", /* "smcup", */ "sgr0", "cnorm"];
        for &cap in terms.iter() {
            match info::escape(cap) {
                Some(e) => { stream.write_str(e.as_slice());  }
                None    => (), // not a big deal if these don't exist
            }
        }

        Term {
            r: TermReader::new(),
            w: TermWriter::new(stream),
        }
    }

    /// Clears the screen.
    pub fn clear (&mut self) {
        self.w.clear();
    }

    /// Moves the cursor to (`col`, `row`).
    pub fn move_cursor (&mut self, col: uint, row: uint) {
        self.w.move_cursor(col, row);
    }

    /// Scrolls the text up.
    pub fn scroll_forward (&mut self, lines: uint) {
        self.w.scroll_forward(lines);
    }

    /// Scrolls the text down.
    pub fn scroll_reverse (&mut self, lines: uint) {
        self.w.scroll_reverse(lines);
    }

    /// Changes the currently active foreground color to `color`.
    pub fn fg_color (&mut self, color: info::Color) {
        self.w.fg_color(color);
    }

    /// Changes the currently active background color to `color`.
    pub fn bg_color (&mut self, color: info::Color) {
        self.w.bg_color(color);
    }

    /// Resets the foreground and background colors to the default.
    pub fn reset_color (&mut self) {
        self.w.reset_color();
    }

    /// Enables or disables underline mode.
    pub fn underline (&mut self, enabled: bool) {
        self.w.underline(enabled);
    }

    /// Enables or disables standout mode.
    pub fn standout (&mut self, enabled: bool) {
        self.w.standout(enabled);
    }

    /// Enables or disables reverse mode.
    pub fn reverse (&mut self, enabled: bool) {
        self.w.reverse(enabled);
    }

    /// Enables or disables bold mode.
    pub fn bold (&mut self, enabled: bool) {
        self.w.bold(enabled);
    }

    /// Enables or disables blink mode.
    pub fn blink (&mut self, enabled: bool) {
        self.w.blink(enabled);
    }

    /// Enables or disables visible cursor mode.
    pub fn cursor (&mut self, enabled: bool) {
        self.w.cursor(enabled);
    }

    /**
     * Switches to or from the alternate screen.
     *
     * This is used to provide a separate place to do all of the drawing for
     * a full screen app, so that at the end of the application, the terminal
     * will be restored to the original state.
     */
    pub fn alternate_screen (&mut self, enabled: bool) {
        self.w.alternate_screen(enabled);
    }

    /**
     * Write a string to the terminal.
     *
     * Due to buffering, using `io::stdout().write_str()` will not work properly. All text
     * written to the terminal must go through the `Term` object, or the state
     * of the screen will likely end up incorrect.
     */
    pub fn write (&mut self, text: &str) {
        self.w.write(text);
    }

    /**
     * Flush the data written so far to the terminal.
     *
     * This is also done implicitly before every call to `read`, so there's
     * not usually a reason to do it manually, other than edge cases such as
     * timed animations.
     */
    pub fn flush (&mut self) {
        self.w.flush();
    }

    /**
     * Read a keypress from the terminal.
     *
     * Returns `Some(Keypress)` if a key was read, and `None` if `stdin`
     * reaches `eof`.
     *
     * Note that most special keys are actually sequences of multiple
     * characters. This means that if a prefix of a special character key
     * sequence was read, it has to wait to see if there are more characters
     * coming, or if that character was the only key. Since most of these
     * multi-character sequences start with escape, there will be a delay in
     * reading a single `KeyEscape` keypress.
     *
     * Also, other special keys are represented as control keys, so for
     * instance, `^J` will likely return `KeyReturn` instead of
     * `KeyCtrl('j')`.
     */
    pub fn read (&mut self) -> Option<Keypress> {
        self.w.flush();
        self.r.read()
    }
}

// XXX - todo - reimplement, doesn't really belong in Drop
/*
impl Drop for Term {
    fn drop (&mut self) {
        // XXX need to come up with a better way to handle optional caps
        // should be able to use something like has_keypad_xmit or something
        let terms = ["rmkx", /* "rmcup", */ "sgr0", "cnorm"];
        for &cap in terms.iter() {
            match info::escape(cap) {
                Some(e) => { self.w.stream.write_str(e.as_slice()); }
                None    => (), // not a big deal if these don't exist
            }
        }

        // XXX should really restore the previous termios mode...
        echo(true);
        cooked();
    }
}
*/

pub struct TermWriter {
    buf: String,
    state: AttrState,
    stream: File,
}

struct AttrState {
    fg: Option<info::Color>,
    bg: Option<info::Color>,
    underline: bool,
    standout: bool,
    reverse: bool,
    bold: bool,
    blink: bool,
}

fn AttrState () -> AttrState {
    AttrState {
        fg: None,
        bg: None,
        underline: false,
        standout: false,
        reverse: false,
        bold: false,
        blink: false,
    }
}

impl TermWriter {
    fn new (stream: File) -> TermWriter {
        TermWriter {
            buf: "".to_string(),
            state: AttrState(),
            stream: stream,
        }
    }

    pub fn clear (&mut self) {
        self.buf.push_str(info::clear_screen().as_slice());
    }

    pub fn move_cursor (&mut self, col: uint, row: uint) {
        if col == 0u && row == 0u {
            self.buf.push_str(info::cursor_home().as_slice());
        }
        else {
            self.buf.push_str(info::cursor_address(row, col).as_slice());
        }
    }

    pub fn scroll_forward (&mut self, lines: uint) {
        if lines == 1 {
            self.buf.push_str(info::scroll_forward().as_slice());
        } else {
            self.buf.push_str(info::scroll_forward_multiple(lines).as_slice());
        }
    }

    pub fn scroll_reverse (&mut self, lines: uint) {
        if lines == 1 {
            self.buf.push_str(info::scroll_reverse().as_slice());
        } else {
            self.buf.push_str(info::scroll_reverse_multiple(lines).as_slice());
        }
    }

    pub fn fg_color (&mut self, color: info::Color) {
        match self.state.fg {
            Some(c) if c == color => {}
            _                     => {
                self.state.fg = Some(color);
                self.buf.push_str(info::set_a_foreground(color).as_slice());
            }
        }
    }

    pub fn bg_color (&mut self, color: info::Color) {
        match self.state.bg {
            Some(c) if c == color => {}
            _                     => {
                self.state.bg = Some(color);
                self.buf.push_str(info::set_a_background(color).as_slice());
            }
        }
    }

    pub fn underline (&mut self, enabled: bool) {
        if self.state.underline != enabled {
            self.state.underline = enabled;
            if enabled {
                self.buf.push_str(info::enter_underline_mode().as_slice());
            }
            else {
                self.buf.push_str(info::exit_underline_mode().as_slice());
            }
        }
    }

    pub fn standout (&mut self, enabled: bool) {
        if self.state.standout != enabled {
            self.state.standout = enabled;
            if enabled {
                self.buf.push_str(info::enter_standout_mode().as_slice());
            }
            else {
                self.buf.push_str(info::exit_standout_mode().as_slice());
            }
        }
    }

    pub fn reverse (&mut self, enabled: bool) {
        if self.state.reverse != enabled {
            self.state.reverse = enabled;
            if enabled {
                self.buf.push_str(info::enter_reverse_mode().as_slice());
            }
            else {
                self.apply_state();
            }
        }
    }

    pub fn bold (&mut self, enabled: bool) {
        if self.state.bold != enabled {
            self.state.bold = enabled;
            if enabled {
                self.buf.push_str(info::enter_bold_mode().as_slice());
            }
            else {
                self.apply_state();
            }
        }
    }

    pub fn blink (&mut self, enabled: bool) {
        if self.state.blink != enabled {
            self.state.blink = enabled;
            if enabled {
                self.buf.push_str(info::enter_blink_mode().as_slice());
            }
            else {
                self.apply_state();
            }
        }
    }

    pub fn reset_color (&mut self) {
        self.state.fg = None;
        self.state.bg = None;
        self.buf.push_str(info::orig_pair().as_slice());
    }

    pub fn reset_attributes (&mut self) {
        self.state = AttrState();
        self.apply_state();
    }

    pub fn apply_state (&mut self) {
        self.buf.push_str(info::exit_attribute_mode().as_slice());
        match self.state.fg {
            Some(c) => self.fg_color(c),
            None    => (),
        }
        match self.state.bg {
            Some(c) => self.bg_color(c),
            None    => (),
        }
        if self.state.underline {
            self.underline(true);
        }
        if self.state.standout {
            self.standout(true);
        }
        if self.state.reverse {
            self.reverse(true);
        }
        if self.state.bold {
            self.bold(true);
        }
        if self.state.blink {
            self.blink(true);
        }
    }

    pub fn cursor (&mut self, enabled: bool) {
        if enabled {
            self.buf.push_str(info::cursor_invisible().as_slice());
        }
        else {
            self.buf.push_str(info::cursor_normal().as_slice());
        }
    }

    pub fn alternate_screen (&mut self, enabled: bool) {
        if enabled {
            self.buf.push_str(info::enter_ca_mode().as_slice());
        }
        else {
            self.buf.push_str(info::exit_ca_mode().as_slice());
        }
    }

    pub fn write (&mut self, text: &str) {
        self.buf.push_str(text);
    }

    pub fn flush (&mut self) {
        self.stream.write_str(self.buf.as_slice());
        self.stream.flush();
        self.buf = "".to_string();
    }
}

pub struct TermReader {
    escapes: Trie<Keypress>,
    buf: String,
}

impl TermReader {
    fn new () -> TermReader {
        TermReader { escapes: build_escapes_trie(), buf: "".to_string() }
    }

    pub fn read (&mut self) -> Option<Keypress> {
        if self.buf.len() > 0 {
            return Some(self.next_key());
        }

        let first = util::timed_read(-1);
        if first.is_none() {
            return None;
        }

        let mut buf = str::from_char(*first.get_ref());
        loop {
            if !self.escapes.has_prefix(buf.as_slice()) {
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
                    let k = self.escapes.find(buf.as_slice());
                    if k.is_some() {
                        return *k;
                    }
                }
                self.unget(buf.as_slice());
                return self.read();
            }

            match util::timed_read(1000000) {
                Some(next) => { buf.push_char(next) }
                None       => {
                    self.unget(buf.as_slice());
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
        for i in iter::range(0, self.buf.len()) {
            match self.escapes.find(self.buf.as_slice().slice(0, i)) {
                &Some(k) => {
                    for _ in iter::range(0, i) {
                        self.buf.shift_char();
                    }
                    return k
                }
                &None    => { }
            }
        }
        let next = self.buf.shift_char().unwrap();
        return KeyCharacter(next);
    }
}

// XXX this whole thing needs to be able to deal with caps that don't exist
fn build_escapes_trie () -> Trie<Keypress> {
    let mut trie = Trie();

    trie.insert(info::key_backspace().as_slice(), KeyBackspace);
    trie.insert("\n",  KeyReturn);
    trie.insert("\t",  KeyTab);

    trie.insert(info::key_up().as_slice(), KeyUp);
    trie.insert(info::key_down().as_slice(), KeyDown);
    trie.insert(info::key_left().as_slice(), KeyLeft);
    trie.insert(info::key_right().as_slice(), KeyRight);

    trie.insert(info::key_home().as_slice(), KeyHome);
    trie.insert(info::key_end().as_slice(),  KeyEnd);
    trie.insert(info::key_ic().as_slice(), KeyInsert);
    trie.insert(info::key_dc().as_slice(), KeyDelete);

    for i in iter::range(1u, 12u) {
        trie.insert(info::key_f(i).as_slice(), KeyF(i as int));
    }

    for i in iter::range(1u8, 26u8) {
        let s = str::from_char(i as char);
        if (trie.find(s.as_slice()).is_none()) {
            trie.insert(s.as_slice(), KeyCtrl(i as char));
        }
    }

    trie.insert(str::from_char(27u8 as char).as_slice(), KeyEscape);

    trie
}
