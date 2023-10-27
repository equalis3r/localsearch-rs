use crate::errors::LocalSearchError;
use rand::Rng;

#[derive(Clone)]
pub struct Problem<O>(O);

impl<O> Problem<O> {
    pub fn new(problem: O) -> Self {
        Self(problem)
    }
}

impl<O> std::ops::Deref for Problem<O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<O> std::ops::DerefMut for Problem<O> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait CostFunction {
    /// Type of the parameter vector
    type Param;

    /// Compute cost function
    /// # Errors
    ///
    /// Will return `Err` if
    fn cost(&self, param: &Self::Param) -> Result<f64, LocalSearchError>;
}

pub trait Neighborhood {
    type Param;
    type Neighbor;

    /// # Errors
    ///
    /// Will return `Err` if
    fn get_neighbors<R: Rng>(
        &self,
        rng: &mut R,
        param: &Self::Param,
        num_neighbors: Option<u32>,
    ) -> Result<Vec<Self::Neighbor>, LocalSearchError>;

    /// # Errors
    ///
    /// Will return `Err` if
    fn make_move(
        &self,
        param: &Self::Param,
        neighbor: Self::Neighbor,
    ) -> Result<Self::Param, LocalSearchError>;
}
