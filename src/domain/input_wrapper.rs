use crate::domain::{course::Course, teacher::Teacher, room::Room, group::Group};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimetableInput{
    pub rooms: Vec<Room>,
    pub teachers: Vec<Teacher>,
    pub courses: Vec<Course>,
    pub groups: Vec<Group>,
}

