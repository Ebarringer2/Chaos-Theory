pub mod rossler {
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
        pub fn new(&self, x: f64, y: f64, z: f64, a: f64, b: f64, c: f64, dt: f64, steps: usize) -> RosslerSystem {
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
        pub fn simulate(&self) -> (f64, f64, f64) {
            let mut x: f64 = self.x;
            let mut y: f64 = self.y;
            let mut z: f64 = self.z;

            for _ in 0..self.steps {
                let dx: f64 = -y - z;
                let dy: f64 = x + self.a * y;
                let dz: f64 = self.b + z * (x - self.c);

                x = x + self.dt * dx;
                y = y + self.dt * dy;
                z = z + self.dt * dz;
            }

            (x, y, z)
        }
    }
}