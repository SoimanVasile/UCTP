use UCTP::domain::schedule::Schedule;
use UCTP::domain::input_wrapper::TimetableInput;

#[cfg(test)]
mod tests {
    use super::*;
    use UCTP::domain::{course::Course, group::Group, room::Room, teacher::Teacher};

    // Helper to create a dummy input with:
    // - 1 Group (100 students)
    // - 2 Rooms (Room 0: Capacity 200, Normal; Room 1: Capacity 50, Lab)
    // - 2 Courses (Course 0: Lecture; Course 1: Lab)
    fn create_mock_input() -> TimetableInput {
        TimetableInput {
            rooms: vec![
                Room { // Room 0: Big Lecture Hall
                    id: 0, name: "C1".to_string(), capacity: 200, 
                    is_laboratory: false, free: vec![], building_id: 0 
                },
                Room { // Room 1: Small Lab
                    id: 1, name: "L1".to_string(), capacity: 50, 
                    is_laboratory: true, free: vec![], building_id: 0 
                },
            ],
            groups: vec![
                Group { id: 0, name: "G1".to_string(), numbers_of_students: 100, courses: vec![] }
            ],
            teachers: vec![], // Not needed for these tests
            courses: vec![
                Course { // Course 0: Big Lecture (Needs 100 seats, No Lab)
                    id: 0, subject_name: "Math".to_string(), professor_id: 0,
                    group_ids: vec![0], required_hours: 2, required_lab: false
                },
                Course { // Course 1: Physics Lab (Needs 100 seats, Is Lab)
                    id: 1, subject_name: "Physics".to_string(), professor_id: 0,
                    group_ids: vec![0], required_hours: 2, required_lab: true
                },
            ],
        }
    }

    #[test]
    fn test_valid_schedule() {
        let input = create_mock_input();
        
        // Course 0 (Lecture) -> Room 0 (Big Hall) -> OK
        // Course 1 (Lab)     -> Room 1 is too small (50 < 100), so we must pick a Big Lab if existed.
        // Wait, Room 1 is capacity 50. Group is 100.
        // Let's testing a VALID assignment first.
        // We put Course 0 in Room 0.
        
        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), // Mon, 8am, Room 0 (Big Hall) for Course 0
                (0, 2, 1), // Mon, 12pm, Room 1 (Lab) for Course 1
            ]
        };
        
        // NOTE: This should actually FAIL capacity for Course 1 (100 students > 50 cap)
        // Let's see if your code catches it.
        let penalty = schedule.calculate_penalty(&input);
        
        // Expect: 10,000 penalty (Capacity overflow on Room 1)
        assert_eq!(penalty, 10000, "Should punish capacity overflow");
    }

    #[test]
    fn test_perfect_schedule() {
        // Let's modify the input so Room 1 is big enough
        let mut input = create_mock_input();
        input.rooms[1].capacity = 150; // Now big enough for 100 students

        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), // Course 0 -> Room 0 (Valid)
                (0, 2, 1), // Course 1 -> Room 1 (Valid Lab)
            ]
        };

        let penalty = schedule.calculate_penalty(&input);
        assert_eq!(penalty, 0, "Perfect schedule should have 0 penalty");
    }

    #[test]
    fn test_double_booking_collision() {
        let mut input = create_mock_input();
        input.rooms[1].capacity = 150; // Fix capacity so we only test collision

        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), // Course 0 -> Monday Slot 0, Room 0
                (0, 0, 0), // Course 1 -> Monday Slot 0, Room 0 (COLLISION!)
            ]
        };

        let penalty = schedule.calculate_penalty(&input);
        
        // We expect ONE collision penalty.
        // However, Course 1 is a LAB, and Room 0 is NOT a Lab.
        // So Course 1 generates: Collision (10k) AND RoomType Mismatch (10k).
        // Total = 20,000.
        assert_eq!(penalty, 20000, "Should punish Collision AND Room Type mismatch");
    }

    #[test]
    fn test_room_type_mismatch() {
        let input = create_mock_input();
        
        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), // Course 0 (Lecture) -> Room 0 (Hall) -> OK
                (0, 2, 0), // Course 1 (Lab)     -> Room 0 (Hall) -> ERROR (Not a lab)
            ]
        };

        let penalty = schedule.calculate_penalty(&input);
        assert_eq!(penalty, 10000, "Should punish putting a Lab in a Lecture Hall");
    }
}
