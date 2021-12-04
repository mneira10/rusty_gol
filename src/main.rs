use std::io::{stdin, stdout, Read, Write};
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

const GAME_TITLE: &str = "Conway's game of life.";
const GAME_AUTHOR: &str = "by Sharp Rabbit";
const INSTRUCTIONS: &str = "q to quit, s to start, r to reset";

struct Game {
    width: u16,
    height: u16,
}

impl Game {
    fn get_main_screen(&self) -> String {
        return format!(
            "{}{}{}{}{}{}{}{}{}{}",
            clear::All,
            style::Bold,
            cursor::Goto(
                (self.width - GAME_TITLE.chars().count() as u16) / 2,
                self.height / 2 - 2
            ),
            GAME_TITLE,
            style::Reset,
            cursor::Goto(
                (self.width - GAME_AUTHOR.chars().count() as u16) / 2,
                self.height / 2
            ),
            GAME_AUTHOR,
            cursor::Goto(
                (self.width - INSTRUCTIONS.chars().count() as u16) / 2,
                self.height / 2 + 2
            ),
            INSTRUCTIONS,
            cursor::Hide
        );
    }
}

fn main() {
    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    let (width, height) = termion::terminal_size().ok().unwrap();

    let game = Game {
        width: width,
        height: height,
    };

    write!(stdout, "{}", game.get_main_screen());
    stdout.flush().unwrap();

    let mut bytes = stdin.bytes();
    loop {
        let b = bytes.next().unwrap().unwrap();

        match b {
            // Quit
            b'q' => return,
            // Clear the screen
            b'c' => write!(stdout, "{}", termion::clear::All),
            // Set red color
            b'r' => write!(stdout, "{}", color::Fg(color::Rgb(5, 0, 0))),
            // Write it to stdout.
            a => write!(stdout, "{}", a),
        }
        .unwrap();

        stdout.flush().unwrap();
    }
}
