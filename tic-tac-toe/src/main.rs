use std::fmt;
use std::ops::{Deref, DerefMut};
use std::str;

use rustyline::Editor;

#[derive(PartialEq)]
enum BoardValue {
    Filled(Player),
    Empty,
}

impl fmt::Display for BoardValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Filled(player) => write!(f, "{}", player),
            Self::Empty => write!(f, " "),
        }
    }
}

impl Default for BoardValue {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Copy, PartialEq)]
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
                Self::X => "X",
                Self::O => "O",
            }
        )
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::X
    }
}

type BoardType = [[BoardValue; 3]; 3];

#[derive(Default)]
struct Board(BoardType);

impl Deref for Board {
    type Target = BoardType;

    fn deref(&self) -> &BoardType {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut BoardType {
        &mut self.0
    }
}

impl fmt::Display for Board {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "┌───┬───┬───┐")?;
        writeln!(f, "│ {} │ {} │ {} │", self[0][0], self[0][1], self[0][2])?;
        writeln!(f, "├───┼───┼───┤")?;
        writeln!(f, "│ {} │ {} │ {} │", self[1][0], self[1][1], self[1][2])?;
        writeln!(f, "├───┼───┼───┤")?;
        writeln!(f, "│ {} │ {} │ {} │", self[2][0], self[2][1], self[2][2])?;
        write!  (f, "└───┴───┴───┘")
    }
}

struct Game {
    board: Board,
    current_player: Player,
}

enum GameStatus {
    Continue,
    PlayerWins(Player),
    Draw,
}

enum PlayError {
    InvalidMove,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::default(),
            current_player: Player::default(),
        }
    }

    fn play(&mut self, row: usize, column: usize) -> Result<GameStatus, PlayError> {
        if self.board[row][column] != BoardValue::Empty {
            return Err(PlayError::InvalidMove);
        }

        self.board[row][column] = BoardValue::Filled(self.current_player);

        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        Ok(GameStatus::Continue)
    }
}

fn main() {
    let mut game = Game::new();

    println!("Welcome to tic tac toe!");
    loop {
        println!("{}", game.board);
        println!("It's {}'s turn.", game.current_player);

        let inputs = match get_row_and_column() {
            Ok(inputs) => inputs,
            Err(_) => return,
        };
        let result = game.play(inputs.0, inputs.1);
        if result.is_err() {
            break;
        }
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
