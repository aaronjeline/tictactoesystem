use crate::board::*;
use crate::game::*;
use std::fmt;

pub struct HumanPlayer {}

impl HumanPlayer {
    pub fn new() -> Self {
        HumanPlayer {}
    }
}

impl GamePlayer for HumanPlayer {
    fn make_move(&self, board: &Board) -> Move {
        print!("{}\nEnter move:\n", board);
        get_stdin()
    }
}
fn get_stdin() -> (usize, usize) {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    let mut split = input.split(",");
    let x = split.next().unwrap().parse::<usize>().unwrap();
    let y = split.next().unwrap().parse::<usize>().unwrap();
    (x - 1, y - 1)
}
