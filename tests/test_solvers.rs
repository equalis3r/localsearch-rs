mod common;

use common::EightQueens;
use localsearch_rs::{Executor, State, TabuSearch, VariableNeighborhood};

#[test]
fn test_variable_neighborhood() {
    let mut rng = rand::thread_rng();
    let problem = EightQueens {};
    let param = EightQueens::init_solution(&mut rng).unwrap();
    let solver = VariableNeighborhood::new(Some(10), &mut rng).with_init_temp(100.0);
    let res = Executor::new(problem, solver)
        .configure(|state| state.param(param).target_cost(0.0))
        .unwrap()
        .run()
        .unwrap();
    let cost = res.state().get_best_cost();
    assert!(cost.abs() < f64::EPSILON);
}

#[test]
fn test_tabu_search() {
    let mut rng = rand::thread_rng();
    let problem = EightQueens {};
    let param = EightQueens::init_solution(&mut rng).unwrap();
    let solver = TabuSearch::new(Some(10), 20, &mut rng).with_init_temp(100.0);
    let res = Executor::new(problem, solver)
        .configure(|state| state.param(param).target_cost(0.0))
        .unwrap()
        .run()
        .unwrap();
    let cost = res.state().get_best_cost();
    assert!(cost.abs() < f64::EPSILON);
}
