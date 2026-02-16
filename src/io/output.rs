use crate::domain::{input_wrapper::TimetableInput, schedule::Schedule};

pub fn print_schedule(schedule: &Schedule, input: &TimetableInput) {
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    let slots = [
        "08:00-10:00",
        "10:00-12:00",
        "12:00-14:00",
        "14:00-16:00",
        "16:00-18:00",
        "18:00-20:00",
    ];

    // Column width (fixed)
    let col_width = 22; 

    for group in &input.groups {
        println!("\n");
        println!("╔════════════════════════════════════════════════════════════════════════════════════════════════════════╗");
        println!("║ GROUP: {:<95} ║", format!("{} (ID: {})", group.name, group.id));
        println!("╚════════════════════════════════════════════════════════════════════════════════════════════════════════╝");

        // 1. Header
        print!("{:^15} |", "Time");
        for day in days {
            print!("{:^width$} |", day, width = col_width);
        }
        println!();
        print_separator(col_width);

        // 2. Rows
        for (slot_idx, time_label) in slots.iter().enumerate() {
            // We need to print TWO lines for every time slot:
            // Line A: Course Name
            // Line B: Room Name
            
            let mut line_a_courses = Vec::new(); // Stores course names
            let mut line_b_rooms = Vec::new();   // Stores room names

            for day_idx in 0..5 {
                // Find if there is a course for this Group at this Day/Slot
                let match_course = group.courses.iter().find(|&&cid| {
                    let (d, s, _) = schedule.assignments[cid];
                    d as usize == day_idx && s as usize == slot_idx
                });

                if let Some(&course_id) = match_course {
                    let course = input.get_course(course_id);
                    let (_, _, room_id) = schedule.assignments[course_id];
                    let room = input.get_room(room_id);

                    line_a_courses.push(truncate(&course.subject_name, col_width));
                    line_b_rooms.push(format!("({})", truncate(&room.name, col_width - 2)));
                } else {
                    line_a_courses.push("---".to_string());
                    line_b_rooms.push("".to_string());
                }
            }

            // PRINT LINE A (Subject)
            print!("{:^15} |", time_label);
            for text in &line_a_courses {
                print!("{:^width$} |", text, width = col_width);
            }
            println!();

            // PRINT LINE B (Room) - No time label here
            print!("{:^15} |", ""); 
            for text in &line_b_rooms {
                print!("{:^width$} |", text, width = col_width);
            }
            println!();

            // Divider between time slots
            print_separator(col_width);
        }
    }
}

/// Helper: Safely truncates a string to 'max_len' characters (handling UTF-8)
fn truncate(s: &str, max_len: usize) -> String {
    if s.chars().count() > max_len {
        let mut truncated: String = s.chars().take(max_len - 3).collect();
        truncated.push_str("...");
        truncated
    } else {
        s.to_string()
    }
}

/// Helper: Prints the dashed line separator
fn print_separator(col_width: usize) {
    let total_width = 15 + 3 + (col_width + 3) * 5; // Time col + divider + 5 * (col + divider)
    println!("{:-<1$}", "", total_width);
}
