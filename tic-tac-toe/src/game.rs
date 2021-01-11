use std::fmt;
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, PartialEq)]
pub enum BoardValue {
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

impl BoardValue {
    fn player(self) -> Player {
        if let Self::Filled(player) = self {
            player
        } else {
            panic!("not filled")
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
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
pub struct Board(BoardType);

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

impl Board {
    fn get_winnable_slices(&self) -> Vec<[&BoardValue; 3]> {
        vec![
            // Horizontal
            [&self[0][0], &self[0][1], &self[0][2]],
            [&self[1][0], &self[1][1], &self[1][2]],
            [&self[2][0], &self[2][1], &self[2][2]],
            // Vertical
            [&self[0][0], &self[1][0], &self[2][0]],
            [&self[0][1], &self[1][1], &self[2][1]],
            [&self[0][2], &self[1][2], &self[2][2]],
            // Diagonal
            [&self[0][0], &self[1][1], &self[2][2]],
            [&self[0][2], &self[1][1], &self[2][0]],
        ]
    }

    fn get_all_cells(&self) -> Vec<&BoardValue> {
        self.iter().flatten().collect::<Vec<&BoardValue>>()
    }
}

pub struct Game {
    pub board: Board,
    pub current_player: Player,
}

#[derive(PartialEq)]
pub enum GameStatus {
    Continue,
    PlayerWins(Player),
    Draw,
}

pub enum PlayError {
    InvalidMove,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::default(),
            current_player: Player::default(),
        }
    }

    pub fn play(&mut self, row: usize, column: usize) -> Result<GameStatus, PlayError> {
        if self.board[row][column] != BoardValue::Empty {
            return Err(PlayError::InvalidMove);
        }

        self.board[row][column] = BoardValue::Filled(self.current_player);

        let game_status = self.get_game_status();

        if game_status == GameStatus::Continue {
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
        }

        Ok(game_status)
    }

    fn get_game_status(&self) -> GameStatus {
        for slice in self.board.get_winnable_slices() {
            if slice[0] != &BoardValue::Empty && slice[0] == slice[1] && slice[1] == slice[2] {
                return GameStatus::PlayerWins(slice[0].player());
            }
        }

        if self
            .board
            .get_all_cells()
            .into_iter()
            .all(|cell| *cell != BoardValue::Empty)
        {
            return GameStatus::Draw;
        }

        GameStatus::Continue
    }
}
