use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Room { id: usize, name: String, capacity: u32, is_laboratory: bool, building_id: u32, free: Vec<String> }
#[derive(Serialize)]
struct Teacher { id: usize, name: String, course_id: Vec<usize> }
#[derive(Serialize)]
struct Group { id: usize, name: String, numbers_of_students: u32, courses: Vec<usize> }
#[derive(Serialize)]
struct Course { id: usize, subject_name: String, professor_id: usize, group_ids: Vec<usize>, required_hours: u32, required_lab: bool }
#[derive(Serialize)]
struct TimetableInput { rooms: Vec<Room>, teachers: Vec<Teacher>, groups: Vec<Group>, courses: Vec<Course> }

fn main() {
    let mut rooms = Vec::new();
    for i in 1..=2 { rooms.push(Room { id: 100+i, name: format!("Amphitheater {}", i), capacity: 300, is_laboratory: false, building_id: 1, free: vec![] }); }
    for i in 1..=8 { rooms.push(Room { id: 200+i, name: format!("Seminar Room {}", i), capacity: 40, is_laboratory: false, building_id: 1, free: vec![] }); }
    for i in 1..=8 { rooms.push(Room { id: 300+i, name: format!("Laboratory {}", i), capacity: 40, is_laboratory: true, building_id: 2, free: vec![] }); }

    let mut groups = Vec::new();
    for y in 1..=3 {
        for g in 1..=7 {
            groups.push(Group { id: y*10 + g, name: format!("Year {} - G{}", y, g), numbers_of_students: 30, courses: vec![] });
        }
    }

    let mut courses = Vec::new();
    let mut course_id_counter = 1;

    for y in 1..=3 {
        let year_groups: Vec<usize> = (1..=7).map(|g| y * 10 + g).collect();

        for s in 1..=6 {
            let subject_name = format!("Y{} Subj {}", y, s);
            
            let lecture_id = course_id_counter;
            course_id_counter += 1;
            courses.push(Course { id: lecture_id, subject_name: format!("{} (Lecture)", subject_name), professor_id: 1, group_ids: year_groups.clone(), required_hours: 2, required_lab: false });

            for g in &mut groups {
                if g.id / 10 == y { g.courses.push(lecture_id); }
            }

            for g_idx in 1..=7 {
                let g_id = y * 10 + g_idx;

                let sem_id = course_id_counter;
                course_id_counter += 1;
                courses.push(Course { id: sem_id, subject_name: format!("{} (Sem G{})", subject_name, g_idx), professor_id: 2, group_ids: vec![g_id], required_hours: 2, required_lab: false });

                let lab_id = course_id_counter;
                course_id_counter += 1;
                courses.push(Course { id: lab_id, subject_name: format!("{} (Lab G{})", subject_name, g_idx), professor_id: 3, group_ids: vec![g_id], required_hours: 2, required_lab: true });

                for g in &mut groups {
                    if g.id == g_id {
                        g.courses.push(sem_id);
                        g.courses.push(lab_id);
                    }
                }
            }
        }
    }

    let input = TimetableInput { rooms, teachers: vec![], groups, courses };
    
    // Save to JSON
    let json = serde_json::to_string_pretty(&input).unwrap();
    let mut file = File::create("input.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
    println!("🔥 Successfully generated 'hard_data.json' with {} courses and {} groups!", input.courses.len(), input.groups.len());
}
