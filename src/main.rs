mod lorenz; 
use lorenz::lorenz::LorenzSystem;

fn main() {
    let beta: f64 = 8.0 / 3.0;
    let mut ls: LorenzSystem = LorenzSystem::new(
        0.01,
        0.01,
        0.01,
        10.0,
        28.0,
        beta,
        0.01,
        1000
    );
    let trajectories: (Vec<f64>, Vec<f64>, Vec<f64>) = ls.simulate();
    println!("\n\n");
    println!("X traj: {:?}\n", trajectories.0);
    println!("Y traj: {:?}\n", trajectories.1);
    println!("Z traj: {:?}\n", trajectories.2);
    ls.display(trajectories);
}