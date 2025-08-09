//! 游戏全局常量定义
//!
//! 包含扫雷游戏的所有硬编码数值，便于维护和修改

/// 简单难度配置
pub mod easy {
    /// 棋盘宽度
    pub const WIDTH: usize = 9;
    /// 棋盘高度
    pub const HEIGHT: usize = 9;
    /// 地雷数量
    pub const MINES: usize = 10;
    /// 总格子数
    pub const TOTAL_CELLS: usize = WIDTH * HEIGHT; // 81
}

/// 中等难度配置
pub mod medium {
    /// 棋盘宽度
    pub const WIDTH: usize = 16;
    /// 棋盘高度
    pub const HEIGHT: usize = 16;
    /// 地雷数量
    pub const MINES: usize = 40;
    /// 总格子数
    pub const TOTAL_CELLS: usize = WIDTH * HEIGHT; // 256
}

/// 困难难度配置
pub mod hard {
    /// 棋盘宽度
    pub const WIDTH: usize = 30;
    /// 棋盘高度
    pub const HEIGHT: usize = 16;
    /// 地雷数量
    pub const MINES: usize = 99;
    /// 总格子数
    pub const TOTAL_CELLS: usize = WIDTH * HEIGHT; // 480
}

/// 游戏界面文本常量
pub mod ui_text {
    use super::*;

    /// 游戏标题
    pub const GAME_TITLE: &str = "🎮 扫雷游戏启动！";

    /// 难度选择提示
    pub const DIFFICULTY_PROMPT: &str = "请选择难度：";

    /// 演示模式描述
    pub const DEMO_DESC: &str = "4. 演示模式 (查看核心功能演示)";

    /// 输入提示
    pub const INPUT_PROMPT: &str = "请输入选择 (1-4): ";

    /// 输入错误提示
    pub const INPUT_ERROR: &str = "❌ 输入错误，请重试";

    /// 无效选择提示
    pub const INVALID_CHOICE: &str = "❌ 无效选择，请输入 1-4";

    /// 游戏启动提示
    pub const GAME_START: &str = "\n🚀 启动游戏...\n";

    /// 演示模式启动提示
    pub const DEMO_START: &str = "\n🔍 演示模式启动！";

    /// 演示完成提示
    pub const DEMO_COMPLETE: &str = "\n✅ 核心功能演示完成！";

    /// 游戏就绪提示
    pub const GAME_READY: &str = "🚀 游戏逻辑已就绪，可以接入GUI界面了！";

    /// 测试提示
    pub const TEST_HINT: &str = "📋 运行 'cargo test' 查看详细测试";

    /// 动态生成简单难度描述
    pub fn easy_desc() -> String {
        format!(
            "1. 简单 ({}x{}, {}个地雷)",
            easy::WIDTH,
            easy::HEIGHT,
            easy::MINES
        )
    }

    /// 动态生成中等难度描述
    pub fn medium_desc() -> String {
        format!(
            "2. 中等 ({}x{}, {}个地雷)",
            medium::WIDTH,
            medium::HEIGHT,
            medium::MINES
        )
    }

    /// 动态生成困难难度描述
    pub fn hard_desc() -> String {
        format!(
            "3. 困难 ({}x{}, {}个地雷)",
            hard::WIDTH,
            hard::HEIGHT,
            hard::MINES
        )
    }
}

/// 测试用常量
pub mod test {
    /// 测试用小棋盘宽度
    pub const SMALL_WIDTH: usize = 4;
    /// 测试用小棋盘高度
    pub const SMALL_HEIGHT: usize = 4;
    /// 测试用小棋盘地雷数
    pub const SMALL_MINES: usize = 3;

    /// 测试用中型棋盘宽度
    pub const MEDIUM_TEST_WIDTH: usize = 5;
    /// 测试用中型棋盘高度
    pub const MEDIUM_TEST_HEIGHT: usize = 5;
    /// 测试用中型棋盘地雷数
    pub const MEDIUM_TEST_MINES: usize = 2;

    /// 无效测试位置 - 严重越界
    pub const INVALID_LARGE_POS: usize = 999;
}

/// 游戏机制常量
pub mod game {
    /// 最大相邻地雷数（一个格子周围最多8个地雷）
    pub const MAX_ADJACENT_MINES: u8 = 8;

    /// 空白格子的数字值
    pub const EMPTY_CELL_VALUE: u8 = 0;
}
