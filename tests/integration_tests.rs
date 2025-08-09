use minesweeper_game::config::{Difficulty, difficulty_to_board_config};
use minesweeper_game::core::CellState;
use minesweeper_game::core::{Board, ClickResult, Position};

// 基础功能测试
#[test]
fn test_board_creation() {
    let config = difficulty_to_board_config(Difficulty::Easy);
    let board = Board::new(config);

    // 验证棋盘创建成功
    let board_config = board.get_board_config();
    assert_eq!(board_config.board_size.width, 9);
    assert_eq!(board_config.board_size.height, 9);
    assert_eq!(board_config.mine_count, 10);

    // 验证初始状态
    assert!(!board.are_mines_placed());
}

#[test]
fn test_first_click_safety() {
    let config = difficulty_to_board_config(Difficulty::Easy);
    let mut board = Board::new(config);

    // 首次点击应该安全
    let center_pos = Position { row: 4, col: 4 };
    let result = board.left_click(center_pos);

    // 验证结果
    assert!(matches!(result, ClickResult::Continue));
    assert!(board.are_mines_placed());

    // 验证首次点击位置是安全的（不是地雷且已揭示）
    let cell_state = board.get_cell_state(center_pos);
    assert!(matches!(cell_state, CellState::Revealed));
}

// 标记功能完整测试
#[test]
fn test_flag_functionality() {
    let config = difficulty_to_board_config(Difficulty::Easy);
    let mut board = Board::new(config);

    let test_pos = Position { row: 0, col: 0 };

    // 测试标记功能
    let result = board.right_click(test_pos);
    assert!(matches!(result, ClickResult::Continue));

    let cell_state = board.get_cell_state(test_pos);
    assert!(matches!(cell_state, CellState::Flagged));

    // 测试取消标记
    let result = board.right_click(test_pos);
    assert!(matches!(result, ClickResult::Continue));

    let cell_state = board.get_cell_state(test_pos);
    assert!(matches!(cell_state, CellState::Hidden));
}

// 自动揭示机制测试
#[test]
fn test_auto_reveal_mechanism() {
    let config = difficulty_to_board_config(Difficulty::Easy);
    let mut board = Board::new(config);

    // 先进行首次点击以生成地雷
    let first_pos = Position { row: 4, col: 4 };
    board.left_click(first_pos);

    // 统计揭示的格子数量
    let mut revealed_count = 0;
    for row in 0..9 {
        for col in 0..9 {
            let p = Position { row, col };
            if matches!(board.get_cell_state(p), CellState::Revealed) {
                revealed_count += 1;
            }
        }
    }
    // 应该有一些格子被自动揭示（至少首次点击的位置）
    assert!(revealed_count > 0);
}

// 游戏流程集成测试
#[test]
fn test_complete_game_flow() {
    let config = difficulty_to_board_config(Difficulty::Easy);
    let mut board = Board::new(config);

    // 首次点击
    let first_click = Position { row: 1, col: 1 };
    let result = board.left_click(first_click);
    assert!(matches!(
        result,
        ClickResult::Continue | ClickResult::Victory
    ));

    // 如果第一次点击就获胜了，游戏结束
    if matches!(result, ClickResult::Victory) {
        return;
    }

    // 测试标记功能 - 找一个隐藏的格子
    let mut flag_pos = Position { row: 7, col: 7 };
    if matches!(board.get_cell_state(flag_pos), CellState::Revealed) {
        // 扫描寻找隐藏格子
        'outer: for r in 0..config.board_size.height {
            for c in 0..config.board_size.width {
                let p = Position { row: r, col: c };
                if matches!(board.get_cell_state(p), CellState::Hidden) {
                    flag_pos = p;
                    break 'outer;
                }
            }
        }
    }

    let result = board.right_click(flag_pos);
    assert!(matches!(result, ClickResult::Continue));

    // 尝试点击被标记的格子（应该无效）
    let result = board.left_click(flag_pos);
    assert!(matches!(result, ClickResult::Invalid));
}

// 边界条件测试
#[test]
fn test_boundary_conditions() {
    let config = difficulty_to_board_config(Difficulty::Easy);

    let corners = vec![
        Position { row: 0, col: 0 },
        Position { row: 0, col: 8 },
        Position { row: 8, col: 0 },
        Position { row: 8, col: 8 },
    ];

    for corner in corners {
        let mut board = Board::new(config);
        let result = board.left_click(corner);
        assert!(!matches!(result, ClickResult::Invalid));
        assert!(!matches!(result, ClickResult::GameOver));
    }
}

#[test]
fn test_invalid_positions() {
    let config = difficulty_to_board_config(Difficulty::Easy);
    let mut board = Board::new(config);

    let invalid_positions = vec![
        Position { row: 9, col: 0 },
        Position { row: 0, col: 9 },
        Position { row: 10, col: 10 },
    ];

    for pos in invalid_positions {
        let left_result = board.left_click(pos);
        let right_result = board.right_click(pos);

        assert!(matches!(left_result, ClickResult::Invalid));
        assert!(matches!(right_result, ClickResult::Invalid));
    }
}

#[test]
fn test_different_difficulties() {
    let test_cases = vec![
        (Difficulty::Easy, 9, 9, 10),
        (Difficulty::Medium, 16, 16, 40),
        (Difficulty::Hard, 30, 16, 99),
    ];

    for (difficulty, expected_width, expected_height, expected_mines) in test_cases {
        let config = difficulty_to_board_config(difficulty);
        let mut board = Board::new(config);

        // 验证配置
        let board_config = board.get_board_config();
        assert_eq!(board_config.board_size.width, expected_width);
        assert_eq!(board_config.board_size.height, expected_height);
        assert_eq!(board_config.mine_count, expected_mines);

        // 触发地雷生成
        let center_pos = Position {
            row: expected_height / 2,
            col: expected_width / 2,
        };
        let _ = board.left_click(center_pos);

        // 验证地雷已生成
        assert!(board.are_mines_placed());
    }
}
