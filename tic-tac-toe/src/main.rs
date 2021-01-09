use std::fmt;

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
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board::default(),
        }
    }
}

fn main() {
    let game = Game::new();
    println!("Welcome to tic tac toe!");
    loop {
        println!("{}", game.board);
        break;
    }
}
