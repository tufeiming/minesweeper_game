//! 扫雷游戏库
//!
//! 这是一个完整的扫雷游戏实现，包含：
//! - 核心游戏逻辑
//! - 配置管理
//! - 演示功能
//! - 完整的测试套件

// 核心模块
pub mod core;

// 配置模块
pub mod config;

// CLI 应用模块（从 main.rs 抽离）
pub mod app;

// 演示模块
pub mod demo;

// 测试模块
#[cfg(test)]
mod tests;

// 重新导出主要的公共API，方便外部使用
pub use config::difficulty::Difficulty;
pub use core::board::{Board, Position};
pub use core::game::Game;
