//! Board模块的测试用例
//!
//! 包含Board相关的所有单元测试，包括：
//! - 基础功能测试
//! - 游戏逻辑测试
//! - 完整流程测试
//! - 边界条件测试

#[cfg(test)]
mod tests {
    use crate::core::board::{Board, ClickResult, Position};
    use crate::core::cell::{CellState};
    use crate::config::difficulty::{Difficulty, difficulty_to_board_config};

    #[test]
    fn test_board_creation_easy() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let board = Board::new(config);

        // 验证棋盘尺寸
        assert_eq!(board.get_board_config().board_size.width, 9);
        assert_eq!(board.get_board_config().board_size.height, 9);
        assert_eq!(board.get_board_config().mine_count, 10);

        // 验证初始状态
        assert!(!board.are_mines_placed());
        assert_eq!(board.get_revealed_count(), 0);

        // 验证所有格子都是初始状态
        verify_initial_board_state(&board);
    }

    #[test]
    fn test_first_click_generates_mines() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        // 初始状态没有地雷
        assert!(!board.are_mines_placed());

        // 首次点击
        let center_pos = Position { row: 4, col: 4 };
        let result = board.left_click(center_pos);

        // 首次点击后应该生成地雷
        assert!(board.are_mines_placed());

        // 首次点击不应该踩到地雷
        assert!(!matches!(result, ClickResult::GameOver));

        // 点击位置应该被翻开
        assert!(matches!(
            board.get_cell_state(center_pos),
            CellState::Revealed
        ));
    }

    #[test]
    fn test_invalid_position_click() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        // 测试各种无效位置
        let invalid_positions = vec![
            Position { row: 9, col: 0 },   // 行越界
            Position { row: 0, col: 9 },   // 列越界
            Position { row: 10, col: 10 }, // 双重越界
        ];

        for pos in invalid_positions {
            let result = board.left_click(pos);
            assert!(matches!(result, ClickResult::Invalid));
            let result = board.right_click(pos);
            assert!(matches!(result, ClickResult::Invalid));
        }
    }

    #[test]
    fn test_basic_click_operations() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        let pos = Position { row: 0, col: 0 };

        // 第一次点击应该成功
        let result1 = board.left_click(pos);
        assert!(!matches!(result1, ClickResult::Invalid));

        // 第二次点击应该无效（格子已翻开）
        let result2 = board.left_click(pos);
        assert!(matches!(result2, ClickResult::Invalid));
    }

    #[test]
    fn test_right_click_functionality() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        let pos = Position { row: 0, col: 0 };

        // 初始状态是隐藏
        assert!(matches!(board.get_cell_state(pos), CellState::Hidden));

        // 右键标记
        let result = board.right_click(pos);
        assert!(matches!(result, ClickResult::Continue));
        assert!(matches!(board.get_cell_state(pos), CellState::Flagged));

        // 再次右键取消标记
        let result = board.right_click(pos);
        assert!(matches!(result, ClickResult::Continue));
        assert!(matches!(board.get_cell_state(pos), CellState::Hidden));
    }

    #[test]
    fn test_board_size_calculations() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let board = Board::new(config);

        let size = &board.get_board_config().board_size;
        assert_eq!(size.cell_count(), 81); // 9 * 9

        let config = difficulty_to_board_config(Difficulty::Medium);
        let board = Board::new(config);

        let size = &board.get_board_config().board_size;
        assert_eq!(size.cell_count(), 256); // 16 * 16
    }

    // 辅助函数
    fn verify_initial_board_state(board: &Board) {
        let size = &board.get_board_config().board_size;
        for row in 0..size.height {
            for col in 0..size.width {
                let pos = Position { row, col };
                assert!(
                    matches!(board.get_cell_state(pos), CellState::Hidden),
                    "初始状态下所有格子都应该是隐藏状态"
                );
            }
        }
    }
}
