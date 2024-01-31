pub mod random_matrix_theory {
    use rand::Rng;

    #[derive(Debug)]
    pub struct Matrix {
        pub rows: usize,
        pub cols: usize,
        pub data: Vec<Vec<f64>>
    }

    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Matrix {
            let data: Vec<Vec<f64>> = vec![vec![0.0; cols]; rows];
            Matrix {
                rows,
                cols,
                data
            }
        }
        
        pub fn random_gaussian(&mut self) {
            let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
            for i in 0..self.rows {
                for j in 0..self.cols {
                    let v: f64 = rng.gen();
                    self.data[i][j] = v;
                    self.data[j][i] = v;
                }
            }
        }
    }
}