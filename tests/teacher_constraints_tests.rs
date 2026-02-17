use UCTP::domain::{
    course::Course,
    input_wrapper::TimetableInput,
    room::Room,
    schedule::Schedule,
    teacher::Teacher,
};

// --- HELPER FUNCTIONS ---

fn create_dummy_room(id: usize, building_id: usize) -> Room {
    Room {
        id,
        name: format!("Room {}", id),
        capacity: 50,
        is_laboratory: false,
        building_id,
        free: vec![], // Assuming this field exists
    }
}

fn create_dummy_course(id: usize, teacher_id: usize) -> Course {
    Course {
        id,
        subject_name: "Test Subject".to_string(),
        professor_id: teacher_id,
        group_ids: vec![1],
        required_hours: 2,
        required_lab: false,
    }
}

fn create_dummy_teacher(id: usize, course_ids: Vec<usize>) -> Teacher {
    Teacher {
        id,
        name: "Prof. Test".to_string(),
        course_id: course_ids,
    }
}

// --- TESTS ---

#[test]
fn test_teacher_double_booking() {
    // SCENARIO: Teacher has 2 classes at Mon 08:00 (Collision)
    
    // 1. Setup: 2 Rooms in same building (location doesn't matter for collision)
    let rooms = vec![
        create_dummy_room(0, 1), // Room 0, Building 1
        create_dummy_room(1, 1), // Room 1, Building 1
    ];
    
    let teacher = create_dummy_teacher(1, vec![0, 1]);
    let courses = vec![
        create_dummy_course(0, 1),
        create_dummy_course(1, 1),
    ];

    let input = TimetableInput {
        rooms,
        teachers: vec![teacher],
        groups: vec![], 
        courses,
    };

    // 2. Schedule: Both at Mon 08:00 (Day 0, Slot 0)
    let assignments = vec![
        (0, 0, 0), // Course 0: Mon 08:00 in Room 0
        (0, 0, 1), // Course 1: Mon 08:00 in Room 1
    ];
    let schedule = Schedule { assignments };

    // 3. Verify: Should trigger heavy penalty
    let penalty = schedule.gap_teleportation_check_teachers(&input);
    assert!(penalty >= 10000, "Teacher double-booking should have massive penalty");
}

#[test]
fn test_teacher_teleportation_penalty() {
    // SCENARIO: Teacher has back-to-back classes in DIFFERENT buildings
    // 08:00-10:00 (Building 1) -> 10:00-12:00 (Building 2)
    
    // 1. Setup: Room 0 (Bldg 1) and Room 1 (Bldg 2)
    let rooms = vec![
        create_dummy_room(0, 1), // Building 1
        create_dummy_room(1, 2), // Building 2 (Far away!)
    ];

    let teacher = create_dummy_teacher(1, vec![0, 1]);
    let courses = vec![
        create_dummy_course(0, 1),
        create_dummy_course(1, 1),
    ];

    let input = TimetableInput {
        rooms,
        teachers: vec![teacher],
        groups: vec![],
        courses,
    };

    // 2. Schedule: Back-to-back
    let assignments = vec![
        (0, 0, 0), // Course 0: Mon 08:00 in Room 0 (Bldg 1)
        (0, 1, 1), // Course 1: Mon 10:00 in Room 1 (Bldg 2)
    ];
    let schedule = Schedule { assignments };

    // 3. Verify: Should trigger teleportation penalty
    let penalty = schedule.gap_teleportation_check_teachers(&input);
    assert!(penalty > 0, "Teacher moving between buildings instantly should be penalized");
}

#[test]
fn test_teacher_safe_schedule() {
    // SCENARIO: Back-to-back classes in SAME building (Safe)
    
    let rooms = vec![
        create_dummy_room(0, 1), // Building 1
        create_dummy_room(1, 1), // Building 1
    ];

    let teacher = create_dummy_teacher(1, vec![0, 1]);
    let courses = vec![
        create_dummy_course(0, 1),
        create_dummy_course(1, 1),
    ];

    let input = TimetableInput {
        rooms,
        teachers: vec![teacher],
        groups: vec![],
        courses,
    };

    // 2. Schedule: Back-to-back in same building
    let assignments = vec![
        (0, 0, 0), // Course 0: Mon 08:00 in Room 0
        (0, 1, 1), // Course 1: Mon 10:00 in Room 1
    ];
    let schedule = Schedule { assignments };

    // 3. Verify: Should be 0 penalty
    let penalty = schedule.gap_teleportation_check_teachers(&input);
    assert_eq!(penalty, 0, "Same building movement should be allowed");
}
