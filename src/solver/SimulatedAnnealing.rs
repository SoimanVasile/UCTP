use crate::domain::{input_wrapper::TimetableInput, schedule::Schedule};
use rand;

#[derive(Debug, Clone)]
pub struct simulated_annealing{
    pub input: TimetableInput,
    pub start_temp: f64,
    pub cooling_rate: f64,
    pub max_iterations: u32,
}

impl simulated_annealing{
    pub fn new(input: TimetableInput, start_temp: f64, cooling_rate: f64, max_iterations: u32) -> Self {
        Self {
            input,
            start_temp,
            cooling_rate,
            max_iterations,
        }
    }

    fn generate_first_schedule(&self) -> Schedule{
        let mut assignments: Vec<(u32, u32, usize)> = Vec::new();

        Schedule { assignments }
    }

    pub fn run(&self) -> Schedule{
        let assignments: Vec<(u32, u32, usize)> = Vec::new();

        Schedule { assignments }
    }
}
