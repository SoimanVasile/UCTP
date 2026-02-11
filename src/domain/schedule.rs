use crate::domain::input_wrapper::TimetableInput;

pub struct Schedule{
    pub assignments: Vec<(u32, u32, usize)>,
}

impl Schedule{
    pub fn calculate_penalty(&self, input: &TimetableInput){
        
    }

    fn collision_grid(&self, input: &TimetableInput) -> u32{
        let mut penalty: u32 = 0;
        let mut grid = vec![vec![vec![None::<usize>; input.rooms.len()]; 6]; 5];
        for (course_id, assignment) in self.assignments.iter().enumerate(){
            let (day, slot, room_id) = *assignment;
            
            let course = input.get_course(course_id);
            let room = input.get_room(room_id);
            if room.capacity < course.capacity_needed(&input.groups){
                penalty += 10000
            }

            if course.required_lab == true && room.is_laboratory == false{
                penalty += 10000
            }

            if grid[day as usize][slot as usize][room_id].is_some(){
                penalty += 10000
            }
            else{
                grid[day as usize][slot as usize][room_id] = Some(course_id);
            }
        }
        penalty
    }
}
