//! 配置模块
//!
//! 包含游戏的配置相关组件：
//! - constants: 全局常量定义
//! - difficulty: 难度配置和转换

pub mod constants;
pub mod difficulty;

// 重新导出常用的配置类型
pub use constants::*;
pub use difficulty::{Difficulty, difficulty_to_board_config};
