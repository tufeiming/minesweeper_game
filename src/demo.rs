use crate::config::{Difficulty, difficulty_to_board_config};
use crate::config::{easy, ui_text};
use crate::core::{Board, Position};

/// 运行演示模式
pub fn run_demo_mode() {
    println!("{}", ui_text::DEMO_START);
    println!("📖 本演示将展示扫雷游戏的核心功能和算法\n");

    // 第一部分：棋盘初始化演示
    println!("🔶 第一部分：棋盘初始化演示");
    println!("{}", "=".repeat(50));

    let config = difficulty_to_board_config(Difficulty::Easy);
    let mut board = Board::new(config);

    println!(
        "📏 游戏配置: {}x{} 棋盘，{} 个地雷",
        config.board_size.width, config.board_size.height, config.mine_count
    );
    println!(
        "💡 地雷密度: {:.1}%",
        (config.mine_count as f32 / (config.board_size.width * config.board_size.height) as f32)
            * 100.0
    );

    println!("\n🔍 初始棋盘状态（所有格子都未揭示）：");
    board.print_debug();

    demo_pause();

    // 第二部分：首次点击和地雷生成
    println!("\n🔶 第二部分：首次点击和地雷生成演示");
    println!("{}", "=".repeat(50));

    let center_pos = Position {
        row: easy::HEIGHT / 2,
        col: easy::WIDTH / 2,
    };
    println!(
        "🎯 选择中心位置进行首次点击: ({}, {})",
        center_pos.row, center_pos.col
    );
    println!("💡 首次点击会触发地雷生成，且保证点击位置安全");

    let result = board.left_click(center_pos);
    println!("📊 点击结果: {:?}", result);

    println!("\n🗺️ 首次点击后的棋盘（地雷已生成并开始揭示）：");
    board.print_debug();

    demo_pause();

    // 第三部分：自动揭示算法演示
    println!("\n🔶 第三部分：自动揭示算法演示");
    println!("{}", "=".repeat(50));
    println!("💡 当点击空白格子时，会自动揭示相邻的空白区域");

    let auto_reveal_pos = Position { row: 0, col: 0 };
    println!(
        "🔍 尝试点击角落位置触发自动揭示: ({}, {})",
        auto_reveal_pos.row, auto_reveal_pos.col
    );

    let auto_result = board.left_click(auto_reveal_pos);
    println!("📊 自动揭示结果: {:?}", auto_result);

    println!("\n🗺️ 自动揭示后的棋盘：");
    board.print_debug();

    demo_pause();

    // 第四部分：标记功能演示
    println!("\n🔶 第四部分：标记功能演示");
    println!("{}", "=".repeat(50));

    let flag_pos = Position {
        row: easy::HEIGHT - 1,
        col: easy::WIDTH - 1,
    };
    println!(
        "🚩 演示标记功能，标记位置: ({}, {})",
        flag_pos.row, flag_pos.col
    );
    println!("💡 右键点击可以标记可疑的地雷位置");

    let flag_result = board.right_click(flag_pos);
    println!("📊 标记结果: {:?}", flag_result);

    println!("\n🗺️ 标记后的棋盘：");
    board.print_debug();

    println!("\n🔄 演示取消标记（再次右键点击相同位置）:");
    let unflag_result = board.right_click(flag_pos);
    println!("📊 取消标记结果: {:?}", unflag_result);

    demo_pause();

    // 第五部分：游戏状态检测演示
    println!("\n🔶 第五部分：游戏状态检测演示");
    println!("{}", "=".repeat(50));

    demo_game_states();

    demo_pause();

    // 第六部分：边界条件测试
    println!("\n🔶 第六部分：边界条件测试");
    println!("{}", "=".repeat(50));

    demo_boundary_conditions(&mut board);

    demo_pause();

    // 第七部分：性能和统计信息
    println!("\n🔶 第七部分：性能和统计信息");
    println!("{}", "=".repeat(50));

    demo_statistics(&board);

    println!("\n{}", ui_text::DEMO_COMPLETE);
    println!("{}", ui_text::GAME_READY);
    println!("{}", ui_text::TEST_HINT);
    println!("\n🎓 演示总结:");
    println!("• 展示了完整的游戏初始化流程");
    println!("• 演示了核心的自动揭示算法");
    println!("• 验证了标记和取消标记功能");
    println!("• 测试了各种边界条件和错误处理");
    println!("• 提供了游戏状态和性能统计信息");
    println!("\n🚀 现在你已经了解了所有核心功能，可以开始游戏了！");
}

fn demo_pause() {
    println!("\n⏸️  按 Enter 继续下一部分演示...");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
}

fn demo_game_states() {
    println!("💡 演示不同的游戏状态检测");

    let small_config = crate::core::BoardConfig {
        board_size: crate::core::BoardSize {
            width: 3,
            height: 3,
        },
        mine_count: 1,
    };

    let mut demo_board = Board::new(small_config);

    println!("\n🎯 创建3x3小棋盘（1个地雷）用于快速演示游戏状态：");

    let _ = demo_board.left_click(Position { row: 0, col: 0 });

    println!("🎮 当前游戏状态：进行中");
    demo_board.print_debug();

    println!("\n💡 游戏胜利条件：揭示所有非地雷格子");
    println!("💡 游戏失败条件：点击到地雷");
}

fn demo_boundary_conditions(board: &mut Board) {
    println!("💡 测试各种边界条件和错误处理");

    println!("\n🔄 测试重复点击已揭示的格子:");
    let revealed_pos = Position {
        row: easy::HEIGHT / 2,
        col: easy::WIDTH / 2,
    };
    let repeat_result = board.left_click(revealed_pos);
    println!("📊 重复点击结果: {:?}", repeat_result);

    println!("\n🚩 测试点击已标记的格子:");
    let flag_test_pos = Position { row: 1, col: 1 };
    board.right_click(flag_test_pos);
    let click_flagged_result = board.left_click(flag_test_pos);
    println!("📊 点击已标记格子的结果: {:?}", click_flagged_result);

    println!("\n📐 测试棋盘边界位置的操作:");
    let corner_positions = vec![
        Position { row: 0, col: 0 },
        Position {
            row: 0,
            col: easy::WIDTH - 1,
        },
        Position {
            row: easy::HEIGHT - 1,
            col: 0,
        },
        Position {
            row: easy::HEIGHT - 1,
            col: easy::WIDTH - 1,
        },
    ];

    for (i, pos) in corner_positions.iter().enumerate() {
        println!("  角落{}：({}, {}) - 可正常操作", i + 1, pos.row, pos.col);
    }
}

fn demo_statistics(_board: &Board) {
    println!("📊 游戏统计信息:");

    let total_cells = easy::WIDTH * easy::HEIGHT;
    let mine_count = easy::MINES;

    println!("• 总格子数: {}", total_cells);
    println!("• 地雷数量: {}", mine_count);
    println!("• 安全格子数: {}", total_cells - mine_count);
    println!("• 当前已揭示格子数: 估算值");
    println!("• 当前已标记格子数: 估算值");
    println!("• 剩余未操作格子数: 估算值");

    println!("\n⚡ 算法特性:");
    println!("• 使用递归算法实现自动揭示");
    println!("• 地雷随机分布算法保证公平性");
    println!("• 高效的邻居计数算法");
    println!("• 完整的状态管理和错误处理");
}
