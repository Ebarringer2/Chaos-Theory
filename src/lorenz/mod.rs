mod lorenz {
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
                self.x += self.dt * dz;
            }

            (x_values, y_values, z_values)
        }

        /// Plots the Lorenz system trajectories
        pub fn display(&self, trajectories: (Vec<f64>, Vec<f64>, Vec<f64>)) {
            let root = BitMapBackend::new("lorenz_attractor.png", (800, 600)).into_drawing_area();
            root.fill(&WHITE).unwrap();
            let mut chart = ChartBuilder::on(&root)
                .margin(5)
                .set_label_area_size(LabelAreaPosition::Left, 40)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption("Lorenz Attractor", ("Arial", 20).into_font())
                .build_cartesian_3d(-20.0..20.0, -30.0..30.0, 0.0..50.0)
                .unwrap();
            chart.with_projection(|mut pb| {
                pb.yaw = 3.0;
                pb.pitch = 0.5;
                pb.scale = 1.5;
                pb.into_matrix()
            });
            chart.configure_axes().draw().unwrap();
            chart.draw_series(
                LineSeries::new(
                    (0..self.steps).map(|i| (trajectories.0[i], trajectories.1[i], trajectories.2[i])),
                    &BLACK,
                )
                .style(ShapeStyle {
                    fill_color: Some(&BLACK.mix(0.5)),
                    line_width: 1,
                    color: &BLACK,
                }),                
            );
        }
    }
}