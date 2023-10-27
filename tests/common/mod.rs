use localsearch_rs::{CostFunction, MetaError, Neighborhood};
use rand::seq::SliceRandom;
use rand::Rng;

const BOARD_SIZE: usize = 8;

type ChessBoard = [[bool; BOARD_SIZE]; BOARD_SIZE];

pub struct EightQueens {}

impl EightQueens {
    pub fn init_solution<R: Rng>(rng: &mut R) -> Result<ChessBoard, MetaError> {
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

    fn get_neighbors<R: Rng>(
        &self,
        rng: &mut R,
        _param: &ChessBoard,
        num_neighbors: Option<u32>,
    ) -> Result<Vec<Self::Neighbor>, MetaError> {
        let neighbors = match num_neighbors {
            Some(val) => (0..val)
                .into_iter()
                .map(|_| (rng.gen_range(0..BOARD_SIZE), rng.gen_range(0..BOARD_SIZE)))
                .collect(),
            None => {
                vec![(rng.gen_range(0..BOARD_SIZE), rng.gen_range(0..BOARD_SIZE))]
            }
        };
        Ok(neighbors)
    }

    fn make_move(
        &self,
        param: &Self::Param,
        neighbor: Self::Neighbor,
    ) -> Result<Self::Param, MetaError> {
        let mut new_state = param.clone();
        let (queen_on_row, queen_new_col) = neighbor;
        new_state[queen_on_row] = [false; BOARD_SIZE];
        new_state[queen_on_row][queen_new_col] = true;
        Ok(new_state)
    }
}

impl CostFunction for EightQueens {
    type Param = ChessBoard;

    fn cost(&self, param: &Self::Param) -> Result<f64, MetaError> {
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
