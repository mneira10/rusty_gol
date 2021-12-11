use std::io::{stdin, stdout, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use termion::raw::IntoRawMode;
use termion::{clear, color, cursor, style};

use rand::Rng;

const GAME_TITLE: &str = "Conway's game of life.";
const GAME_AUTHOR: &str = "by Sharp Rabbit";
const INSTRUCTIONS: &str = "q to quit, s to start, r to reset";
const SQUARE_CHAR: &str = "#";
const TIME_BETWEEN_FRAMES_IN_MILLISECONDS: u64 = 200;

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
            }
        }

        return output_string;
    }

    fn get_neighbours(&self, row_i: usize, col_i: usize) -> usize {
        let steps: Vec<i32> = vec![-1, 0, 1];

        let mut neighbours = 0;

        for row_step in steps.iter() {
            for col_step in steps.iter() {
                let row_window = ((row_i as i32 + row_step) % (self.height as i32)
                    + self.height as i32)
                    % self.height as i32;
                let col_window = ((col_i as i32 + col_step) % (self.width as i32)
                    + self.width as i32)
                    % self.width as i32;

                if self.state[row_window as usize][col_window as usize]
                    && *row_step != 0
                    && *col_step != 0
                {
                    neighbours += 1;
                }
            }
        }

        return neighbours;
    }

    fn evolve(&mut self) {
        let mut new_state: Vec<Vec<bool>> = Vec::new();

        for (row_i, row) in self.state.iter().enumerate() {
            let mut new_row: Vec<bool> = Vec::new();

            for (col_i, current_cell) in row.iter().enumerate() {
                let neighbours = self.get_neighbours(row_i, col_i);

                let mut new_cell = false;
                if *current_cell && (neighbours == 2 || neighbours == 3) {
                    new_cell = true;
                }

                if !current_cell && neighbours == 3 {
                    new_cell = true;
                }

                new_row.push(new_cell);
            }

            new_state.push(new_row);
        }

        self.state = new_state;
    }
}

fn get_time_now() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let in_ms =
        since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    return in_ms;
}

fn main() {
    // Initialize 'em all.
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut start_time = get_time_now();
    let mut playing = false;

    let mut rng = rand::thread_rng();

    let (width, height) = termion::terminal_size().ok().unwrap();

    let mut game_life = Vec::new();

    for _ in 0..height {
        let mut temp_vec = Vec::new();
        for _ in 0..width {
            let rand_number = rng.gen::<f64>();

            if rand_number < 0.44 {
                temp_vec.push(true);
            } else {
                temp_vec.push(false);
            }
        }
        game_life.push(temp_vec);
    }

    let mut game = Game {
        width: width,
        height: height,
        state: game_life,
    };

    write!(stdout, "{}", game.get_main_screen());
    stdout.flush().unwrap();

    let mut bytes = stdin.bytes();
    loop {
        let b = bytes.next().unwrap().unwrap();
        println!("loooooooped");
        match b {
            // Quit
            b'q' => return,
            // Clear the screen
            b'c' => write!(stdout, "{}", termion::clear::All),
            b's' => {
                playing = true;
                start_time = get_time_now();
                write!(stdout, "{}", game.paint_state())
            }
            b'a' => write!(stdout, "{}", game.get_main_screen()),
            // Write it to stdout.
            a => write!(stdout, "{}", a),
        }
        .unwrap();

        // TODO: need to read from stdin in another thread to unblock this thread
        // https://stackoverflow.com/a/55201400/10296312
        if playing {
            loop {
                if get_time_now() - start_time > TIME_BETWEEN_FRAMES_IN_MILLISECONDS {
                    game.evolve();
                    write!(stdout, "{}", game.paint_state()).unwrap();
                    start_time = get_time_now();

                    stdout.flush().unwrap();
                }
            }
        }

        stdout.flush().unwrap();
    }
}
