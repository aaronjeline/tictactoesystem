use crate::board::*;
use crate::game::*;

pub struct Minimax {
    me: Player,
}

impl Minimax {
    pub fn new(me: Player) -> Self {
        Minimax { me }
    }

    // TODO change to floats
    fn minimax(&self, board: &Board, depth: usize, kind: MinMax) -> i32 {
        if let Some(score) = self.is_end_of_game(board, depth) {
            return score;
        }
        match kind {
            MinMax::Min => {
                let mut worst = std::i32::MAX;
                for m in board.available_moves() {
                    let b = make_move(board, m);
                    let v = self.minimax(&b, depth + 1, MinMax::Max);
                    worst = i32::min(worst, v);
                }
                return worst;
            }
            MinMax::Max => {
                let mut best = std::i32::MIN;
                for mv in board.available_moves() {
                    let b = make_move(board, mv);
                    let v = self.minimax(&b, depth + 1, MinMax::Min);
                    best = i32::max(v, best);
                }
                return best;
            }
        }
    }

    fn is_end_of_game(&self, board: &Board, depth: usize) -> Option<i32> {
        let depth = depth as i32;
        match board.check_winner() {
            Some(winner) => Some(if winner == self.me {
                10 - depth
            } else {
                -10 - depth
            }),
            None => {
                if board.out_of_moves() {
                    Some(0)
                } else {
                    None
                }
            }
        }
    }
}

fn make_move(board: &Board, m: Move) -> Board {
    let mut new_board = board.clone();
    new_board.make_move(m.0, m.1);
    new_board
}

#[derive(Debug, Clone, Copy)]
enum MinMax {
    Min,
    Max,
}

impl MinMax {
    fn other(self) -> Self {
        match self {
            MinMax::Min => MinMax::Max,
            MinMax::Max => MinMax::Min,
        }
    }
}

impl GamePlayer for Minimax {
    fn make_move(&self, board: &Board) -> Move {
        for (x, y) in board.available_moves() {
            println!(
                "Moving to ({},{}) has score {}",
                x,
                y,
                self.minimax(&make_move(board, (x, y)), 0, MinMax::Max)
            );
        }

        board
            .available_moves()
            .into_iter()
            .map(|x| x)
            .max_by_key(|m| self.minimax(&make_move(board, *m), 1, MinMax::Max))
            .unwrap()
    }
}
