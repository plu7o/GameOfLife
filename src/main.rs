mod cell;
mod config;
mod game;

use clap::Parser;
use config::Config;
use game::GameOfLife;

#[derive(Parser)]
#[command(version = "1.0")]
struct Cli {
    #[arg(long)]
    width: Option<usize>,
    #[arg(long)]
    height: Option<usize>,
    #[arg(short, long)]
    population: Option<usize>,
    #[arg(short, long)]
    fps: Option<usize>,
    #[arg(
        short,
        long,
        help = "Set the radius that is checked to determine if a cell lives on or is born"
    )]
    radius: Option<usize>,
    #[arg(short, long)]
    cluster_size: Option<usize>,
}

fn main() {
    let cli = Cli::parse();

    let config = Config::new(
        cli.width.unwrap_or(650),
        cli.height.unwrap_or(110),
        cli.population.unwrap_or(2000),
        cli.fps.unwrap_or(24),
        cli.radius.unwrap_or(1),
        cli.cluster_size.unwrap_or(10),
    );

    let mut game = GameOfLife::new(config);
    game.simulate()
}
