use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use crate::cell::{Cell, CellState};
use crate::config::Config;
use rand::Rng;

// Game Loop (Infinite loop and frame updates)
#[derive(Debug, Clone)]
struct GameState {
    h: usize,
    w: usize,
    map: Vec<Vec<Cell>>,
    offsets: &'static [(isize, isize)],
}

impl GameState {
    fn new(config: Config) -> Self {
        let mut state = Self {
            map: Vec::new(),
            w: config.w,
            h: config.h,
            offsets: config.offsets,
        };
        GameState::gen_empty_map(&mut state);
        state
    }

    fn init(config: Config) -> Self {
        let mut state = GameState::new(config);
        GameState::gen_clustered_random_map(&mut state, config);
        state
    }

    fn gen_empty_map(state: &mut GameState) {
        let w = state.w;
        let h = state.h;

        for y in 0..h {
            let mut row = Vec::new();
            for x in 0..w {
                row.push(Cell::new(x, y, CellState::Dead));
            }
            state.map.push(row);
        }
    }

    fn gen_clustered_random_map(state: &mut GameState, config: Config) {
        let mut rng = rand::thread_rng();
        let cluster_size = config.cluster_size; // Size of random clusters

        for _ in 0..config.population {
            let center_x = rng.gen_range(0..state.w);
            let center_y = rng.gen_range(0..state.h);

            for _ in 0..cluster_size {
                let x_offset = rng.gen_range(-1..=1);
                let y_offset = rng.gen_range(-1..=1);

                let x = ((center_x as isize) + x_offset)
                    .max(0)
                    .min(state.w as isize - 1) as usize;
                let y = ((center_y as isize) + y_offset)
                    .max(0)
                    .min(state.h as isize - 1) as usize;

                state.map[y][x].state = CellState::Alive;
            }
        }
    }
}

pub struct GameOfLife {
    pub config: Config,
    state: GameState,
}

impl GameOfLife {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            state: GameState::init(config),
        }
    }

    pub fn simulate(&mut self) {
        let fps = self.config.fps;

        fn clear() {
            print!("\x1B[2J\x1B[1;1H");
        }

        fn calc_fps(elapsed: Duration, fps: f64) {
            let frame_duration = Duration::from_secs_f64(1.0 / fps);
            if elapsed < frame_duration {
                sleep(frame_duration - elapsed);
            }
        }

        loop {
            let frame_start = Instant::now();

            self.run();

            let frame_end = frame_start.elapsed();
            calc_fps(frame_end, fps as f64);

            let actual_fps = 1.0 / frame_start.elapsed().as_secs_f64();
            println!("FPS: {:.2}", actual_fps);
            clear();
        }
    }

    pub fn generate_ring_offsets(radius: isize) -> Vec<(isize, isize)> {
        let mut offsets = Vec::new();

        for dx in -radius..=radius {
            for dy in -radius..=radius {
                if dx == 0 && dy == 0 {
                    continue;
                }
                // if dx.abs() == radius || dy.abs() == radius {
                offsets.push((dx, dy));
                // }
            }
        }
        offsets
    }

    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut neighbours = 0;
        for (dx, dy) in self.state.offsets.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if let Some(row) = self.state.map.get((ny) as usize) {
                if let Some(cell) = row.get((nx) as usize) {
                    if let CellState::Alive = cell.state {
                        neighbours += 1;
                    }
                }
            }
        }
        neighbours
    }

    fn run(&mut self) {
        let mut next = GameState::new(self.config);
        let mut output_buffer = String::new();

        for (i, row) in self.state.map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let neighbours = self.count_neighbours(j, i);
                next.map[i][j] = match (cell.state, neighbours) {
                    (CellState::Alive, x) if x < 2 => Cell::new(j, i, CellState::Dead), // Underpopulation
                    (CellState::Alive, 2..=3) => Cell::new(j, i, CellState::Alive), // Stay Alive
                    (CellState::Alive, x) if x > 3 => Cell::new(j, i, CellState::Dead), // Overpopulation
                    (CellState::Dead, 3) => Cell::new(j, i, CellState::Alive), // Reproduction
                    _ => Cell::new(j, i, cell.state.clone()),                  // Stay the same
                };
                output_buffer.push_str(&format!("{}", cell));
            }
            output_buffer.push_str("\n");
        }
        self.state = next;
        println!("{}", output_buffer);
    }
}
