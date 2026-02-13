#[cfg(test)]
mod tests {
    use UCTP::domain::{
        course::Course, group::Group, input_wrapper::TimetableInput, room::Room, schedule::Schedule,
    };

    // --- Helper: Build a world with specific buildings ---
    fn create_teleport_scenario_with2_courses() -> TimetableInput {
        TimetableInput {
            rooms: vec![
                Room {
                    id: 0,
                    name: "Room A (Building 1)".to_string(),
                    capacity: 100,
                    is_laboratory: false,
                    free: vec![],
                    building_id: 1, // <--- Building 1
                },
                Room {
                    id: 1,
                    name: "Room B (Building 2)".to_string(),
                    capacity: 100,
                    is_laboratory: false,
                    free: vec![],
                    building_id: 2, // <--- Building 2 (Different!)
                },
                Room {
                    id: 2,
                    name: "Room C (Building 1)".to_string(),
                    capacity: 100,
                    is_laboratory: false,
                    free: vec![],
                    building_id: 1, // <--- Building 1 (Same as A)
                },
            ],
            groups: vec![Group {
                id: 0,
                name: "G1".to_string(),
                numbers_of_students: 10,
                courses: vec![0, 1], // This group attends both courses
            }],
            courses: vec![
                Course {
                    id: 101,
                    subject_name: "Math".to_string(),
                    professor_id: 0,
                    group_ids: vec![0], // Attended by G1
                    required_hours: 2,
                    required_lab: false,
                },
                Course {
                    id: 102,
                    subject_name: "Physics".to_string(),
                    professor_id: 0,
                    group_ids: vec![0], // Attended by G1
                    required_hours: 2,
                    required_lab: false,
                },
            ],
            teachers: vec![],
        }
    }

    fn create_teleport_scenario_with3_courses() -> TimetableInput {
        TimetableInput {
            rooms: vec![
                Room {
                    id: 0,
                    name: "Room A (Building 1)".to_string(),
                    capacity: 100,
                    is_laboratory: false,
                    free: vec![],
                    building_id: 1, // <--- Building 1
                },
                Room {
                    id: 1,
                    name: "Room B (Building 2)".to_string(),
                    capacity: 100,
                    is_laboratory: false,
                    free: vec![],
                    building_id: 2, // <--- Building 2 (Different!)
                },
                Room {
                    id: 2,
                    name: "Room C (Building 1)".to_string(),
                    capacity: 100,
                    is_laboratory: false,
                    free: vec![],
                    building_id: 1, // <--- Building 1 (Same as A)
                },
            ],
            groups: vec![Group {
                id: 0,
                name: "G1".to_string(),
                numbers_of_students: 10,
                courses: vec![0, 1, 2], // This group attends all the courses
            }],
            courses: vec![
                Course {
                    id: 101,
                    subject_name: "Math".to_string(),
                    professor_id: 0,
                    group_ids: vec![0], // Attended by G1
                    required_hours: 2,
                    required_lab: false,
                },
                Course {
                    id: 102,
                    subject_name: "Physics".to_string(),
                    professor_id: 0,
                    group_ids: vec![0], // Attended by G1
                    required_hours: 2,
                    required_lab: false,
                },
            ],
            teachers: vec![],
        }
    }

    #[test]
    fn test_teleportation_penalty() {
        let input = create_teleport_scenario_with2_courses();

        // Scenario: Back-to-back classes in DIFFERENT buildings
        // Slot 0: Room 0 (Building 1)
        // Slot 1: Room 1 (Building 2)
        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), // Mon, 8-10, Room 0
                (0, 1, 1), // Mon, 10-12, Room 1 (Different Building!)
            ],
        };

        let penalty = schedule.gap_teleportation_check(&input);
        assert_eq!(penalty, 10000, "Should punish moving between buildings instantly");
    }

    #[test]
    fn test_no_teleportation_penalty() {
        let input = create_teleport_scenario_with2_courses();

        // Scenario: Back-to-back classes in SAME building
        // Slot 0: Room 0 (Building 1)
        // Slot 1: Room 2 (Building 1)
        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), // Mon, 8-10, Room 0
                (0, 1, 2), // Mon, 10-12, Room 2 (Same Building)
            ],
        };

        let penalty = schedule.gap_teleportation_check(&input);
        assert_eq!(penalty, 0, "Should allow moving within the same building");
    }

    // --- GAP TESTS ---

    #[test]
    fn test_gap_2_hours() {
        let input = create_teleport_scenario_with2_courses(); // Re-use helper (rooms don't matter for gaps)

        // Scenario: Class, Empty, Class (1 Slot Gap)
        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), // Mon, 8-10
                // Gap at 10-12 (Slot 1)
                (0, 2, 0), // Mon, 12-14
            ],
        };

        let penalty = schedule.gap_teleportation_check(&input);
        assert_eq!(penalty, 20, "2 hour gap should be 20 points");
    }

    #[test]
    fn test_gap_4_hours() {
        let input = create_teleport_scenario_with2_courses();

        // Scenario: Class, Empty, Empty, Class (2 Slot Gap)
        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), // Mon, 8-10
                // Gap at 10-12 (Slot 1)
                // Gap at 12-14 (Slot 2)
                (0, 3, 0), // Mon, 14-16
            ],
        };

        let penalty = schedule.gap_teleportation_check(&input);
        assert_eq!(penalty, 15, "4 hour gap should be 15 points");
    }

    #[test]
    fn test_gap_6_hours() {
        let input = create_teleport_scenario_with2_courses();

        // Scenario: Class, Empty, Empty, Empty, Class (3 Slot Gap)
        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), // Mon, 8-10
                // Gap (1, 2, 3)
                (0, 4, 0), // Mon, 16-18
            ],
        };

        let penalty = schedule.gap_teleportation_check(&input);
        assert_eq!(penalty, 10, "6 hour gap should be 10 points");
    }

    #[test]
    fn test_gap_8_hours() {
        let input = create_teleport_scenario_with2_courses();

        // Scenario: Class (Start), Empty x4, Class (End)
        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), // Mon, 8-10
                // Gap (1, 2, 3, 4)
                (0, 5, 0), // Mon, 18-20
            ],
        };

        let penalty = schedule.gap_teleportation_check(&input);
        assert_eq!(penalty, 5, "8 hour gap should be 5 points");
    }
    
    #[test]
    fn test_complex_day() {
        let input = create_teleport_scenario_with3_courses();
        
        // Scenario: 
        // 8-10 (Class)
        // 10-12 (Gap 2h -> 20pts)
        // 12-14 (Class)
        // 14-18 (Gap 4h -> 15pts)
        // 18-20 (Class)
        let schedule = Schedule {
            assignments: vec![
                (0, 0, 0), 
                // Gap
                (0, 2, 0),
                // Gap Gap
                (0, 5, 0)
            ],
        };
        // We need 3 courses for this test, so let's mock the input slightly differently
        // or just accept that we are reusing course indices for the sake of the penalty function
        // (The penalty function looks at assignments, not unique course logic, unless you check for duplicates)

        let penalty = schedule.gap_teleportation_check(&input);
        assert_eq!(penalty, 35, "Should sum multiple gaps (20 + 15 = 35)");
    }
}
