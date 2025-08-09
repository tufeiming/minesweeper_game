use crate::core::{Cell, CellContent, CellState};
use rand::seq::SliceRandom;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub enum ClickResult {
    Continue, // 继续游戏
    GameOver, // 踩到地雷
    Victory,  // 胜利
    Invalid,  // 无效操作
}

#[derive(Debug, Clone, Copy)]
pub struct BoardSize {
    pub width: usize,
    pub height: usize,
}

impl BoardSize {
    pub fn cell_count(&self) -> usize {
        self.width * self.height
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoardConfig {
    pub board_size: BoardSize,
    pub mine_count: usize,
}

pub struct Board {
    board_config: BoardConfig,
    cells: Vec<Vec<Cell>>,
    mines_placed: bool,
    revealed_count: usize,
}

fn get_adjacent_mines_count(cells: &[Vec<Cell>], pos: Position, board_size: BoardSize) -> u8 {
    let mut mines_count = 0;
    let adj_positions = get_adjacent_positions(pos, board_size, true);
    for adj_pos in adj_positions {
        if let CellContent::Mine = cells[adj_pos.row][adj_pos.col].content() {
            mines_count += 1
        }
    }
    mines_count
}

fn is_valid_index(index: isize, limit: usize) -> bool {
    index >= 0 && index < (limit as isize)
}

pub fn get_adjacent_positions(pos: Position, size: BoardSize, skip_center: bool) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    for dr in -1..=1 {
        for dc in -1..=1 {
            if skip_center && dr == 0 && dc == 0 {
                continue;
            }

            let new_row = pos.row as isize + dr;
            let new_col = pos.col as isize + dc;
            let width = size.width;
            let height = size.height;
            if is_valid_index(new_row, height) && is_valid_index(new_col, width) {
                positions.push(Position {
                    row: new_row as usize,
                    col: new_col as usize,
                });
            }
        }
    }
    positions
}

impl Board {
    pub fn new(board_config: BoardConfig) -> Self {
        let width = board_config.board_size.width;
        let height = board_config.board_size.height;
        let cells = vec![vec![Cell::new(); width]; height];
        Board {
            board_config,
            cells,
            mines_placed: false,
            revealed_count: 0,
        }
    }

    fn ensure_mines_placed(&mut self, first_click_pos: Position) {
        if !self.mines_placed {
            self.place_mines_avoiding_first_click(first_click_pos);
            self.calculate_numbers();
            self.mines_placed = true;
        }
    }

    fn place_mines_avoiding_first_click(&mut self, first_click_pos: Position) {
        let width = self.board_config.board_size.width;
        let height = self.board_config.board_size.height;
        let mine_count = self.board_config.mine_count;
        let board_size = self.board_config.board_size;

        let forbidden_area = get_adjacent_positions(first_click_pos, board_size, false);

        // 所有坐标位置，排除禁区
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for row in 0..height {
            for col in 0..width {
                if !forbidden_area.contains(&Position { row, col }) {
                    positions.push((row, col));
                }
            }
        }
        if positions.len() < mine_count {
            panic!("No enough positions to place mines!")
        }
        // 随机打乱位置rng
        let mut rng = rand::rng();
        positions.shuffle(&mut rng);
        // 选择前mine_count个位置放置地雷
        for &(row, col) in positions.iter().take(mine_count) {
            self.cells[row][col].set_content(CellContent::Mine);
        }
    }

    fn calculate_numbers(&mut self) {
        // 计算每个单元格周围地雷数量的逻辑
        let board_size = self.board_config.board_size;
        let width = board_size.width;
        let height = board_size.height;
        for row in 0..height {
            for col in 0..width {
                if let CellContent::Mine = self.cells[row][col].content() {
                    continue; // 如果是地雷，跳过
                } else {
                    let pos = Position { row, col };
                    let mines_count = get_adjacent_mines_count(&self.cells, pos, board_size);
                    self.cells[row][col].set_content(CellContent::Number(mines_count));
                }
            }
        }
    }

    pub fn print_debug(&self) {
        let width = self.board_config.board_size.width;
        let height = self.board_config.board_size.height;

        // ANSI 颜色常量
        const RESET: &str = "\x1b[0m";
        const DIM: &str = "\x1b[90m"; // 暗色（列/行号、隐藏）
        const RED: &str = "\x1b[31m"; // 红色（地雷、旗子）
        const BLUE: &str = "\x1b[34m"; // 1
        const GREEN: &str = "\x1b[32m"; // 2
        const YELLOW: &str = "\x1b[33m"; // 5（传统中为棕色，使用黄代替）
        const MAGENTA: &str = "\x1b[35m"; // 4
        const CYAN: &str = "\x1b[36m"; // 6
        const WHITE: &str = "\x1b[37m"; // 7
        const BRIGHT_BLACK: &str = "\x1b[90m"; // 8（浅灰）

        fn color_for_number(n: u8) -> &'static str {
            match n {
                1 => BLUE,
                2 => GREEN,
                3 => RED,
                4 => MAGENTA,
                5 => YELLOW,
                6 => CYAN,
                7 => WHITE,
                8 => BRIGHT_BLACK,
                _ => RESET,
            }
        }

        // 列号表头（使用暗色）
        print!("    ");
        for col in 0..width {
            print!("{}{:>2} {}", DIM, col, RESET);
        }
        println!();

