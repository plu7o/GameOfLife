mod cell;
mod config;
mod game;

use anyhow::Result;
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
    #[arg(short = 'c', long, help = "cluster size")]
    size: Option<usize>,
    #[arg(short, long, help = "cluster density")]
    density: Option<f64>,
    #[arg(short, long, default_value_t = false)]
    info: bool,

    #[arg(short = 'x', long)]
    reproduction: Option<usize>,
    #[arg(short, long)]
    overpopulation: Option<usize>,
    #[arg(short, long)]
    underpopulation: Option<usize>,
    #[arg(short, long)]
    survivability: Option<usize>,
    #[arg(short, long)]
    age: Option<usize>,
    #[arg(short, long)]
    mutation: Option<f64>,
    #[arg(short = 't', long)]
    resitence: Option<usize>,
    #[arg(short = 'g', long)]
    aging: Option<usize>,
    #[arg(short = 'P', long)]
    predetor_rate: Option<f64>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::new(
        cli.width,
        cli.height,
        cli.population.unwrap_or(2000),
        cli.fps.unwrap_or(24),
        cli.radius.unwrap_or(5),
        cli.size.unwrap_or(50),
        cli.density.unwrap_or(0.7),
        cli.info,
        cli.reproduction.unwrap_or(3),
        cli.overpopulation.unwrap_or(4),
        cli.underpopulation.unwrap_or(1),
        cli.survivability.unwrap_or(2),
        cli.age.unwrap_or(100),
        cli.mutation.unwrap_or(0.01),
        cli.resitence.unwrap_or(2),
        cli.aging.unwrap_or(1),
        cli.predetor_rate.unwrap_or(0.01),
    );

    let mut game = GameOfLife::new(config);
    game.run()
}
