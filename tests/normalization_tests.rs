use UCTP::domain::{
    course::Course, group::Group, input_wrapper::TimetableInput, room::Room, teacher::Teacher,
};
use UCTP::io::normalize_input::normalize_data;

#[test]
fn test_normalization_logic() {
    // 1. SETUP: Create data with "Random" Database IDs
    // Notice the IDs are NOT 0, 1, 2. They are 101, 999, 50, etc.
    let raw_input = TimetableInput {
        rooms: vec![], // Rooms don't need normalization yet
        teachers: vec![
            Teacher {
                id: 10,
                name: "Prof. X".to_string(),
                course_id: vec![2002], // Teaches 'Physics' (ID 2002)
            },
        ],
        courses: vec![
            Course {
                id: 1001, // Index 0
                subject_name: "Math".to_string(),
                professor_id: 0,
                group_ids: vec![555, 777], // Attended by Group 555 and 777
                required_hours: 2,
                required_lab: false,
            },
            Course {
                id: 2002, // Index 1
                subject_name: "Physics".to_string(),
                professor_id: 10,
                group_ids: vec![555], // Attended by Group 555
                required_hours: 2,
                required_lab: true,
            },
        ],
        groups: vec![
            Group {
                id: 555, // Index 0
                name: "Group A".to_string(),
                numbers_of_students: 20,
                courses: vec![1001, 2002], // Attends Math (1001) and Physics (2002)
            },
            Group {
                id: 777, // Index 1
                name: "Group B".to_string(),
                numbers_of_students: 30,
                courses: vec![1001], // Attends only Math (1001)
            },
        ],
    };

    // 2. ACT: Run the normalization
    let normalized = normalize_data(raw_input);

    // 3. ASSERT: Check if IDs were converted to Indices

    // --- Check Courses -> Groups Link ---
    // Course 0 (Math, old ID 1001) should point to Group Indices 0 and 1
    let math = &normalized.courses[0];
    assert_eq!(math.subject_name, "Math");
    assert!(math.group_ids.contains(&0), "Math should point to Group Index 0 (was 555)");
    assert!(math.group_ids.contains(&1), "Math should point to Group Index 1 (was 777)");

    // Course 1 (Physics, old ID 2002) should point to Group Index 0
    let physics = &normalized.courses[1];
    assert_eq!(physics.subject_name, "Physics");
    assert_eq!(physics.group_ids, vec![0], "Physics should point to Group Index 0");

    // --- Check Groups -> Courses Link ---
    // Group 0 (Group A, old ID 555) should point to Course Indices 0 and 1
    let group_a = &normalized.groups[0];
    assert_eq!(group_a.name, "Group A");
    assert!(group_a.courses.contains(&0), "Group A should point to Course Index 0 (was 1001)");
    assert!(group_a.courses.contains(&1), "Group A should point to Course Index 1 (was 2002)");

    // Group 1 (Group B, old ID 777) should point to Course Index 0
    let group_b = &normalized.groups[1];
    assert_eq!(group_b.name, "Group B");
    assert_eq!(group_b.courses, vec![0], "Group B should point to Course Index 0");

    // --- Check Teachers -> Courses Link ---
    // Prof X (old ID 10) should point to Course Index 1 (Physics, was 2002)
    let prof = &normalized.teachers[0];
    assert_eq!(prof.course_id, vec![1], "Teacher should point to Course Index 1");
}

#[test]
#[should_panic(expected = "Reference to a non existent ID")]
fn test_normalization_panics_on_bad_id() {
    // This test ensures your code protects you from bad data.
    // If a group points to a course that doesn't exist, it MUST crash.
    
    let bad_input = TimetableInput {
        rooms: vec![],
        teachers: vec![],
        courses: vec![], // Empty course list
        groups: vec![
            Group {
                id: 1,
                name: "Bad Group".to_string(),
                numbers_of_students: 10,
                courses: vec![99999], // <--- This ID does not exist!
            }
        ],
    };

    normalize_data(bad_input); // Should Panic here
}
