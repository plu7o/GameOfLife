use crate::game::GameOfLife;

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub w: usize,
    pub h: usize,
    pub population: usize,
    pub fps: usize,
    pub offsets: &'static [(isize, isize)],
    pub cluster_size: usize,
}

impl Config {
    pub fn new(
        w: usize,
        h: usize,
        population: usize,
        fps: usize,
        radius: usize,
        cluster_size: usize,
    ) -> Self {
        let offsets: &'static mut [(isize, isize)] =
            GameOfLife::generate_ring_offsets(radius as isize).leak();
        Self {
            w,
            h,
            population,
            fps,
            offsets,
            cluster_size,
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
        }
    }
}
