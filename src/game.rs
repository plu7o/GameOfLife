use std::{
    io::{self, Stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};

use crate::cell::{Cell, CellState, CellType};
use crate::config::Config;
use anyhow::Result;
use crossterm::{
    cursor,
    style::{self, Color, Print, Stylize},
    QueueableCommand,
};
use rand::{thread_rng, Rng};

// Game Loop (Infinite loop and frame updates)
#[derive(Debug, Clone)]
struct GameState {
    h: usize,
    w: usize,
    map: Vec<Vec<Cell>>,
    offsets: &'static [(isize, isize)],
    preys: usize,
    predators: usize,
    generation: usize,
}

impl GameState {
    fn new(config: Config) -> Self {
        let mut state = Self {
            map: vec![vec![Cell::default(); config.w]; config.h],
            w: config.w,
            h: config.h,
            offsets: config.offsets,
            preys: 0,
            predators: 0,
            generation: 0,
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
            for x in 0..w {
                state.map[y][x] = Cell::new(x, y, CellState::Dead, CellType::Prey);
            }
        }
    }

    fn gen_clustered_random_map(state: &mut GameState, config: Config) {
        let mut rng = rand::thread_rng();
        let cluster_size = config.cluster_size as isize;
        let cluster_density = config.cluster_density;
        let predetor_rate = config.predetor_rate;

        for _ in 0..config.population {
            let center_x = rng.gen_range(0..state.w);
            let center_y = rng.gen_range(0..state.h);

            let range_x = ((center_x as isize - cluster_size).max(0) as usize)
                ..=((center_x as isize + cluster_size).min(state.w as isize - 1) as usize);
            let range_y = ((center_y as isize - cluster_size).max(0) as usize)
                ..=((center_y as isize + cluster_size).min(state.h as isize - 1) as usize);

            for x in range_x.clone() {
                for y in range_y.clone() {
                    if rng.gen_bool(cluster_density) {
                        state.map[y][x].state = CellState::Alive(1);
                        state.map[y][x].kind = if rng.gen_bool(predetor_rate) {
                            CellType::Predetor
                        } else {
                            CellType::Prey
                        };
                    }
                }
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

    fn info(&self) -> String {
        let mut info = String::new();
        info.push_str(&format!("generation: {}\n", self.state.generation));
        info.push_str(&format!("Preys: {}\n", self.state.preys));
        info.push_str(&format!("predators: {}\n", self.state.predators));

        info.push_str(&format!("Width: {}\n", self.config.w));
        info.push_str(&format!("Height: {}\n", self.config.h));
        info.push_str(&format!("Population: {}\n", self.config.population));
        info.push_str(&format!("cluster_size: {}\n", self.config.cluster_size));
        info.push_str(&format!(
            "cluster_density: {}\n",
            self.config.cluster_density
        ));
        info.push_str(&format!("age: {}\n", self.config.age));
        info.push_str(&format!("mutation: {}\n", self.config.mutation));
        info.push_str(&format!("Reproduction: {}\n", self.config.reproduction));
        info.push_str(&format!("Overpopulation: {}\n", self.config.overpopulation));
        info.push_str(&format!(
            "Underpopulation: {}\n",
            self.config.underpopulation
        ));
        info.push_str(&format!("survival: {}\n", self.config.survivability));
        info.push_str(&format!("resistence: {}\n", self.config.resistence));
        info.push_str(&format!("aging_rate: {}\n", self.config.aging_rate));
        info.push_str(&format!("predetor_rate: {}\n", self.config.predetor_rate));
        info
    }

    pub fn run(&mut self) -> Result<()> {
        let mut stdout = io::stdout();
        let fps = self.config.fps;

        let clear = |stdout: &mut Stdout| -> Result<()> {
            stdout.queue(cursor::MoveTo(0, 0))?;
            stdout.flush()?;
            Ok(())
        };

        let hold_fps = |elapsed: Duration| {
            let frame_duration = Duration::from_secs_f64(1.0 / fps as f64);
            if elapsed < frame_duration {
                sleep(frame_duration - elapsed);
            }
        };

        let print_info = |stdout: &mut Stdout, frame_start: Instant, info: String| -> Result<()> {
            let actual_fps = 1.0 / frame_start.elapsed().as_secs_f64();

            stdout
                .queue(cursor::MoveTo(0, 0))?
                .queue(style::PrintStyledContent(
                    format!("FPS: {actual_fps}").blue(),
                ))?
                .queue(style::PrintStyledContent(format!("\n{}", info).blue()))?;
            Ok(())
        };

        stdout.queue(cursor::Hide)?;
        clear(&mut stdout)?;

        loop {
            let frame_start = Instant::now();
            self.simulate(&mut stdout)?;
            let frame_end = frame_start.elapsed();

            if self.config.info {
                print_info(&mut stdout, frame_start, self.info())?;
            }

            hold_fps(frame_end);
            clear(&mut stdout)?;
        }
    }

    fn simulate(&mut self, stdout: &mut Stdout) -> Result<()> {
        let mut next = GameState::new(self.config);
        next.generation = self.state.generation + 1;
        self.draw(&mut next, stdout)?;
        // self.state = next;
        std::mem::swap(&mut self.state, &mut next);
        Ok(())
    }

    fn draw(&self, next: &mut GameState, stdout: &mut Stdout) -> Result<()> {
        let mut buf = String::new();
        for row in self.state.map.iter() {
            for cell in row.iter() {
                self.apply_rules(cell, next);
                let symbol = match cell.state {
                    CellState::Alive(age) => {
                        let live = (age as f64 / self.config.age as f64) * 100.0;
                        let color = if live >= 75.0 {
                            match cell.kind {
                                // Red for Prey
                                CellType::Prey => Color::AnsiValue(196),
                                // Blue for Predator
                                CellType::Predetor => Color::AnsiValue(21),
                            }
                        } else if live >= 50.0 {
                            match cell.kind {
                                //  Orange for Prey
                                CellType::Prey => Color::AnsiValue(214),
                                // Magenta for Predator
                                CellType::Predetor => Color::AnsiValue(201),
                            }
                        } else if live >= 25.0 {
                            match cell.kind {
                                // Yellow for Prey
                                CellType::Prey => Color::AnsiValue(226),
                                // Violet for Predator
                                CellType::Predetor => Color::AnsiValue(135),
                            }
                        } else {
                            match cell.kind {
                                // Green for Prey
                                CellType::Prey => Color::AnsiValue(47),
                                // Cyan for Predator
                                CellType::Predetor => Color::AnsiValue(81),
                            }
                        };

                        match cell.kind {
                            CellType::Prey => next.preys += 1,
                            CellType::Predetor => next.predators += 1,
                        };
                        style::style(format!("{}", cell)).with(color)
                    }
                    CellState::Dead => {
                        match cell.kind {
                            CellType::Prey => next.preys.saturating_sub(1),
                            CellType::Predetor => next.predators.saturating_sub(1),
                        };
                        // Dark grey
                        style::style(format!("{}", cell)).black()
                    }
                };
                buf.push_str(&symbol.to_string());
            }
            buf.push_str("\n");
        }
        stdout.queue(cursor::MoveTo(0, 0))?.queue(Print(buf))?;
        Ok(())
    }

    fn apply_rules(&self, cell: &Cell, next: &mut GameState) {
        let reprod = self.config.reproduction;
        let over = self.config.overpopulation;
        let under = self.config.underpopulation;
        let survival = self.config.survivability;
        let max_age = self.config.age;
        let resistence = self.config.resistence;
        let aging = self.config.aging_rate;
        let mutation = thread_rng().gen_bool(self.config.mutation);
        let (prey, predators) = self.count_neighbours(cell.x, cell.y);

        next.map[cell.y][cell.x] = match cell.state {
            CellState::Alive(age) => {
                match cell.kind {
                    CellType::Prey => {
                        // Age - prey dies of old age
                        if age >= max_age {
                            Cell::new(cell.x, cell.y, CellState::Dead, cell.kind)
                        }
                        // Eaten by predator - prey dies if predators nearby exceed resistance threshold
                        else if predators >= resistence && !thread_rng().gen_bool(0.01) {
                            Cell::new(cell.x, cell.y, CellState::Dead, cell.kind)
                        }
                        // Underpopulation - prey dies if not enough prey around to survive
                        else if prey < under {
                            Cell::new(cell.x, cell.y, CellState::Dead, cell.kind)
                        }
                        // Overpopulation - prey dies due to overcrowding
                        else if prey > over {
                            Cell::new(cell.x, cell.y, CellState::Dead, cell.kind)
                        }
                        // Prey reproduction - breed new prey based on reproduction conditions
                        else if prey >= reprod && prey <= over && thread_rng().gen_bool(0.1) {
                            Cell::prey(cell.x, cell.y, CellState::Alive(1)) // Newborn prey
                        }
                        // Stay Alive - prey survives under normal conditions
                        else if prey >= survival && prey <= over {
                            Cell::prey(cell.x, cell.y, CellState::Alive(age + 1))
                        } else {
                            Cell::new(cell.x, cell.y, cell.state, cell.kind)
                        }
                    }
                    CellType::Predetor => {
                        // Age - predator dies of old age
                        if age >= max_age {
                            Cell::new(cell.x, cell.y, CellState::Dead, cell.kind)
                        }
                        // Predator dies due to underpopulation - not enough other predators around
                        else if predators < under {
                            Cell::new(cell.x, cell.y, CellState::Dead, cell.kind)
                        }
                        // Predator dies due to overpopulation - too many predators in one area
                        else if predators > over {
                            Cell::new(cell.x, cell.y, CellState::Dead, cell.kind)
                        }
                        // Predator dies of hunger if it hasn't found food for too long
                        else if prey == 0 && thread_rng().gen_bool(0.5) {
                            Cell::new(cell.x, cell.y, CellState::Dead, cell.kind)
                        // Starve to death
                        }
                        // Predator survives if it finds prey and isn't overcrowded
                        else if prey > 0 && predators <= over {
                            Cell::prededator(cell.x, cell.y, CellState::Alive(age + 1))
                        // Reset hunger after eating
                        }
                        // No food, predator ages faster and gets hungrier
                        else if prey == 0 {
                            Cell::prededator(cell.x, cell.y, CellState::Alive(age + aging))
                        }
                        // Predator reproduction - breed new predator based on reproduction conditions
                        else if prey > 0 && predators <= over && thread_rng().gen_bool(0.1) {
                            Cell::prededator(cell.x, cell.y, CellState::Alive(1))
                            // Newborn predator
                        } else {
                            Cell::new(cell.x, cell.y, cell.state, cell.kind)
                        }
                    }
                }
            }
            CellState::Dead => {
                // Prey reproduction or mutation - spawn new prey if conditions are met
                if prey == reprod || mutation {
                    Cell::prey(cell.x, cell.y, CellState::Alive(1)) // Newborn or mutated prey
                }
                // Predator reproduction or mutation - spawn new predator if conditions are met
                else if predators > 0 && prey == 0 || mutation {
                    Cell::prededator(cell.x, cell.y, CellState::Alive(1)) // Newborn or mutated predator
                }
                // Stay dead if no reproduction or mutation occurs
                else {
                    Cell::new(cell.x, cell.y, cell.state, cell.kind)
                }
            }
        };
    }

    fn count_neighbours(&self, x: usize, y: usize) -> (usize, usize) {
        let mut preys = 0;
        let mut predators = 0;

        for (dx, dy) in self.state.offsets.iter() {
            let nx = x.wrapping_add(*dx as usize);
            let ny = y.wrapping_add(*dy as usize);
            if nx < self.config.w && ny < self.config.h {
                if let CellState::Alive(_) = self.state.map[ny][nx].state {
                    if let CellType::Prey = self.state.map[ny][nx].kind {
                        preys += 1;
                    } else {
                        predators += 1;
                    }
                }
            }
        }
        (preys, predators)
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
