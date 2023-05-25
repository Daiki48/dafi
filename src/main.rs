use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", termion::clear::All).unwrap();

    for key in stdin.keys() {
        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        match key.unwrap() {
            Key::Char('q') => break,
            Key::Char(c) => writeln!(stdout, "You pressed: {}", c).unwrap(),
            Key::Alt(c) => writeln!(stdout, "You pressed: Alt-{}", c).unwrap(),
            Key::Ctrl(c) => writeln!(stdout, "You pressed: Ctrl-{}", c).unwrap(),
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
