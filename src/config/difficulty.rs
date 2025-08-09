use crate::config::{easy, hard, medium};
use crate::core::{BoardConfig, BoardSize};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub fn difficulty_to_board_config(difficulty: Difficulty) -> BoardConfig {
    match difficulty {
        Difficulty::Easy => BoardConfig {
            board_size: BoardSize {
                width: easy::WIDTH,
                height: easy::HEIGHT,
            },
            mine_count: easy::MINES,
        },
        Difficulty::Medium => BoardConfig {
            board_size: BoardSize {
                width: medium::WIDTH,
                height: medium::HEIGHT,
            },
            mine_count: medium::MINES,
        },
        Difficulty::Hard => BoardConfig {
            board_size: BoardSize {
                width: hard::WIDTH,
                height: hard::HEIGHT,
            },
            mine_count: hard::MINES,
        },
    }
}

pub struct Minesweeper {
    // board: Board,
    // game_state: GameState,
    // difficulty: Difficulty,
}

impl Minesweeper {}
