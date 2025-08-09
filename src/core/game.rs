use crate::config::{Difficulty, difficulty_to_board_config};
use crate::core::{Board, ClickResult, Position};
use std::io::{self, Write};

pub struct Game {
    board: Board,
    game_over: bool,
    victory: bool,
}

impl Game {
    pub fn new(difficulty: Difficulty) -> Self {
        let config = difficulty_to_board_config(difficulty);
        let board = Board::new(config);
        Game {
            board,
            game_over: false,
            victory: false,
        }
    }

    pub fn run(&mut self) {
        self.print_welcome();
        self.print_help();

        while !self.game_over {
            self.print_board();
            self.print_status();

            match self.get_user_input() {
                Some(command) => {
                    if !self.process_command(&command) {
                        break; // 用户选择退出
                    }
                }
                None => {
                    println!("❌ 无效输入，请重试");
                    continue;
                }
            }
        }

        self.print_game_over();
    }

    fn print_welcome(&self) {
        println!("🎮 欢迎来到命令行扫雷游戏！");
        let config = self.board.get_board_config();
        println!(
            "📏 游戏配置: {}x{}, {} 个地雷",
            config.board_size.width, config.board_size.height, config.mine_count
        );
        println!();
    }

    fn print_help(&self) {
        println!("📋 游戏指令：");
        println!("  click <行> <列>   - 左键点击格子 (例: click 3 5)");
        println!("  flag <行> <列>    - 右键标记/取消标记 (例: flag 2 4)");
        println!("  help              - 显示帮助信息");
        println!("  quit              - 退出游戏");
        println!("  💡 坐标从0开始计算");
        println!();
    }

    fn print_board(&self) {
        println!("🗺️ 当前棋盘状态：");
        // 复用 Board 的统一人类友好打印
        self.board.print_debug();
        println!();
    }

    fn print_status(&self) {
        if self.victory {
            println!("🎉 恭喜你！游戏胜利！");
        } else if self.game_over {
            println!("💥 游戏结束！你踩到了地雷！");
        } else {
            println!("🎯 继续游戏...");
        }
    }

    fn get_user_input(&self) -> Option<String> {
        print!("请输入指令: ");
        io::stdout().flush().ok()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok()?;

        Some(input.trim().to_lowercase())
    }

    fn process_command(&mut self, command: &str) -> bool {
        let parts: Vec<&str> = command.split_whitespace().collect();

        if parts.is_empty() {
            return true;
        }

        match parts[0] {
            "help" => {
                self.print_help();
            }
            "quit" | "exit" => {
                println!("👋 再见！");
                return false;
            }
            "click" => {
                if parts.len() != 3 {
                    println!("❌ 用法: click <行> <列>");
                } else {
                    self.handle_click(&parts[1..]);
                }
            }
            "flag" => {
                if parts.len() != 3 {
                    println!("❌ 用法: flag <行> <列>");
                } else {
                    self.handle_flag(&parts[1..]);
                }
            }
            _ => {
                println!("❌ 未知指令: {}. 输入 'help' 查看帮助", parts[0]);
            }
        }

        true
    }

    fn handle_click(&mut self, coords: &[&str]) {
        match self.parse_coordinates(coords) {
            Some(pos) => {
                let result = self.board.left_click(pos);
                match result {
                    ClickResult::Continue => {
                        println!("✅ 点击成功");
                    }
                    ClickResult::Victory => {
                        println!("🎉 恭喜！你赢了！");
                        self.victory = true;
                        self.game_over = true;
                    }
                    ClickResult::GameOver => {
                        println!("💥 糟糕！你踩到了地雷！");
                        self.game_over = true;
                        // 游戏结束时自动翻开所有格子
                        self.board.reveal_all_mines();
                    }
                    ClickResult::Invalid => {
                        println!("❌ 无效操作（格子已翻开或已标记）");
                    }
                }
            }
            None => {
                println!("❌ 坐标格式错误");
            }
        }
    }

    fn handle_flag(&mut self, coords: &[&str]) {
        match self.parse_coordinates(coords) {
            Some(pos) => {
                let result = self.board.right_click(pos);
                match result {
                    ClickResult::Continue => {
                        println!("🚩 标记操作成功");
                    }
                    ClickResult::Invalid => {
                        println!("❌ 无法标记已翻开的格子");
                    }
                    _ => {} // 标记操作不会导致游戏结束
                }
            }
            None => {
                println!("❌ 坐标格式错误");
            }
        }
    }

    fn parse_coordinates(&self, coords: &[&str]) -> Option<Position> {
        if coords.len() != 2 {
            return None;
        }

        let row = coords[0].parse::<usize>().ok()?;
        let col = coords[1].parse::<usize>().ok()?;

        let config = self.board.get_board_config();
        if row >= config.board_size.height || col >= config.board_size.width {
            println!(
                "❌ 坐标超出范围! 有效范围: 行 0-{}, 列 0-{}",
                config.board_size.height - 1,
                config.board_size.width - 1
            );
            return None;
        }

        Some(Position { row, col })
    }

    fn print_game_over(&self) {
        self.print_board();

        if self.victory {
            println!("🎊🎊🎊 游戏胜利！🎊🎊🎊");
            println!("🏆 你成功找到了所有地雷！");
        } else {
            println!("💀💀💀 游戏结束！💀💀💀");
            println!("💣 不要灰心，再试一次吧！");
        }

        println!("感谢游玩！");
    }
}
