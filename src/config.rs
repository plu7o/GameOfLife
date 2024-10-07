use crate::game::generate_ring_offsets;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub w: usize,
    pub h: usize,
    pub population: usize,
    pub fps: usize,
    pub offsets: &'static [(isize, isize)],
    pub cluster_size: usize,
    pub cluster_density: f64,
    pub info: bool,
    pub reproduction: usize,
    pub overpopulation: usize,
    pub underpopulation: usize,
    pub survivability: usize,
    pub age: usize,
    pub mutation: f64,
    pub resistence: usize,
    pub aging_rate: usize,
    pub predetor_rate: f64,
}

impl Config {
    pub fn new(
        w: Option<usize>,
        h: Option<usize>,
        population: usize,
        fps: usize,
        radius: usize,
        cluster_size: usize,
        cluster_density: f64,
        info: bool,
        reproduction: usize,
        overpopulation: usize,
        underpopulation: usize,
        survivability: usize,
        age: usize,
        mutation: f64,
        resistence: usize,
        aging_rate: usize,
        predetor_rate: f64,
    ) -> Self {
        let offsets: &'static mut [(isize, isize)] = generate_ring_offsets(radius as isize).leak();
        Self {
            w: w.unwrap_or(crossterm::terminal::size().unwrap().0.into()),
            h: h.unwrap_or(crossterm::terminal::size().unwrap().1.into()),
            population,
            fps,
            offsets,
            cluster_size,
            cluster_density,
            info,
            reproduction,
            overpopulation,
            underpopulation,
            survivability,
            age,
            mutation,
            resistence,
            aging_rate,
            predetor_rate,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            w: 470,
            h: 110,
            population: 2000,
            fps: 24,
            offsets: &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
            cluster_size: 10,
            cluster_density: 0.5,
            info: true,
            reproduction: 3,
            overpopulation: 3,
            underpopulation: 2,
            survivability: 2,
            age: 100,
            mutation: 0.01,
            resistence: 1,
            aging_rate: 5,
            predetor_rate: 0.01,
        }
    }
}
