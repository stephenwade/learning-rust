mod game;

use game::*;

use std::str;
use rustyline::Editor;

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
        match game.play(inputs.0, inputs.1) {
            Ok(GameStatus::Continue) => continue,
            Ok(GameStatus::Draw) => {
                println!("{}", game.board);
                println!("It's a draw.");
                return;
            }
            Ok(GameStatus::PlayerWins(player)) => {
                println!("{}", game.board);
                println!("{} wins!", player);
                return;
            }
            Err(PlayError::InvalidMove) => {
                println!("Invalid move. Please try again.");
            }
        };
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
