use crate::board::*;
use crate::human::*;

pub type Move = (usize, usize);

pub trait GamePlayer {
    fn make_move(&self, board: &Board) -> Move;
}

pub struct Game {
    board: Board,
    player_x: Box<dyn GamePlayer>,
    player_o: Box<dyn GamePlayer>,
}

impl Game {
    pub fn double_human_game() -> Self {
        Game {
            board: Board::new(),
            player_x: Box::new(HumanPlayer::new()),
            player_o: Box::new(HumanPlayer::new()),
        }
    }

    pub fn new(player_x: Box<dyn GamePlayer>, player_o: Box<dyn GamePlayer>) -> Self {
        Game {
            board: Board::new(),
            player_x: player_x,
            player_o: player_o,
        }
    }

    pub fn run_game(&mut self) {
        match self.play_game() {
            Some(p) => println!("Winner: {}", p),
            None => println!("Tie!"),
        }
        println!("Final board: {}", self.board);
    }

    fn play_game(&mut self) -> Option<Player> {
        loop {
            let p = self.current_player();
            let m = p.make_move(&self.board);
            let r = self.board.make_move(m.0, m.1);
            match r {
                MoveKind::Valid => (),
                MoveKind::Invalid => return Some(self.board.turn().other()),
            }
            match self.board.check_winner() {
                Some(p) => return Some(p),
                None => (),
            }
        }
    }
    

    pub fn current_player(&self) -> &Box<dyn GamePlayer> {
        match self.board.turn() {
            Player::X => &self.player_x,
            Player::O => &self.player_o,
        }
    }
}
