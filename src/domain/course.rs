use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course{
    pub id: u32,
    pub subject_name: String,
    pub professor_id: usize,
    pub group_ids: Vec<usize>,
    pub required_hours: u32,
    pub required_lab: bool,
}

use crate::domain::group::Group;
impl Course{
    ///Calculates the capacity needed for this course
    pub fn capacity_needed(&self, vec_of_groups: &Vec<Group>) -> u32{
        let mut capacity_needed: u32 = 0;
        for &group_id in &self.group_ids{
            capacity_needed += vec_of_groups[group_id].numbers_of_students;
        }
        capacity_needed
    }
}
