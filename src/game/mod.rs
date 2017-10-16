#[derive(Debug)]
pub struct Game {
    width: usize,
    height: usize,
    board: Vec<bool>
}

impl Game {
    pub fn new(size: usize, alive_cells: &Vec<(usize, usize)>) -> Game {
        let mut board = vec![false; size * size];
        let filtered_points = alive_cells.iter()
            .filter(|p| p.0 < size && p.1 < size);
        for point in filtered_points {
            board[point.1 * size + point.0] = true;
        }
        Game {
            width: size,
            height: size,
            board
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn board(&self) -> &Vec<bool> {
        &self.board
    }

    pub fn next_gen(&mut self) {
        let new_board: Vec<bool> = self.board.iter().enumerate()
            .map(|(idx, is_alive)| {
                match (is_alive, self.count_alive(idx)) {
                    (&true, 2)  => true,
                    (_, 3)      => true,
                    _           => false
                }
            })
            .collect();
        self.board = new_board;
    }

    fn neighbours_coords(&self, idx: usize) -> Vec<usize> {
        let x: usize = idx % self.width;
        let y: usize = idx / self.width;

        let start_x = if x > 0 { x - 1 } else { 0 };
        let start_y = if y > 0 { y - 1 } else { 0 };
        (start_x..(x + 2)).flat_map(|ix| {
            (start_y..(y + 2)).filter_map(move |iy| {
                if ix < self.width && iy < self.height && (x, y) != (ix, iy) {
                    Some(iy * self.width + ix)
                } else {
                    None
                }
            })
        }).collect()
    }

    fn count_alive(&self, idx: usize) -> usize {
        self.neighbours_coords(idx).into_iter()
            .filter(move |idx| self.board[*idx])
            .count()
    }
}
