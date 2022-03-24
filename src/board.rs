use crate::game::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Empty,
    Player(Player),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Empty => write!(f, " "),
            State::Player(p) => write!(f, "{}", p),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    board: [[State; 3]; 3],
    turn: Player,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveKind {
    Valid,
    Invalid,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  1 2 3\n")?;
        for (i, row) in self.board.iter().enumerate() {
            write!(f, "{}", i + 1)?;
            for col in row.iter() {
                write!(f, " {}", col)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            board: [[State::Empty; 3]; 3],
            turn: Player::X,
        }
    }

    pub fn turn(&self) -> Player {
        self.turn
    }

    pub fn move_is_valid(&self, x: usize, y: usize) -> bool {
        self.board[x][y] == State::Empty
    }

    pub fn make_move(&mut self, x: usize, y: usize) -> MoveKind {
        if self.move_is_valid(x, y) {
            self.board[x][y] = State::Player(self.turn);
            self.next_turn();
            MoveKind::Valid
        } else {
            MoveKind::Invalid
        }
    }

    pub fn query_board(&self, (x, y): (usize, usize)) -> State {
        self.board[x][y]
    }

    pub fn available_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                if self.board[x][y] == State::Empty {
                    moves.push((x, y));
                }
            }
        }
        moves
    }

    pub fn out_of_moves(&self) -> bool {
        self.available_moves().is_empty()
    }

    pub fn check_winner(&self) -> Option<Player> {
        //Check horizontal rows
        for row in self.board.iter() {
            if row[0] == row[1] && row[1] == row[2] {
                match row[0] {
                    State::Player(p) => return Some(p),
                    _ => (),
                }
            }
        }
        // Check vertical columns
        for x in 0..3 {
            if self.board[0][x] == self.board[1][x] && self.board[1][x] == self.board[2][x] {
                match self.board[0][x] {
                    State::Player(p) => return Some(p),
                    _ => (),
                }
            }
        }
        // Check first diagonal
        if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            match self.board[0][0] {
                State::Player(p) => return Some(p),
                _ => (),
            }
        }
        // Check second diagonal
        if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            match self.board[0][2] {
                State::Player(p) => return Some(p),
                _ => (),
            }
        }
        return None;
    }

    fn next_turn(&mut self) {
        self.turn = self.turn.other();
    }
}
