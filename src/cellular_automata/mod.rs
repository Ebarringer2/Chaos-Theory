pub mod cellular_automata {
    #[derive(Clone)]
    pub struct Cell {
        pub alive: bool,
        pub dead: bool
    }

    pub struct Grid {
        pub height: usize,
        pub width: usize,
        pub cells: Vec<Cell>
    }

    impl Grid {
        /// Creates a new Grid object.
        /// 
        /// *Params
        /// 
        /// height: y dimension of the grid
        /// 
        /// width: x dimension of the grid
        /// 
        /// num_cells: number of cells in the grid
        pub fn new(height: usize, width: usize, num_cells: usize) -> Grid {
            let mut cells: Vec<Cell> = Vec::with_capacity(num_cells);
            for _ in 0..num_cells {
                cells.push(Cell {alive: true, dead: false })
            }

            Grid {
                height,
                width,
                cells
            }
        }

        /// Simulates the Cellular Automata grid with an inputted number of iterations
        pub fn simulate(&mut self, steps: usize) {
            for _ in 0..steps {
                self.update_cells();
                self.print_grid();
            }
        }

        fn get_index(&self, x: usize, y: usize) -> usize {
            y * self.width + x
        }

        fn get_neighbors(&self, x: usize, y: usize) -> Vec<usize> {
            let mut neighbors = Vec::new();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx = (x as isize + dx) as usize;
                    let ny = (y as isize + dy) as usize;

                    if nx < self.width && ny < self.height {
                        neighbors.push(self.get_index(nx, ny));
                    }
                }
            }
            neighbors
        }

        fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
            let neighbors = self.get_neighbors(x, y);
            neighbors.iter().filter(|&&idx| self.cells[idx].alive).count()
        }

        fn update_cells(&mut self) {
            let mut new_cells = self.cells.clone();

            for x in 0..self.width {
                for y in 0..self.height {
                    let idx = self.get_index(x, y);
                    let alive_neighbors = self.count_alive_neighbors(x, y);

                    if self.cells[idx].alive {
                        new_cells[idx].alive = alive_neighbors == 2 || alive_neighbors == 3;
                    } else {
                        new_cells[idx].alive = alive_neighbors == 3;
                    }

                    new_cells[idx].dead = !new_cells[idx].alive;
                }
            }

            self.cells = new_cells;
        }

        fn print_grid(&self) {
            for y in 0..self.height {
                for x in 0..self.width {
                    let idx = self.get_index(x, y);
                    print!(
                        "{} ",
                        if self.cells[idx].alive {
                            'X'
                        } else {
                            '.'
                        }
                    );
                }
                println!();
            }
            println!();
        }
    }
}