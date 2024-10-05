#![allow(unused)]

#[derive(Debug, Clone, Copy)]
pub enum CellState {
    Alive,
    Dead,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub state: CellState,
}

impl Cell {
    pub fn new(x: usize, y: usize, state: CellState) -> Self {
        Self { x, y, state }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            // CellState::Alive => write!(f, "█"),
            CellState::Alive => write!(f, "◈"),
            // CellState::Alive => write!(f, "▢"),
            CellState::Dead => write!(f, " "),
        }
    }
}
