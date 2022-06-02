mod board;
mod game;
mod human;
mod minimax;
mod random;
use std::rc::Rc;
fn main() {
    let me = Rc::new(random::Random::new());
    let computer = Rc::new(minimax::Minimax::new(board::Player::O));
    game::Game::run_games(100, me, computer);
}
