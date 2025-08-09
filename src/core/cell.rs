#[derive(Clone, Copy, Debug)]
pub enum CellContent {
    Mine,
    Number(u8), // 周围地雷的数量， 0-8
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Clone)]
pub struct Cell {
    content: CellContent,
    state: CellState,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            content: CellContent::Number(0), // 默认内容为数字0
            state: CellState::Hidden,        // 默认状态为隐藏
        }
    }

    pub fn content(&self) -> CellContent {
        self.content
    }

    pub fn set_content(&mut self, content: CellContent) {
        self.content = content;
    }

    pub fn state(&self) -> CellState {
        self.state
    }

    pub fn set_state(&mut self, state: CellState) {
        self.state = state;
    }

    pub fn is_mine(&self) -> bool {
        matches!(self.content, CellContent::Mine)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new()
    }
}
