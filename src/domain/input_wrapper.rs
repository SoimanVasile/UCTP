use crate::domain::{course::Course, teacher::Teacher, room::Room, group::Group};
use serde::{Deserialize, Serialize};
/// The Read-Only "World" data.
/// 
/// This struct holds all the static information loaded from the JSON file.
/// It is passed to the cost function to provide context (Room capacities, Group sizes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimetableInput {
    pub rooms: Vec<Room>,
    pub teachers: Vec<Teacher>,
    pub courses: Vec<Course>,
    pub groups: Vec<Group>,
}

impl TimetableInput {
    /// Helper to get a Course reference by its index in O(1) time.
    /// 
    /// # Panics
    /// Panics if `course_id` is out of bounds (which shouldn't happen if normalized).
    pub fn get_course(&self, course_id: usize) -> &Course {
        &self.courses[course_id]
    }

    /// Helper to get a Room reference by its index in O(1) time.
    pub fn get_room(&self, room_id: usize) -> &Room {
        &self.rooms[room_id]
    }
}
