use crate::errors::LocalSearchError;
use rand::Rng;
use std::{collections::HashMap, hash::Hash};

/// To use `crate::GuidedLocalSearch`, a penalty struct is required.
///
/// * `alpha`:
/// * `lambda`:
/// * `values`:
#[derive(Clone)]
pub struct Penalty<F: Hash + Eq> {
    pub alpha: f64,
    pub lambda: f64,
    pub values: HashMap<F, f64>,
}

impl<F: Hash + Eq> Penalty<F> {
    pub fn new(alpha: f64) -> Self {
        Self {
            alpha,
            lambda: 0.0,
            values: HashMap::default(),
        }
    }

    pub fn get(&self, feature: &F) -> f64 {
        self.values.get(feature).map_or(0.0, |val| *val)
    }

    pub fn calibrate(&mut self, ratio: f64) {
        for (_, val) in self.values.iter_mut() {
            *val = ratio;
        }
    }

    pub fn update(&mut self, feature: F) {
        self.values
            .entry(feature)
            .and_modify(|fea| *fea += 1.0)
            .or_insert(1.0);
    }

    pub fn utility(&self, feature: &F, cost: f64) -> f64 {
        cost / (1.0 + self.get(feature))
    }
}

/// Given a set of parameter, the problem needs to know the cost of that parameters
pub trait CostFunction {
    type Param;

    fn cost(&self, param: &Self::Param) -> Result<f64, LocalSearchError>;
}

/// A problem needs to know how to get to other neighbors of a parameter.
pub trait Neighborhood {
    type Param;
    type Neighbor;

    fn get_neighbor_moves<R: Rng>(
        &self,
        rng: &mut R,
        param: &Self::Param,
    ) -> Result<Vec<Self::Neighbor>, LocalSearchError>;

    /// To avoid costly calculation, we only consider the delta of moving from a param
    /// to its neighbor.
    fn get_neighbor_delta(
        &self,
        param: &Self::Param,
        neighbor: &Self::Neighbor,
    ) -> Result<f64, LocalSearchError>;

    fn make_move(
        &self,
        param: &Self::Param,
        neighbor: &Self::Neighbor,
    ) -> Result<Self::Param, LocalSearchError>;
}

/// To use `crate::GuidedLocalSearch`, we need to also implement `AugmentedNeighborhood` /// trait on top of `Neighborhood` trait.
pub trait AugmentedNeighborhood<F> {
    type Param;
    type Neighbor;
    type Penalty;

    fn get_neighbor_augmented_delta(
        &self,
        param: &Self::Param,
        neighbor: &Self::Neighbor,
        penalty: &Self::Penalty,
    ) -> Result<f64, LocalSearchError>;

    fn update_penalty(
        &self,
        param: &Self::Param,
        penalty: &mut Self::Penalty,
    ) -> Result<(), LocalSearchError>;

    /// Compute number of features
    fn number_of_features(&self, param: &Self::Param) -> Result<u32, LocalSearchError>;
}

#[cfg(test)]
mod tests {
    use super::Penalty;

    #[test]
    fn test_penalty_new() {
        let penalty = Penalty::<i32>::new(0.5);
        assert_eq!(penalty.alpha, 0.5);
        assert_eq!(penalty.lambda, 0.0);
        assert!(penalty.values.is_empty());
    }

    #[test]
    fn test_penalty_get() {
        let mut penalty = Penalty::<i32>::new(0.5);
        penalty.values.insert(1, 2.0);
        assert_eq!(penalty.get(&1), 2.0);
        assert_eq!(penalty.get(&2), 0.0);
    }

    #[test]
    fn test_penalty_calibrate() {
        let mut penalty = Penalty::<i32>::new(0.5);
        penalty.values.insert(1, 2.0);
        penalty.values.insert(2, 3.0);
        penalty.calibrate(1.0);
        assert_eq!(penalty.get(&1), 1.0);
        assert_eq!(penalty.get(&2), 1.0);
    }

    #[test]
    fn test_penalty_update() {
        let mut penalty = Penalty::<i32>::new(0.5);
        penalty.update(1);
        assert_eq!(penalty.get(&1), 1.0);
        penalty.update(1);
        assert_eq!(penalty.get(&1), 2.0);
    }

    #[test]
    fn test_penalty_utility() {
        let mut penalty = Penalty::<i32>::new(0.5);
        penalty.values.insert(1, 2.0);
        assert_eq!(penalty.utility(&1, 10.0), 10.0 / 3.0);
    }
}