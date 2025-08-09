//! 核心游戏逻辑模块
//!
//! 包含扫雷游戏的核心组件：
//! - Board: 游戏棋盘逻辑
//! - Cell: 单元格状态管理
//! - Game: 游戏流程控制

pub mod board;
pub mod cell;
pub mod game;

// 重新导出核心类型，方便外部使用
pub use board::{Board, BoardConfig, BoardSize, ClickResult, Position};
pub use cell::{Cell, CellContent, CellState};
pub use game::Game;
