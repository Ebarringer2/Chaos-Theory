pub mod rossler {
    use plotters::prelude::*;
    pub struct RosslerSystem {
        pub x: f64,
        pub y: f64,
        pub z: f64,
        pub a: f64,
        pub b: f64,
        pub c: f64,
        pub dt: f64,
        pub steps: usize
    }

    impl RosslerSystem {
        /// Creates a new Rossler System object
        /// 
        /// *Params
        /// 
        /// x, y, z: state variables of the system, which represent the position of the system in 3d space
        /// 
        /// a: controls the stretching and folding of the system in the y direction
        /// 
        /// b: influences the overall speed of the system's motion
        /// 
        /// c: shifts the system along the x axis
        /// 
        /// dt: small time increment used in each iteration of the simulation
        /// 
        /// steps: number of iterations in the simulation
        pub fn new(x: f64, y: f64, z: f64, a: f64, b: f64, c: f64, dt: f64, steps: usize) -> RosslerSystem {
            RosslerSystem {
                x, 
                y,
                z,
                a,
                b,
                c,
                dt,
                steps
            }
        }

        /// Returns the points of the Rossler System after the simulation is completed
        pub fn simulate(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
            let mut x_values: Vec<f64> = Vec::with_capacity(self.steps);
            let mut y_values: Vec<f64> = Vec::with_capacity(self.steps);
            let mut z_values: Vec<f64> = Vec::with_capacity(self.steps);

            let mut x: f64 = self.x;
            let mut y: f64 = self.y;
            let mut z: f64 = self.z;

            for _ in 0..self.steps {
                x_values.push(x);
                y_values.push(y);
                z_values.push(z);

                let dx: f64 = -y - z;
                let dy: f64 = x + self.a * y;
                let dz: f64 = self.b + z * (x - self.c);

                x = x + self.dt * dx;
                y = y + self.dt * dy;
                z = z + self.dt * dz;
            }

            (x_values, y_values, z_values)
        }
        
        /// Displays the Rossler system trajectories
        pub fn display(&self, trajectories: (Vec<f64>, Vec<f64>, Vec<f64>)) {
            let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> = BitMapBackend::new("rossler_plot.png", (800, 600)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let mut chart = ChartBuilder::on(&root)
                .caption("Rossler System Trajectories", ("Arial", 20).into_font())
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