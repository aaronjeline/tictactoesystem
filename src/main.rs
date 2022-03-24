mod board;
mod game;
mod human;
mod minimax;

fn main() {
    let me = Box::new(human::HumanPlayer::new());
    let computer = Box::new(minimax::Minimax::new(board::Player::O));
    let mut game = game::Game::new(me, computer);
    game.run_game();
}
