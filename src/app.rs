use crate::config::constants::ui_text;
use crate::config::difficulty::Difficulty;
use crate::core::game::Game;
use std::io::{self, Write};

/// 应用程序主入口点
pub fn run() {
    println!("{}", ui_text::GAME_TITLE);
    println!("{}", ui_text::DIFFICULTY_PROMPT);
    println!("{}", ui_text::easy_desc());
    println!("{}", ui_text::medium_desc());
    println!("{}", ui_text::hard_desc());
    println!("{}", ui_text::DEMO_DESC);

    let difficulty = loop {
        print!("{}", ui_text::INPUT_PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("{}", ui_text::INPUT_ERROR);
            continue;
        }

        match input.trim() {
            "1" => break Difficulty::Easy,
            "2" => break Difficulty::Medium,
            "3" => break Difficulty::Hard,
            "4" => {
                crate::demo::run_demo_mode();
                return;
            }
            _ => {
                println!("{}", ui_text::INVALID_CHOICE);
                continue;
            }
        }
    };

    println!("{}", ui_text::GAME_START);

    // 创建并运行游戏
    let mut game = Game::new(difficulty);
    game.run();
}
