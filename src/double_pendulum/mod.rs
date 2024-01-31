pub mod double_pendulum {
    use plotters::prelude::*;
    use rand::Rng;
    pub struct DoublePendulum {
        initial_theta_1: f64,
        initial_theta_2: f64,
        initial_omega_1: f64,
        initial_omega_2: f64,
        dt: f64,
        g: f64,
        steps: usize
    }

    impl DoublePendulum {
        /// Creates a DoublePendulum object
        /// 
        /// *Params
        /// 
        /// initial_theta_1: initial angle of the first pendulum
        /// 
        /// initial_theta_2: initial angle of the second pendulum
        /// 
        /// initial_omega_1: initial angular velocity of the first pendulum
        /// 
        /// initial_omega_2: initial angular velocity of the second pendulum
        /// 
        /// dt: small time increment used in each iteration of the simulation
        /// 
        /// g: acceleration due to gravity. On Earth, g = 9.81
        pub fn new(initial_theta_1: f64, initial_theta_2: f64, initial_omega_1: f64, initial_omega_2: f64, dt: f64, g: f64, steps: usize) -> DoublePendulum {
            DoublePendulum {
                initial_theta_1,
                initial_theta_2,
                initial_omega_1,
                initial_omega_2,
                dt,
                g,
                steps
            }
        }

        /// Generates a Double Pendulum model with random attributes.
        /// 
        /// Be careful! Creating a random model will almost always have unpredicatable and faulty behavior.
        /// 
        /// *Params
        /// 
        /// dt: small time increment used in each iteration of the simulation
        /// 
        /// g: acceleration due to gravity
        /// 
        /// steps: number of iterations
        pub fn new_random(dt: f64, g: f64, steps: usize) -> DoublePendulum {
            let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
            let max_value: f64 = 90.0;
            let min_value: f64 = 0.0;
            let theta1: f64 = rng.gen_range(min_value..max_value);
            let theta2: f64 = rng.gen_range(min_value..max_value);
            let omega1: f64 = rng.gen_range(min_value..max_value);
            let omega2: f64 = rng.gen_range(min_value..max_value);

            println!("Creating random Double Pendulum model");
            println!("Theta 1: {}", theta1);
            println!("Theta 2: {}", theta2);
            println!("Omega 1: {}", omega1);
            println!("Omega 2: {}", omega2);

            DoublePendulum {
                initial_theta_1: theta1,
                initial_theta_2: theta2,
                initial_omega_1: omega1,
                initial_omega_2: omega2,
                dt,
                g,
                steps
            }
        }

        /// Returns the trajectories of the double pendulum
        pub fn simulate(&mut self) -> (Vec<f64>, Vec<f64>) {
            let mut theta1: f64 = self.initial_theta_1;
            let mut theta2: f64 = self.initial_theta_2;
            let mut omega1: f64 = self.initial_omega_1;
            let mut omega2: f64 = self.initial_omega_2;

            let mut theta1_values: Vec<f64> = Vec::with_capacity(self.steps);
            let mut theta2_values: Vec<f64> = Vec::with_capacity(self.steps);

            for _ in 0..self.steps {
                theta1_values.push(theta1);
                theta2_values.push(theta2);

                let numerator_alpha1 = -self.g * (2.0 * theta1.sin()
                + (theta1 - 2.0 * theta2).sin()
                + 2.0 * (theta1 - theta2).sin() * (omega2.powi(2) + omega1.powi(2) * (theta1 - theta2).cos()));
                let denominator_alpha1 = 3.0 - (2.0 * theta1 - 2.0 * theta2).cos();
                let alpha1 = numerator_alpha1 / denominator_alpha1;
                let term1_alpha2 = 2.0 * (theta1 - theta2).sin();
                let term2_alpha2 = 2.0 * theta1.cos() * omega1.powi(2);
                let term3_alpha2 =
                    self.g * (2.0 * theta1.sin() + (theta1 - 2.0 * theta2).sin() + 2.0 * (theta1 - theta2).sin()
                        * (omega2.powi(2) + omega1.powi(2) * (theta1 - theta2).cos()));
                let numerator_alpha2 = term1_alpha2 * (term2_alpha2 + term3_alpha2);
                let denominator_alpha2 = 3.0 - (2.0 * theta1 - 2.0 * theta2).cos();
                let alpha2 = numerator_alpha2 / denominator_alpha2;

                omega1 = omega1 + alpha1 * self.dt;
                omega2 = omega2 + alpha2 * self.dt;
                theta1 = theta1 + omega1 * self.dt;
                theta2 = theta2 + omega2 * self.dt;

                //println!("Theta 1: {}", theta1);
                //println!("Theta 2: {}", theta2);
                //println!("Omega 1: {}", omega1);
                //println!("Omega 2: {}", omega2);
            }
            //println!("Theta 1 Values: {:?}", theta1_values);
            //println!("Theta 2 Values: {:?}", theta2_values);
            (theta1_values, theta2_values)
        }

        /// Plots the Double Pendulum trajectories
        pub fn display(&self, trajectories: (Vec<f64>, Vec<f64>)) {
            let root: DrawingArea<BitMapBackend<'_>, plotters::coord::Shift> = BitMapBackend::new("double_pendulum_plot.png", (800, 600)).into_drawing_area();
            root.fill(&WHITE).unwrap();

            let mut chart: ChartContext<'_, BitMapBackend<'_>, Cartesian2d<plotters::coord::types::RangedCoordf64, plotters::coord::types::RangedCoordf64>> = ChartBuilder::on(&root)
                .caption("Lorenz System Trajectories", ("Arial", 20).into_font())
                .margin(5)
                .x_label_area_size(40)
                .y_label_area_size(40)
                .build_cartesian_2d(0.0..self.steps as f64, -100.0..100.0)
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
                .label("Pendulum 1")
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

            chart
                .draw_series(LineSeries::new(trajectories.1.iter().enumerate().map(|(i, &val)| (i as f64, val)), &GREEN))
                .unwrap()
                .label("Pendulum 2")
                .legend(|(x, y)| PathElement::new(vec![(x, y - 10), (x + 20, y - 10)], &GREEN));
        }
    }

}