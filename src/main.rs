mod lorenz; 
mod double_pendulum;
mod rossler;
mod cellular_automata;

use lorenz::lorenz::LorenzSystem;
use double_pendulum::double_pendulum::DoublePendulum;
use rossler::rossler::RosslerSystem;

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
    let ls_trajectories: (Vec<f64>, Vec<f64>, Vec<f64>) = ls.simulate();
    ls.display(ls_trajectories);

    //let mut dp: DoublePendulum = DoublePendulum::new_random(0.001, 9.81, 10000);
    //let dp_trajectories: (Vec<f64>, Vec<f64>) = dp.simulate();
    //dp.display(dp_trajectories);

    let mut rs: RosslerSystem = RosslerSystem::new(
        1.0,
        1.0,
        1.0,
        0.15,
        0.225,
        20.0,
        0.001,
        10000
    );
    let rs_trajectories: (Vec<f64>, Vec<f64>, Vec<f64>) = rs.simulate();
    rs.display(rs_trajectories);
}