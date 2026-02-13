use crate::domain::input_wrapper::TimetableInput;

/// Represents a candidate solution for the Timetable Problem.
/// It contains a list of assignments where the index corresponds to the Course ID.
pub struct Schedule {
    /// A flat vector representing the gene code.
    /// - Index: Course ID (from normalized input)
    /// - Value: (Day, Slot, RoomID)
    ///   - Day: 0..4 (Mon-Fri)
    ///   - Slot: 0..5 (2-hour blocks)
    ///   - RoomID: Index in the input.rooms vector
    pub assignments: Vec<(u32, u32, usize)>,
}

impl Schedule {
    /// Calculates the total "Energy" (Cost) of this schedule.
    /// Lower energy means a better schedule.
    ///
    /// Currently sums up penalties from:
    /// 1. Room Collisions (Hard Constraint)
    /// 2. Room Capacity Overflow (Hard Constraint)
    /// 3. Laboratory Mismatches (Hard Constraint)
    pub fn calculate_penalty(&self, input: &TimetableInput) -> u32 {
        // We accumulate penalties from different checkers here
        self.collision_grid(input)+self.gap_teleportation_check(input)
    }

    /// Checks for Hard Constraints related to Room Usage.
    ///
    /// # Constraints Checked:
    /// * **Capacity:** Does the room fit all students? (+10,000 penalty)
    /// * **Room Type:** If the course needs a Lab, is the room a Lab? (+10,000 penalty)
    /// * **Double Booking:** Is the room already occupied at this time? (+10,000 penalty)
    ///
    /// # Returns
    /// The total penalty score for these constraints.
    pub fn collision_grid(&self, input: &TimetableInput) -> u32 {
        let mut penalty: u32 = 0;
        
        // A 3D Grid to track room usage: [Day][Slot][RoomID]
        // Used to detect double-booking in O(1) time.
        let mut grid = vec![vec![vec![None::<usize>; input.rooms.len()]; 6]; 5];

        for (course_id, assignment) in self.assignments.iter().enumerate() {
            let (day, slot, room_id) = *assignment;
            
            // 1. Retrieve Context
            let course = input.get_course(course_id);
            let room = input.get_room(room_id);

            // 2. Check Capacity (Hard Constraint)
            if room.capacity < course.capacity_needed(&input.groups) {
                penalty += 10000;
            }

            // 3. Check Room Type (Hard Constraint)
            if course.required_lab && !room.is_laboratory {
                penalty += 10000;
            }

            // 4. Check Double Booking (Hard Constraint)
            if grid[day as usize][slot as usize][room_id].is_some() {
                penalty += 10000;
            } else {
                // Mark the room as occupied by this course
                grid[day as usize][slot as usize][room_id] = Some(course_id);
            }
        }
        penalty
    }

    pub fn gap_teleportation_check(&self, input: &TimetableInput) -> u32{
        let mut penalty: u32 = 0;

        for group in &input.groups{
            let mut grid_teleportation = [[None::<usize>; 6]; 5];
            for course_id in &group.courses{
                if *course_id >= self.assignments.len(){
                    continue;
                }
                let (day, slot, room_id) = self.assignments[*course_id];
                println!("day: {}, slot:{}", day, slot);
                if grid_teleportation[day as usize][slot as usize].is_some(){
                    penalty += 10000;
                } else{
                    grid_teleportation[day as usize][slot as usize] = Some(room_id);
                    println!("{:?}", grid_teleportation);
                    let slot_after = slot+1;
                    if slot!=0{
                        penalty += self.check_adiecent(room_id, &grid_teleportation[day as usize][slot as usize-1], &input);
                    }
                    if slot_after <= 5 {
                        penalty += self.check_adiecent(room_id, &grid_teleportation[day as usize][slot_after as usize], &input);
                    }
                }
            }
            for day in &grid_teleportation{
                println!("{}", penalty);
                penalty += self.check_in_day(day);
            }
        }
        penalty
    }
    fn check_in_day(&self, day: &[Option<usize>; 6]) -> u32{
        let mut slot: usize = 0;
        let mut penalty = 0;
        let mut gap = 0;
        println!("day: {:?}", day);
        while slot < 6 && day[slot] == None{
            slot+=1;
        }
        while slot<6{
            if day[slot].is_some(){
                println!("slot: {}, gap:{}",slot, gap);
                if gap != 0 {
                    penalty += 20-(gap-1)*5;
                }
                gap = 0;
            }
            else {
                gap += 1;
            }
            slot+=1;
        }
        penalty 
    }
    fn check_adiecent(&self, current_room: usize, adiecent_room: &Option<usize>, input: &TimetableInput) -> u32{
            let penalty = match adiecent_room{
                None => return 0,
                Some(t) =>{println!("{}    {}", t, current_room); if input.rooms[*t].building_id != input.rooms[current_room].building_id {10000} else {0}},
            };
            penalty
    }
}
