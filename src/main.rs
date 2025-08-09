fn main() {
    // 将所有 CLI 逻辑委托给库中的 app 模块，保持入口极简
    minesweeper_game::app::run();
}
