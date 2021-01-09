use std::fmt;
use std::str;

use rustyline::Editor;

enum BoardValue {
    None,
    X,
    O,
}

impl fmt::Display for BoardValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BoardValue::None => " ",
                BoardValue::X => "X",
                BoardValue::O => "O",
            }
        )
    }
}

impl Default for BoardValue {
    fn default() -> Self {
        BoardValue::None
    }
}

enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::X => "X",
                Player::O => "O",
            }
        )
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::X
    }
}

#[derive(Default)]
struct Board {
    state: [[BoardValue; 3]; 3],
}

impl fmt::Display for Board {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "┌───┬───┬───┐")?;
        writeln!(f, "│ {} │ {} │ {} │", self.state[0][0], self.state[0][1], self.state[0][2])?;
        writeln!(f, "├───┼───┼───┤")?;
        writeln!(f, "│ {} │ {} │ {} │", self.state[1][0], self.state[1][1], self.state[1][2])?;
        writeln!(f, "├───┼───┼───┤")?;
        writeln!(f, "│ {} │ {} │ {} │", self.state[2][0], self.state[2][1], self.state[2][2])?;
        write!  (f, "└───┴───┴───┘")
    }
}

struct Game {
    board: Board,
    current_player: Player,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::default(),
            current_player: Player::default(),
        }
    }
}

fn main() {
    let game = Game::new();

    println!("Welcome to tic tac toe!");
    loop {
        println!("{}", game.board);
        println!("It's {}'s turn.", game.current_player);
        let inputs = get_row_and_column().unwrap();
        println!("{}, {}", inputs.0, inputs.1);
        break;
    }
}

fn get_row_and_column() -> Result<(usize, usize), ()> {
    let strings = match read_row_and_column_strings() {
        Ok(strings) => strings,
        Err(_) => return Err(()),
    };

    let numbers = match parse_row_and_column_strings(strings) {
        Ok(numbers) => numbers,
        Err(_) => return Err(()),
    };

    Ok((numbers.0 - 1, numbers.1 - 1))
}

fn parse_row_and_column_strings(
    input: (String, String),
) -> Result<(usize, usize), <usize as str::FromStr>::Err> {
    let row: usize = input.0.parse()?;
    let column: usize = input.1.parse()?;

    Ok((row, column))
}

fn read_row_and_column_strings() -> Result<(String, String), rustyline::error::ReadlineError> {
    let mut rl = Editor::<()>::new();

    let valid_inputs: Vec<&str> = vec!["1", "2", "3"];
    let mut row_line: String;
    let mut column_line: String;

    loop {
        row_line = rl.readline("Enter a row (1, 2, 3): ")?;
        if valid_inputs.iter().any(|&s| row_line == s) {
            break;
        };
    }
    loop {
        column_line = rl.readline("Enter a column (1, 2, 3): ")?;
        if valid_inputs.iter().any(|&s| column_line == s) {
            break;
        };
    }

    Ok((row_line, column_line))
}
