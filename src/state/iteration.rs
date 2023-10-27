use crate::state::State;
use crate::termination::{Reason, Status};
use core::mem;
use core::time;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Iteration<P> {
    pub param: Option<P>,
    pub prev_param: Option<P>,
    pub best_param: Option<P>,
    pub prev_best_param: Option<P>,
    pub cost: f64,
    pub prev_cost: f64,
    pub best_cost: f64,
    pub prev_best_cost: f64,
    pub target_cost: f64,
    pub iter: u32,
    pub last_best_iter: u32,
    pub max_iters: u32,
    pub time: Option<std::time::Duration>,
    pub max_time: Option<std::time::Duration>,
    pub termination_status: Status,
}

impl<P> Iteration<P>
where
    Self: State,
{
    #[must_use]
    pub fn param(mut self, param: P) -> Self {
        std::mem::swap(&mut self.prev_param, &mut self.param);
        self.param = Some(param);
        self
    }

    #[must_use]
    pub fn cost(mut self, cost: f64) -> Self {
        std::mem::swap(&mut self.prev_cost, &mut self.cost);
        self.cost = cost;
        self
    }

    #[must_use]
    pub fn target_cost(mut self, target_cost: f64) -> Self {
        self.target_cost = target_cost;
        self
    }

    #[must_use]
    pub fn max_iters(mut self, iters: u32) -> Self {
        self.max_iters = iters;
        self
    }

    #[must_use]
    pub fn max_time(mut self, time: std::time::Duration) -> Self {
        self.max_time = Some(time);
        self
    }

    pub fn get_prev_cost(&self) -> f64 {
        self.prev_cost
    }

    pub fn get_prev_best_cost(&self) -> f64 {
        self.prev_best_cost
    }

    pub fn take_param(&mut self) -> Option<P> {
        self.param.take()
    }

    pub fn get_prev_param(&self) -> Option<&P> {
        self.prev_param.as_ref()
    }

    pub fn take_prev_param(&mut self) -> Option<P> {
        self.prev_param.take()
    }

    pub fn get_prev_best_param(&self) -> Option<&P> {
        self.prev_best_param.as_ref()
    }

    pub fn take_best_param(&mut self) -> Option<P> {
        self.best_param.take()
    }

    pub fn take_prev_best_param(&mut self) -> Option<P> {
        self.prev_best_param.take()
    }
}

impl<P> State for Iteration<P>
where
    P: Clone,
{
    type Param = P;

    fn new() -> Self {
        Self {
            param: None,
            prev_param: None,
            best_param: None,
            prev_best_param: None,
            cost: f64::INFINITY,
            prev_cost: f64::INFINITY,
            best_cost: f64::INFINITY,
            prev_best_cost: f64::INFINITY,
            target_cost: f64::NEG_INFINITY,
            iter: 0,
            last_best_iter: 0,
            max_iters: u32::MAX,
            time: Some(time::Duration::new(0, 0)),
            max_time: Some(time::Duration::MAX),
            termination_status: Status::NotTerminated,
        }
    }

    fn update(&mut self) {
        if self.cost < self.best_cost
            || (self.cost.is_infinite()
                && self.best_cost.is_infinite()
                && self.cost.is_sign_positive() == self.best_cost.is_sign_positive())
        {
            // If there is no parameter vector, then also don't set the best param.
            if let Some(param) = self.param.as_ref().cloned() {
                mem::swap(&mut self.prev_best_param, &mut self.best_param);
                self.best_param = Some(param);
            }
            mem::swap(&mut self.prev_best_cost, &mut self.best_cost);
            self.best_cost = self.cost;
            self.last_best_iter = self.iter;
        }
    }

    fn get_param(&self) -> Option<&P> {
        self.param.as_ref()
    }

    fn get_best_param(&self) -> Option<&P> {
        self.best_param.as_ref()
    }

    fn time(&mut self, time: Option<time::Duration>) -> &mut Self {
        self.time = time;
        self
    }

    fn terminate_with(mut self, reason: Reason) -> Self {
        self.termination_status = Status::Terminated(reason);
        self
    }

    fn get_cost(&self) -> f64 {
        self.cost
    }

    fn get_best_cost(&self) -> f64 {
        self.best_cost
    }

    fn get_target_cost(&self) -> f64 {
        self.target_cost
    }

    fn get_iter(&self) -> u32 {
        self.iter
    }

    fn get_last_best_iter(&self) -> u32 {
        self.last_best_iter
    }

    fn get_max_iters(&self) -> u32 {
        self.max_iters
    }

    fn get_termination_status(&self) -> &Status {
        &self.termination_status
    }

    fn get_termination_reason(&self) -> Option<&Reason> {
        match &self.termination_status {
            Status::Terminated(reason) => Some(reason),
            Status::NotTerminated => None,
        }
    }

    fn get_time(&self) -> Option<time::Duration> {
        self.time
    }

    fn increment_iter(&mut self) {
        self.iter += 1;
    }

    fn is_best(&self) -> bool {
        self.last_best_iter == self.iter
    }
}
