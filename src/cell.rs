#[derive(Debug, Clone, Copy)]
pub enum CellState {
    Alive(usize),
    Dead,
}

#[derive(Debug, Clone, Copy)]
pub enum CellType {
    Prey,
    Predetor,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub state: CellState,
    pub kind: CellType,
}

impl Cell {
    pub fn new(x: usize, y: usize, state: CellState, kind: CellType) -> Self {
        Self { x, y, state, kind }
    }

    pub fn prey(x: usize, y: usize, state: CellState) -> Self {
        Self {
            x,
            y,
            state,
            kind: CellType::Prey,
        }
    }

    pub fn prededator(x: usize, y: usize, state: CellState) -> Self {
        Self {
            x,
            y,
            state,
            kind: CellType::Predetor,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            state: CellState::Dead,
            kind: CellType::Prey,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state {
            CellState::Dead => write!(f, " "),
            CellState::Alive(_) => match self.kind {
                CellType::Prey => write!(f, "◈"),
                CellType::Predetor => write!(f, "{}", '¤'),
            }, // CellState::Alive(_) => write!(f, "{}", '¤'),
        }
    }
}

// static GLYPHSS: [char; 175] = [
//     'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
//     'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
//     'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', // Numbers
//     '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', // Special Characters
//     '@', '#', '$', '%', '&', '*', '+', '-', '=', '~', '`', '^', '\'', '"', '\\', '/', '|', '_',
//     ':', ';', '.', ',', '<', '>', '?', '[', ']', '{', '}', '(', ')', '!', '¤', '÷', '×', '·',
//     '°', // Unicode Symbols
//     '░', '▒', '▓', '█', '▀', '▄', '■', '□', '≡', '≠', '≤', '≥', '∞', '∴', '∵', '⌐', '¬', '±', '∑',
//     '∆', '∇', 'π', 'λ', 'θ', '∅', 'µ', 'ξ', 'β', 'Ω', 'Σ', 'Ξ', 'Ψ', '★', '☆', '☀', '☼', '☽', '☾',
//     '☁', '⚡', '❄', '❇', '✦', '✧', '✩', '✪', '✫', '✬', '✭', '✮', '✯', '☢', '☣', '☤', '☥', '☦', '☧',
//     '☨', '☩', '☪', '☫', '☬', '☭', '☯', '☸', '☹', '☺', '☻', '♠', '♣', '♥', '♦', '♤', '♧', '♡', '♢',
// ];
//
// fn random_glyph() -> char {
//     let mut rng = rand::thread_rng();
//     let idx = rng.gen_range(0..175);
//     GLYPHSS[idx]
// }
