use std::io::{stdin, stdout, Read, Write};
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

const GAME_TITLE: &str = "Conway's game of life.";
const GAME_AUTHOR: &str = "by Sharp Rabbit";
const INSTRUCTIONS: &str = "q to quit, s to start, r to reset";
const SQUARE_CHAR: &str = "#";

struct Game {
    width: u16,
    height: u16,
    state: Vec<Vec<bool>>,
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

    fn paint_state(&self) -> String {
        let mut output_string: String = format!("{}", clear::All,);
        // for i in 1..10 {
        //     output_string.push_str(&format!("{}{}", cursor::Goto(i, 5), SQUARE_CHAR)[..]);
        // }
        for (row_i, row) in self.state.iter().enumerate() {
            for (col_i, value) in row.iter().enumerate() {
                if *value {
                    output_string.push_str(
                        &format!(
                            "{}{}",
                            cursor::Goto((col_i + 1) as u16, (row_i + 1) as u16),
                            SQUARE_CHAR
                        )[..],
                    );
                }

                // output_string
                //     .push_str(&format!("{}{}", cursor::Goto(3 as u16, 3 as u16), SQUARE_CHAR)[..]);
            }
        }

        // println!("aaaa{}", output_string);

        return output_string;
    }
    // fn evolve(&self) {}
}

fn main() {
    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    let (width, height) = termion::terminal_size().ok().unwrap();

    let mut game_life = Vec::new();

    for _ in 1..height {
        let mut temp_vec = Vec::new();
        for _ in 1..width {
            temp_vec.push(true);
        }
        game_life.push(temp_vec);
    }

    let game = Game {
        width: width,
        height: height,
        state: game_life,
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
            b's' => write!(stdout, "{}", game.paint_state()),
            b'a' => write!(stdout, "{}", game.get_main_screen()),
            // Write it to stdout.
            a => write!(stdout, "{}", a),
        }
        .unwrap();

        stdout.flush().unwrap();
    }
}
