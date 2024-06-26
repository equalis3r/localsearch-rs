use localsearch_rs::{CostFunction, LocalSearchError, Neighborhood};
use rand::seq::SliceRandom;
use rand::Rng;

const BOARD_SIZE: usize = 8;

type ChessBoard = [[bool; BOARD_SIZE]; BOARD_SIZE];

pub struct EightQueens {}

impl EightQueens {
    pub fn init_solution<R: Rng>(rng: &mut R) -> Result<ChessBoard, LocalSearchError> {
        let mut board = [[false; BOARD_SIZE]; BOARD_SIZE];
        let mut queen_cols: Vec<usize> = (0..BOARD_SIZE).collect();
        queen_cols.shuffle(rng);
        for i in 0..BOARD_SIZE {
            board[i][queen_cols[i]] = true;
        }
        Ok(board)
    }
}

impl Neighborhood for EightQueens {
    type Param = ChessBoard;
    type Neighbor = (usize, usize);

    fn get_neighbor_moves<R: Rng>(
        &self,
        rng: &mut R,
        _param: &ChessBoard,
    ) -> Result<Vec<Self::Neighbor>, LocalSearchError> {
        Ok((0..100)
            .into_iter()
            .map(|_| (rng.gen_range(0..BOARD_SIZE), rng.gen_range(0..BOARD_SIZE)))
            .collect())
    }

    fn get_neighbor_delta(
        &self,
        param: &Self::Param,
        neighbor: &Self::Neighbor,
    ) -> Result<f64, LocalSearchError> {
        let mut new_state = param.clone();
        let (queen_on_row, queen_new_col) = neighbor;
        new_state[*queen_on_row] = [false; BOARD_SIZE];
        new_state[*queen_on_row][*queen_new_col] = true;
        Ok(self.cost(&new_state)? - self.cost(param)?)
    }

    fn make_move(
        &self,
        param: &Self::Param,
        neighbor: &Self::Neighbor,
    ) -> Result<Self::Param, LocalSearchError> {
        let mut new_state = param.clone();
        let (queen_on_row, queen_new_col) = neighbor;
        new_state[*queen_on_row] = [false; BOARD_SIZE];
        new_state[*queen_on_row][*queen_new_col] = true;
        Ok(new_state)
    }
}

impl CostFunction for EightQueens {
    type Param = ChessBoard;

    fn cost(&self, param: &Self::Param) -> Result<f64, LocalSearchError> {
        let mut num_attacking = 0;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if param[i][j] {
                    // Check for attacks in the same row
                    for k in 0..BOARD_SIZE {
                        if k != j && param[i][k] {
                            num_attacking += 1;
                        }
                    }

                    // Check for attacks in the same column
                    for k in 0..BOARD_SIZE {
                        if k != i && param[k][j] {
                            num_attacking += 1;
                        }
                    }

                    // Check for attacks on diagonals
                    for k in 1..BOARD_SIZE {
                        if i >= k && j >= k && param[i - k][j - k] {
                            num_attacking += 1;
                        }
                        if i + k < BOARD_SIZE && j + k < BOARD_SIZE && param[i + k][j + k] {
                            num_attacking += 1;
                        }
                        if i >= k && j + k < BOARD_SIZE && param[i - k][j + k] {
                            num_attacking += 1;
                        }
                        if i + k < BOARD_SIZE && j >= k && param[i + k][j - k] {
                            num_attacking += 1;
                        }
                    }
                }
            }
        }
        Ok(num_attacking as f64)
    }
}
