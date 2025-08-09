//! 游戏流程测试
//!
//! 测试完整的游戏流程和状态管理

#[cfg(test)]
mod tests {
    use crate::core::board::{Board, ClickResult, Position};
    use crate::core::cell::{CellContent, CellState};
    use crate::config::constants::easy;
    use crate::config::difficulty::{Difficulty, difficulty_to_board_config};

    #[test]
    fn test_complete_game_flow() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        // 验证初始状态
        assert!(!board.are_mines_placed());
        assert_eq!(board.get_revealed_count(), 0);

        // 首次点击
        let first_pos = Position { row: 4, col: 4 };
        let result = board.left_click(first_pos);

        // 验证首次点击结果
        assert!(!matches!(result, ClickResult::Invalid));
        assert!(!matches!(result, ClickResult::GameOver));
        assert!(board.are_mines_placed());

        // 测试标记功能
        let flag_pos = Position { row: 0, col: 0 };
        if matches!(board.get_cell_state(flag_pos), CellState::Hidden) {
            let result = board.right_click(flag_pos);
            assert!(matches!(result, ClickResult::Continue));
            assert!(matches!(board.get_cell_state(flag_pos), CellState::Flagged));

            // 测试点击标记的格子
            let result = board.left_click(flag_pos);
            assert!(matches!(result, ClickResult::Invalid));

            // 取消标记
            let result = board.right_click(flag_pos);
            assert!(matches!(result, ClickResult::Continue));
            assert!(matches!(board.get_cell_state(flag_pos), CellState::Hidden));
        }
    }

    #[test]
    fn test_victory_condition() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        // 触发地雷生成
        let _ = board.left_click(Position { row: 4, col: 4 });

        // 验证初始状态不是胜利
        assert!(!board.check_victory());

        // 计算需要翻开的格子数量
        let total_cells = board.get_board_config().board_size.cell_count();
        let mine_count = board.get_board_config().mine_count;
        let need_to_reveal = total_cells - mine_count;

        // 验证胜利条件逻辑
        let revealed_count = board.get_revealed_count();
        assert_eq!(revealed_count == need_to_reveal, board.check_victory());
    }

    #[test]
    fn test_mine_generation_properties() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        // 触发地雷生成
        let first_pos = Position { row: 4, col: 4 };
        let _ = board.left_click(first_pos);

        // 验证地雷数量正确
        assert_eq!(count_mines(&board), easy::MINES);

        // 验证首次点击位置是安全的
        assert!(!matches!(
            board.get_cell_content(first_pos),
            CellContent::Mine
        ));
    }

    #[test]
    fn test_edge_cases() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        // 测试重复点击
        let pos = Position { row: 1, col: 1 };
        let result1 = board.left_click(pos);
        assert!(!matches!(result1, ClickResult::Invalid));

        let result2 = board.left_click(pos);
        assert!(matches!(result2, ClickResult::Invalid));

        // 测试标记已揭示的格子
        let result = board.right_click(pos);
        assert!(matches!(result, ClickResult::Invalid));
    }

    // 辅助函数
    fn count_mines(board: &Board) -> usize {
        let config = board.get_board_config();
        let mut count = 0;

        for row in 0..config.board_size.height {
            for col in 0..config.board_size.width {
                let pos = Position { row, col };
                if matches!(board.get_cell_content(pos), CellContent::Mine) {
                    count += 1;
                }
            }
        }
        count
    }
}
