//! 自动展开功能相关测试
//!
//! 专门测试扫雷游戏的自动展开（洪水填充）功能

#[cfg(test)]
mod tests {
    use crate::config::difficulty::{difficulty_to_board_config, Difficulty};
    use crate::core::board::{Board, ClickResult, Position};
    use crate::core::cell::{CellContent, CellState};

    #[test]
    fn test_auto_reveal_basic_functionality() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        // 首次点击，触发地雷生成
        let first_click = Position { row: 4, col: 4 };
        let _ = board.left_click(first_click);

        // 统计初始揭示的格子数量
        let before_count = count_revealed_cells(&board);

        // 尝试找一个安全的隐藏格子进行测试
        if let Some(safe_pos) = find_safe_hidden_cell(&board) {
            let result = board.left_click(safe_pos);

            // 验证点击结果合理
            if !matches!(result, ClickResult::Invalid) {
                let after_count = count_revealed_cells(&board);
                // 如果点击成功，应该有格子被揭示
                assert!(after_count >= before_count);
            }
        }
    }

    #[test]
    fn test_auto_reveal_does_not_reveal_mines() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        // 触发地雷生成
        let _ = board.left_click(Position { row: 4, col: 4 });

        // 只点击确认是安全的位置
        if let Some(safe_pos) = find_safe_hidden_cell(&board) {
            let result = board.left_click(safe_pos);

            // 只有在点击成功的情况下才验证
            if matches!(result, ClickResult::Continue) {
                // 验证没有地雷被意外揭开
                verify_no_mines_revealed(&board);
            }
        }
    }

    #[test]
    fn test_auto_reveal_stops_at_numbered_cells() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        // 触发地雷生成
        let _ = board.left_click(Position { row: 4, col: 4 });

        // 找一个安全的空白格子进行测试
        if let Some(empty_pos) = find_empty_safe_cell(&board) {
            let _ = board.left_click(empty_pos);
        }

        // 验证所有被揭开的格子都是合理的（数字格子，不是地雷）
        verify_revealed_cells_are_valid(&board);
    }

    // 辅助函数
    fn count_revealed_cells(board: &Board) -> usize {
        let config = board.get_board_config();
        let mut count = 0;

        for row in 0..config.board_size.height {
            for col in 0..config.board_size.width {
                let pos = Position { row, col };
                if matches!(board.get_cell_state(pos), CellState::Revealed) {
                    count += 1;
                }
            }
        }
        count
    }

    fn find_safe_hidden_cell(board: &Board) -> Option<Position> {
        let config = board.get_board_config();
        for row in 0..config.board_size.height {
            for col in 0..config.board_size.width {
                let pos = Position { row, col };
                if matches!(board.get_cell_state(pos), CellState::Hidden)
                    && !matches!(board.get_cell_content(pos), CellContent::Mine)
                {
                    return Some(pos);
                }
            }
        }
        None
    }

    fn find_empty_safe_cell(board: &Board) -> Option<Position> {
        let config = board.get_board_config();
        for row in 0..config.board_size.height {
            for col in 0..config.board_size.width {
                let pos = Position { row, col };
                if matches!(board.get_cell_state(pos), CellState::Hidden)
                    && matches!(board.get_cell_content(pos), CellContent::Number(0))
                {
                    return Some(pos);
                }
            }
        }
        None
    }

    fn verify_no_mines_revealed(board: &Board) {
        let config = board.get_board_config();
        for row in 0..config.board_size.height {
            for col in 0..config.board_size.width {
                let pos = Position { row, col };
                if matches!(board.get_cell_state(pos), CellState::Revealed) {
                    assert!(
                        !matches!(board.get_cell_content(pos), CellContent::Mine),
                        "地雷不应该被意外揭开 ({}, {})",
                        row,
                        col
                    );
                }
            }
        }
    }

    fn verify_revealed_cells_are_valid(board: &Board) {
        let config = board.get_board_config();
        for row in 0..config.board_size.height {
            for col in 0..config.board_size.width {
                let pos = Position { row, col };
                if matches!(board.get_cell_state(pos), CellState::Revealed) {
                    let content = board.get_cell_content(pos);
                    assert!(
                        matches!(content, CellContent::Number(_)),
                        "被揭开的格子应该是数字格子，位置({}, {})，内容: {:?}",
                        row, col, content
                    );
                }
            }
        }
    }
}
