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
        let mut rng = rand::thread_rng();
        let mut current_assignments: Schedule = self.generate_first_schedule();
        let mut current_penalty: u32 = current_assignments.calculate_penalty(&self.input);

        let mut best_assignments = self.generate_first_schedule();

        best_assignments.assignments = current_assignments.assignments.clone();
        let mut best_penalty = current_penalty;

        let mut start_temp = self.start_temp;
        for _ in 0..self.max_iterations{
            let neighbour_schedule = self.generate_neighbour(&current_assignments);
            let neighbour_penalty = neighbour_schedule.calculate_penalty(&self.input);

        }
        best_assignments
    }

    fn generate_neighbour(&self, current_assignments: &Schedule) -> Schedule{
        let mut rng = rand::thread_rng();

        let rand_course_id = rng.gen_range(0..current_assignments.assignments.len());
        let day = rng.gen_range(0..5);
        let slot = rng.gen_range(0..6);
        let room_id = rng.gen_range(0..self.input.rooms.len());

        let mut neighbour_assignments = current_assignments.assignments.clone();
        neighbour_assignments[rand_course_id] = (day, slot, room_id);

        Schedule { assignments: neighbour_assignments }
    }
}
