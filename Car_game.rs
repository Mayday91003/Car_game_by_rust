use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{Write, stdout, stdin};

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdout = &mut stdout as &mut dyn Write;
    let stdin = stdin.keys();
    let mut screen = AlternateScreen::from(stdout);

    let mut x = 5;
    let mut y = 5;

    write!(screen, "{}", termion::cursor::Goto(x, y)).unwrap();
    write!(screen, "⭐️").unwrap();
    screen.flush().unwrap();

    for c in stdin {
        write!(screen, "{}", termion::cursor::Goto(x, y)).unwrap();
        write!(screen, " ").unwrap();

        match c.unwrap() {
            termion::event::Key::Left => x -= 1,
            termion::event::Key::Right => x += 1,
            termion::event::Key::Up => y -= 1,
            termion::event::Key::Down => y += 1,
            _ => break,
        }

        write!(screen, "{}", termion::cursor::Goto(x, y)).unwrap();
        write!(screen, "⭐️").unwrap();
        screen.flush().unwrap();
    }
}