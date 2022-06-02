use crate::game::*;
use crate::board::*;
use rand::prelude::*;

pub struct Random {

}

impl Random {
    pub fn new() -> Self {
        Random {}
    }
}

impl GamePlayer for Random {
    fn make_move(&self, board: &Board) -> Move {
        let mut rng = rand::thread_rng();
        let mut avail = board.available_moves();
        avail.shuffle(&mut rng);
        avail[0]
    }

    fn name(&self) -> &'static str {
        "Random"
    }
}

