//! Cell 模块测试
//!
//! 包含Cell结构体、CellContent和CellState的完整测试

#[cfg(test)]
mod tests {
    use crate::core::cell::{Cell, CellContent, CellState};
    use crate::core::board::{Board, Position};
    use crate::config::difficulty::{Difficulty, difficulty_to_board_config};

    // === 基础单元测试 ===

    #[test]
    fn test_cell_creation() {
        let cell = Cell::new();

        // 测试初始状态
        assert!(matches!(cell.content(), CellContent::Number(0)));
        assert_eq!(cell.state(), CellState::Hidden);
        assert!(!cell.is_mine());
    }

    #[test]
    fn test_cell_content_operations() {
        let mut cell = Cell::new();

        // 测试设置数字内容
        cell.set_content(CellContent::Number(3));
        assert!(matches!(cell.content(), CellContent::Number(3)));
        assert!(!cell.is_mine());

        // 测试设置地雷内容
        cell.set_content(CellContent::Mine);
        assert!(matches!(cell.content(), CellContent::Mine));
        assert!(cell.is_mine());

        // 测试设置不同数字
        for i in 0..=8 {
            cell.set_content(CellContent::Number(i));
            assert!(matches!(cell.content(), CellContent::Number(n) if n == i));
            assert!(!cell.is_mine());
        }
    }

    #[test]
    fn test_cell_state_operations() {
        let mut cell = Cell::new();

        // 测试初始状态
        assert_eq!(cell.state(), CellState::Hidden);

        // 测试翻开状态
        cell.set_state(CellState::Revealed);
        assert_eq!(cell.state(), CellState::Revealed);

        // 测试标记状态
        cell.set_state(CellState::Flagged);
        assert_eq!(cell.state(), CellState::Flagged);

        // 测试回到隐藏状态
        cell.set_state(CellState::Hidden);
        assert_eq!(cell.state(), CellState::Hidden);
    }

    #[test]
    fn test_is_mine_functionality() {
        let mut cell = Cell::new();

        // 数字格子不是地雷
        for i in 0..=8 {
            cell.set_content(CellContent::Number(i));
            assert!(!cell.is_mine(), "数字 {i} 不应该被识别为地雷");
        }

        // 地雷格子是地雷
        cell.set_content(CellContent::Mine);
        assert!(cell.is_mine());
    }

    #[test]
    fn test_cell_state_transitions() {
        let mut cell = Cell::new();

        // 测试状态转换序列：隐藏 -> 标记 -> 隐藏 -> 翻开
        assert_eq!(cell.state(), CellState::Hidden);

        cell.set_state(CellState::Flagged);
        assert_eq!(cell.state(), CellState::Flagged);

        cell.set_state(CellState::Hidden);
        assert_eq!(cell.state(), CellState::Hidden);

        cell.set_state(CellState::Revealed);
        assert_eq!(cell.state(), CellState::Revealed);
    }

    #[test]
    fn test_cell_clone_functionality() {
        let mut original = Cell::new();
        original.set_content(CellContent::Number(5));
        original.set_state(CellState::Flagged);

        let cloned = original.clone();

        // 克隆应该有相同的内容和状态
        assert!(matches!(cloned.content(), CellContent::Number(5)));
        assert_eq!(cloned.state(), CellState::Flagged);
        assert!(!cloned.is_mine());

        // 修改原始对象不应该影响克隆
        original.set_content(CellContent::Mine);
        assert!(matches!(cloned.content(), CellContent::Number(5)));
        assert!(original.is_mine());
        assert!(!cloned.is_mine());
    }

    #[test]
    fn test_cell_content_copy_semantics() {
        let content1 = CellContent::Number(3);
        let content2 = content1; // Copy

        // Copy 后两个值应该相等
        assert!(matches!(content1, CellContent::Number(3)));
        assert!(matches!(content2, CellContent::Number(3)));

        let mine_content = CellContent::Mine;
        let mine_copy = mine_content; // Copy

        assert!(matches!(mine_content, CellContent::Mine));
        assert!(matches!(mine_copy, CellContent::Mine));
    }

    #[test]
    fn test_cell_state_equality() {
        let state1 = CellState::Hidden;
        let state2 = CellState::Hidden;
        let state3 = CellState::Revealed;

        assert_eq!(state1, state2);
        assert_ne!(state1, state3);
        assert_ne!(state2, state3);

        // 测试所有状态的相等性
        let hidden1 = CellState::Hidden;
        let hidden2 = CellState::Hidden;
        assert_eq!(hidden1, hidden2);

        let revealed1 = CellState::Revealed;
        let revealed2 = CellState::Revealed;
        assert_eq!(revealed1, revealed2);

        let flagged1 = CellState::Flagged;
        let flagged2 = CellState::Flagged;
        assert_eq!(flagged1, flagged2);
    }

    #[test]
    fn test_comprehensive_cell_workflow() {
        let mut cell = Cell::new();

        // 模拟完整的单元格生命周期

        // 1. 初始状态
        assert_eq!(cell.state(), CellState::Hidden);
        assert!(matches!(cell.content(), CellContent::Number(0)));

        // 2. 设置为地雷
        cell.set_content(CellContent::Mine);
        assert!(cell.is_mine());

        // 3. 玩家标记
        cell.set_state(CellState::Flagged);
        assert_eq!(cell.state(), CellState::Flagged);

        // 4. 玩家取消标记
        cell.set_state(CellState::Hidden);
        assert_eq!(cell.state(), CellState::Hidden);

        // 5. 玩家点击翻开（游戏结束）
        cell.set_state(CellState::Revealed);
        assert_eq!(cell.state(), CellState::Revealed);
        assert!(cell.is_mine());
    }

    #[test]
    fn test_default_trait() {
        let default_cell = Cell::default();
        let new_cell = Cell::new();

        // Default trait 应该产生与new()相同的结果
        assert_eq!(default_cell.state(), new_cell.state());
        assert!(matches!(default_cell.content(), CellContent::Number(0)));
        assert!(matches!(new_cell.content(), CellContent::Number(0)));
        assert!(!default_cell.is_mine());
        assert!(!new_cell.is_mine());
    }

    // === 集成测试 ===

    #[test]
    fn test_cell_integration_with_board() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        let pos = Position { row: 0, col: 0 };

        // 验证初始状态
        assert!(matches!(board.get_cell_state(pos), CellState::Hidden));

        // 进行游戏操作后验证Cell状态变化
        let _ = board.right_click(pos); // 标记
        assert!(matches!(board.get_cell_state(pos), CellState::Flagged));

        let _ = board.right_click(pos); // 取消标记
        assert!(matches!(board.get_cell_state(pos), CellState::Hidden));
    }

    #[test]
    fn test_cell_content_after_mine_generation() {
        let config = difficulty_to_board_config(Difficulty::Easy);
        let mut board = Board::new(config);

        // 触发地雷生成
        let _ = board.left_click(Position { row: 4, col: 4 });

        // 验证所有Cell的内容都是合理的
        let board_config = board.get_board_config();
        for row in 0..board_config.board_size.height {
            for col in 0..board_config.board_size.width {
                let pos = Position { row, col };
                let content = board.get_cell_content(pos);

                match content {
                    CellContent::Mine => {
                        // 地雷格子验证通过
                    }
                    CellContent::Number(n) => {
                        assert!(n <= 8, "数字应该在0-8范围内");
                    }
                }
            }
        }
    }
}
