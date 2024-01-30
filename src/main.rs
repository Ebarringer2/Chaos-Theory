mod lorenz; 
mod double_pendulum;

use lorenz::lorenz::LorenzSystem;
use double_pendulum::double_pendulum::DoublePendulum;

fn main() {
    let beta: f64 = 8.0 / 3.0;
    let mut ls: LorenzSystem = LorenzSystem::new(
        0.01,
        0.01,
        0.01,
        10.0,
        28.0,
        beta,
        0.001,
        10000
    );
    let trajectories: (Vec<f64>, Vec<f64>, Vec<f64>) = ls.simulate();
    ls.display(trajectories);
}