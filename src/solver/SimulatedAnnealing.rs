use crate::domain::{input_wrapper::TimetableInput, schedule::Schedule};
use rand::Rng;

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
        let mut rng = rand::thread_rng();
        let mut assignments: Vec<(u32, u32, usize)> = Vec::new();

        for _ in 0..self.input.courses.len(){
            let day: u32 = rng.gen_range(0..5);
            let slot: u32 = rng.gen_range(0..6);
            let room_id: usize = rng.gen_range(0..self.input.rooms.len()); 
            assignments.push((day, slot, room_id));
        }

        Schedule { assignments }
    }

    pub fn run(&self) -> Schedule{
        let assignments: Vec<(u32, u32, usize)> = Vec::new();

        Schedule { assignments }
    }
}
