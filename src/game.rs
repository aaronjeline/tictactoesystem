use crate::board::*;
use crate::human::*;
use std::rc::Rc;

pub type Move = (usize, usize);

pub trait GamePlayer {
    fn make_move(&self, board: &Board) -> Move;
    fn name(&self) -> &'static str;
}

pub struct Game {
    board: Board,
    player_x: Rc<dyn GamePlayer>,
    player_o: Rc<dyn GamePlayer>,
}

#[derive(Debug,Clone)]
pub enum EndGame {
    Winner(Player),
    Tie
}

impl std::fmt::Display for EndGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EndGame::Winner(p) => write!(f, "Player {} wins!", p),
            EndGame::Tie => write!(f, "Tie!"),
        }
    }
}

pub struct OnGoing {}

impl TryFrom<GameState> for EndGame {
    type Error = OnGoing;

    fn try_from(value: GameState) -> Result<Self, Self::Error> {
        match value {
            GameState::InProgress => Err(OnGoing {}),
            GameState::Winner(p) => Ok(EndGame::Winner(p)),
            GameState::Tie => Ok(EndGame::Tie),
        }
    }
}

impl Game {

    pub fn run_games(n : usize, player_x : Rc<dyn GamePlayer>, player_o : Rc<dyn GamePlayer>) 
    {
        let (mut xs, mut os, mut ties) = (0, 0, 0);
        for _ in 0..n {
            let mut game = Game::new(player_x.clone(), player_o.clone());
            match game.play_game() {
                EndGame::Tie => ties += 1,
                EndGame::Winner(Player::X) => xs += 1,
                EndGame::Winner(Player::O) => os += 1,
            }
        }
    
        println!("{}\t{}\tties", player_x.name(), player_o.name());
        println!("{}\t{}\t{}", xs, os, ties);

    }

    pub fn double_human_game() -> Self {
        Game {
            board: Board::new(),
            player_x: Rc::new(HumanPlayer::new()),
            player_o: Rc::new(HumanPlayer::new()),
        }
    }

    pub fn new(player_x: Rc<dyn GamePlayer>, player_o: Rc<dyn GamePlayer>) -> Self {
        Game {
            board: Board::new(),
            player_x: player_x,
            player_o: player_o,
        }
    }

    pub fn run_game(&mut self) {
        println!("Result: {}", self.play_game());
        println!("Final board: {}", self.board);
    }

    fn play_game(&mut self) -> EndGame {
        loop  {
            let p = self.current_player();
            let m = p.make_move(&self.board);
            let r = self.board.make_move(m.0, m.1);
            match r {
                MoveKind::Valid => (),
                MoveKind::Invalid => panic!("Invalid move!"),
            }

            if let Ok(end) = EndGame::try_from(self.board.check_winner()) {
                return end
            } 
        }
    }
    

    pub fn current_player(&self) -> &Rc<dyn GamePlayer> {
        match self.board.turn() {
            Player::X => &self.player_x,
            Player::O => &self.player_o,
        }
    }
}