        // 顶部边框
        print!("   +");
        for _ in 0..width {
            print!("---");
        }
        println!("+");

        // 行内容
        for row in 0..height {
            // 行号（暗色）
            print!("{}{:>2}{} |", DIM, row, RESET);
            for col in 0..width {
                let cell = &self.cells[row][col];
                let (glyph, color): (char, &str) = match cell.state() {
                    // 隐藏：白框图标（暗色）
                    CellState::Hidden => ('□', DIM),
                    // 旗子：红色小旗
                    CellState::Flagged => ('⚑', RED),
                    // 已翻开：数字按经典扫雷配色；0 显示为空格；地雷为红色图标
                    CellState::Revealed => match cell.content() {
                        CellContent::Mine => ('✹', RED),
                        CellContent::Number(0) => (' ', RESET),
                        CellContent::Number(n) => (char::from(b'0' + n), color_for_number(n)),
                    },
                };
                // 固定宽度打印，颜色包裹不影响对齐
                print!(" {}{}{} ", color, glyph, RESET);
            }
            println!("|");
        }

        // 底部边框
        print!("   +");
        for _ in 0..width {
            print!("---");
        }
        println!("+");

        // 图例（暗色）
        println!(
            "{}Legend:{} □ hidden, {}⚑{} flag, {}✹{} mine, colored numbers show adjacent mines",
            DIM, RESET, RED, RESET, RED, RESET
        );
    }

    // 左键点击处理
    pub fn left_click(&mut self, pos: Position) -> ClickResult {
        if !self.is_valid_position(pos) {
            return ClickResult::Invalid;
        }

        self.ensure_mines_placed(pos);
        let cell = &mut self.cells[pos.row][pos.col];
        match cell.state() {
            CellState::Revealed => ClickResult::Invalid,
            CellState::Flagged => ClickResult::Invalid,
            CellState::Hidden => {
                cell.set_state(CellState::Revealed);
                match cell.content() {
                    CellContent::Mine => ClickResult::GameOver,
                    CellContent::Number(number) => {
                        self.revealed_count += 1;

                        // 如果是空白格子，自动展开相邻区域
                        if number == 0 {
                            self.auto_reveal_flood_fill(pos);
                        }

                        // 统一检查胜利条件
                        if self.check_victory() {
                            ClickResult::Victory
                        } else {
                            ClickResult::Continue
                        }
                    }
                }
            }
        }
    }

    // 使用队列实现的洪水填充算法，避免递归栈溢出
    fn auto_reveal_flood_fill(&mut self, start_pos: Position) {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        queue.push_back(start_pos);

        while let Some(pos) = queue.pop_front() {
            let adj_positions = get_adjacent_positions(pos, self.board_config.board_size, true);

            for adj_pos in adj_positions {
                let cell = &mut self.cells[adj_pos.row][adj_pos.col];

                // 显式跳过已标记的格子，避免被自动展开覆盖
                if matches!(cell.state(), CellState::Flagged) {
                    continue;
                }

                // 只处理隐藏状态且非地雷的格子
                if matches!(cell.state(), CellState::Hidden)
                    && !matches!(cell.content(), CellContent::Mine)
                {
                    cell.set_state(CellState::Revealed);
                    self.revealed_count += 1;

                    // 如果相邻格子也是空白格子，加入队列继续展开
                    if let CellContent::Number(0) = cell.content() {
                        queue.push_back(adj_pos);
                    }
                }
            }
        }
    }

    pub fn reveal_all_mines(&mut self) {
        for row in &mut self.cells {
            for cell in row {
                if cell.is_mine() {
                    cell.set_state(CellState::Revealed);
                }
            }
        }
    }

    // 右键点击处理
    pub fn right_click(&mut self, pos: Position) -> ClickResult {
        if !self.is_valid_position(pos) {
            ClickResult::Invalid
        } else {
            let cell = &mut self.cells[pos.row][pos.col];
            match cell.state() {
                CellState::Revealed => ClickResult::Invalid,
                CellState::Flagged => {
                    cell.set_state(CellState::Hidden);
                    ClickResult::Continue
                }
                CellState::Hidden => {
                    cell.set_state(CellState::Flagged);
                    ClickResult::Continue
                }
            }
        }
    }

    fn is_valid_position(&self, pos: Position) -> bool {
        let width = self.board_config.board_size.width;
        let height = self.board_config.board_size.height;
        pos.row < height && pos.col < width
    }

    pub fn check_victory(&self) -> bool {
        let mine_count = self.board_config.mine_count;
        let board_size = &self.board_config.board_size;
        self.revealed_count + mine_count == board_size.cell_count()
    }

    // 为游戏引擎提供的公开方法
    pub fn get_cell_content(&self, pos: Position) -> CellContent {
        self.cells[pos.row][pos.col].content()
    }

    pub fn get_cell_state(&self, pos: Position) -> CellState {
        self.cells[pos.row][pos.col].state()
    }

    // 公共访问方法
    pub fn get_board_config(&self) -> &BoardConfig {
        &self.board_config
    }

    // 为集成测试提供的公开方法
    pub fn are_mines_placed(&self) -> bool {
        self.mines_placed
    }

    // 为测试提供的公开方法
    #[cfg(test)]
    pub fn count_mines(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| matches!(cell.content(), CellContent::Mine))
            .count()
    }

    #[cfg(test)]
    pub fn get_revealed_count(&self) -> usize {
        self.revealed_count
    }
}
