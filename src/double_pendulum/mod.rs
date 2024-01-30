pub mod double_pendulum {

    pub struct DoublePendulum {
        initial_theta_1: f64,
        initial_theta_2: f64,
        initial_omega_1: f64,
        initial_omega_2: f64,
        dt: f64,
        g: f64,
        alpha: f64,
        steps: usize
    }

    impl DoublePendulum {
        /// Creates a DoublePendulum object
        pub fn new(initial_theta_1: f64, initial_theta_2: f64, initial_omega_1: f64, initial_omega_2: f64, dt: f64, g: f64, alpha: f64, steps: usize) -> DoublePendulum {
            DoublePendulum {
                initial_theta_1,
                initial_theta_2,
                initial_omega_1,
                initial_omega_2,
                dt,
                g,
                alpha,
                steps
            }
        }
    }

}