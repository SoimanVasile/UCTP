use crate::domain::input_wrapper::TimetableInput;
use std::hash::Hash;
const HARD_CONSTRAINT: u32 = 100000;


#[derive(Debug, Clone)]
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
    /// 4. Student Group Collisions (Hard Constraint)
    /// 5. Teleportation / Building Distance (Hard Constraint)
    /// 6. Time Gaps between classes (Soft Constraint)
    pub fn calculate_penalty(&self, input: &TimetableInput) -> u32 {
        // We accumulate penalties from different checkers here
        self.collision_grid(input) +
            self.gap_teleportation_check(input, &input.groups, |g| g.courses.clone()) + 
            self.gap_teleportation_check(input, &input.teachers, |g| g.course_id.clone())
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
                penalty += HARD_CONSTRAINT;
            }

            // 3. Check Room Type (Hard Constraint)
            if course.required_lab && !room.is_laboratory {
                penalty += HARD_CONSTRAINT;
            }

            // 4. Check Double Booking (Hard Constraint)
            if grid[day as usize][slot as usize][room_id].is_some() {
                penalty += HARD_CONSTRAINT;
            } else {
                // Mark the room as occupied by this course
                grid[day as usize][slot as usize][room_id] = Some(course_id);
            }
        }
        penalty
    }

    /// Checks all Student Group constraints (Collisions, Teleportation, Gaps).
    ///
    /// This function iterates through every student group to reconstruct their personal weekly schedule.
    /// It then identifies three types of issues:
    /// 1. **Student Collision (Hard):** The group is assigned two courses at the same time.
    /// 2. **Teleportation (Hard):** The group has back-to-back classes in different buildings.
    /// 3. **Gaps (Soft):** The group has empty hours between classes during the day.
    ///
    /// # Returns
    /// The combined penalty for all groups.
    pub fn gap_teleportation_check<T, I, F>(&self, input: &TimetableInput, list_of_items: &[T], get_id: F) -> u32 
    where
        I: IntoIterator<Item = usize>,
        F: Fn(&T) -> I{
        let mut penalty: u32 = 0;

        for item in list_of_items{
            // Stack-allocated grid to track this specific group's week.
            // [Day][Slot] -> Option<RoomID>
            let mut grid_teleportation = [[None::<usize>; 6]; 5];
            
            // Phase 1: Fill the grid and check for instant collisions/teleportation
            for course_id in get_id(item).into_iter() {
                penalty += self.check_penalty_teleportation(&mut grid_teleportation, input, &course_id);
            }
            
            // Phase 2: Scan the filled grid for time gaps
            for day in &grid_teleportation {
                penalty += self.check_in_day(day);
            }
        }
        penalty
    }

    /// Helper that places a single course into a Group's schedule and checks immediate constraints.
    ///
    /// # Penalties Applied
    /// * **+10,000 (Student Collision):** If the slot is already occupied.
    /// * **+10,000 (Teleportation):** If the adjacent slots (before/after) have classes in different buildings.
    ///
    /// # Arguments
    /// * `grid_teleportation` - The mutable 5x6 grid for the current group.
    /// * `course_id` - The ID of the course being placed.
    fn check_penalty_teleportation(
        &self, 
        grid_teleportation: &mut [[Option<usize>; 6]; 5], 
        input: &TimetableInput, 
        course_id: &usize
    ) -> u32 {
        let mut penalty = 0;
        
        let (day, slot, room_id) = self.assignments[*course_id];
        
        // Check 1: Student Collision (Hard)
        if grid_teleportation[day as usize][slot as usize].is_some() {
            penalty += HARD_CONSTRAINT;
        } else {
            // Place the course in the grid
            grid_teleportation[day as usize][slot as usize] = Some(room_id);
            
            // Check 2: Teleportation (Look Backwards)
            if slot != 0 {
                penalty += self.check_adjacent(room_id, &grid_teleportation[day as usize][slot as usize - 1], &input);
            }
            // Check 3: Teleportation (Look Forwards)
            let slot_after = slot + 1;
            if slot_after <= 5 {
                penalty += self.check_adjacent(room_id, &grid_teleportation[day as usize][slot_after as usize], &input);
            }
        }
        penalty
    }



    /// Calculates the "Gap Penalty" for a single day.
    ///
    /// A "Gap" is defined as empty slots strictly *between* two classes.
    /// Morning start times and evening end times are not penalized.
    ///
    /// # Scoring Rule (Soft Constraint)
    /// * 1 Slot (2h) gap: **20 points**
    /// * 2 Slot (4h) gap: **15 points**
    /// * 3 Slot (6h) gap: **10 points**
    /// * 4+ Slot (8h+) gap: **5 points**
    fn check_in_day(&self, day: &[Option<usize>; 6]) -> u32 {
        let mut slot: usize = 0;
        let mut penalty = 0;
        let mut gap_size = 0;

        // 1. Skip morning emptiness (Student hasn't arrived yet)
        while slot < 6 && day[slot] == None {
            slot += 1;
        }
        let start = slot as u32;
        let mut end = slot as u32;
        // 2. Scan the "Active Day"
        while slot < 6 {
            if day[slot].is_some() {
                // We found a class. If we were tracking a gap, finalize it.
                if gap_size != 0 {
                    penalty += match gap_size {
                        1 => 20, // 2 hours
                        2 => 15, // 4 hours
                        3 => 10, // 6 hours
                        _ => 5,  // 8+ hours
                    };
                }
                end = slot as u32;
                gap_size = 0; // Reset gap counter
            } else {
                // We found an empty slot within the active day
                gap_size += 1;
            }
            slot += 1;
        }
        let excess: u32 = end-start+1;
        if excess>4{
            penalty += (excess-4)*(excess-4)*50;
        }
        penalty 
    }

    /// Checks if moving between `current_room` and `adjacent_room` is possible.
    ///
    /// # Returns
    /// * **10,000:** If the rooms are in different buildings (Teleportation).
    /// * **0:** If the rooms are in the same building, or if `adjacent_room` is None.
    fn check_adjacent(&self, current_room: usize, adiecent_room: &Option<usize>, input: &TimetableInput) -> u32 {
        let penalty = match adiecent_room {
            None => return 0,
            Some(t) => {
                //This checks if the rooms are in a different building
                if input.rooms[*t].building_id != input.rooms[current_room].building_id {
                    HARD_CONSTRAINT
                } else {
                    0
                }
            },
        };
        penalty
    }

}
