#[derive(Debug)]
enum BoardValue {
    None,
    X,
    O,
}

impl Default for BoardValue {
    fn default() -> Self {
        BoardValue::None
    }
}

#[derive(Debug, Default)]
struct Board([[BoardValue; 3]; 3]);

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
    println!("{:?}", game.board);
}
