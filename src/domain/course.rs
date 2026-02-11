use serde::{Deserialize, Serialize};
use crate::domain::group::Group;

/// Represents a University Course (Subject) that needs to be scheduled.
/// 
/// This struct holds the static data about a course, such as who teaches it
/// and which student groups must attend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    /// Unique identifier for the course (Database ID).
    pub id: u32,
    
    /// The name of the subject (e.g., "Operating Systems").
    pub subject_name: String,
    
    /// The ID of the professor teaching this course.
    pub professor_id: usize,
    
    /// List of Group IDs that attend this course together.
    /// If multiple groups are listed, they are merged into a single large class.
    pub group_ids: Vec<usize>,
    
    /// Number of 2-hour slots required per week (usually 1).
    pub required_hours: u32,
    
    /// If true, this course requires a room with `is_laboratory = true`.
    pub required_lab: bool,
}

impl Course {
    /// Calculates the total number of students attending this course.
    ///
    /// It sums up the `numbers_of_students` from all groups listed in `group_ids`.
    /// 
    /// # Arguments
    /// * `vec_of_groups` - The full list of groups to look up student counts.
    pub fn capacity_needed(&self, vec_of_groups: &Vec<Group>) -> u32 {
        let mut capacity_needed: u32 = 0;
        for &group_id in &self.group_ids {
            capacity_needed += vec_of_groups[group_id].numbers_of_students;
        }
        capacity_needed
    }
}
