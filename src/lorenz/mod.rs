pub mod lorenz {
    use plotters::prelude::*;
    pub struct LorenzSystem {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub sigma: f64,
        pub rho: f64,
        pub beta: f64,
        pub dt: f64,
        pub steps: usize
    }

    impl LorenzSystem {
        /// Creates a LorenzSystem object
        /// 
        /// *Params
        /// 
        /// x: initial x value
        /// 
        /// y: initial y value
        /// 
        /// z: initial z value
        /// 
        /// sigma: Represents the Prandtl number. Controls the rate at which temperature differences in the fluid translate into velocity differences.
        /// Typically set to 10
        /// 
        /// rho: Represents the Rayleigh number. Influences the onset of convection.
        /// Typically set to 28
        /// 
        /// beta: Represents the ratio of the width to the height of the convecting fluid layer
        /// Typically set to 8 / 3
        /// 
        /// dt: small time increment used in each iteration of the simulation
        /// Typically set in a range from 0.001 to 0.1
        /// 
        /// steps: number of iterations in the simulation
        pub fn new(x: f64, y: f64, z: f64, sigma: f64, rho: f64, beta: f64, dt: f64, steps: usize) -> LorenzSystem {
            LorenzSystem {
                x,
                y,
                z,
                sigma,
                rho,
                beta,
                dt,
                steps
            }
        }

        /// Returns the x, y, and z trajectories of the lorenz system
        pub fn simulate(&mut self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
            let mut x_values: Vec<f64> = Vec::with_capacity(self.steps);
            let mut y_values: Vec<f64> = Vec::with_capacity(self.steps);
            let mut z_values: Vec<f64> = Vec::with_capacity(self.steps);
            
            for _ in 0..self.steps {
                x_values.push(self.x);
                y_values.push(self.y);
                z_values.push(self.z);
                
                let dx: f64 = self.sigma * (self.y - self.x);
                let dy: f64 = self.x * (self.rho - self.z) - self.y;
                let dz: f64 = self.x * self.y - self.beta * self.z;

                self.x += self.dt * dx;
                self.y += self.dt * dy;
                self.z += self.dt * dz;
            }

            (x_values, y_values, z_values)
        }

        /// Plots the Lorenz system trajectories
        pub fn display(&self, trajectories: (Vec<f64>, Vec<f64>, Vec<f64>)) {
            let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> = BitMapBackend::new("lorenz_plot.png", (800, 600)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let mut chart = ChartBuilder::on(&root)
                .caption("Lorenz System Trajectories", ("Arial", 20).into_font())
                .margin(5)
                .x_label_area_size(40)
                .y_label_area_size(40)
                .build_cartesian_2d(0.0..self.steps as f64, -20.0..20.0)
                .unwrap();

            chart
                .configure_mesh()
                .x_desc("Time Step")
                .y_desc("Values")
                .draw()
                .unwrap();

            chart
                .draw_series(LineSeries::new(trajectories.0.iter().enumerate().map(|(i, &val)| (i as f64, val)), &RED))
                .unwrap()
                .label("X Trajectory")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

            chart
                .draw_series(LineSeries::new(trajectories.1.iter().enumerate().map(|(i, &val)| (i as f64, val)), &GREEN))
                .unwrap()
                .label("Y Trajectory")
                .legend(|(x, y)| PathElement::new(vec![(x, y - 10), (x + 20, y - 10)], &GREEN));

            chart
                .draw_series(LineSeries::new(trajectories.2.iter().enumerate().map(|(i, &val)| (i as f64, val)), &BLUE))
                .unwrap()
                .label("Z Trajectory")
                .legend(|(x, y)| PathElement::new(vec![(x, y - 20), (x + 20, y - 20)], &BLUE));

            chart.configure_series_labels().background_style(&WHITE.mix(0.8)).draw().unwrap();
        }
    }
}