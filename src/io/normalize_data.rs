use crate::domain::{input_wrapper::TimetableInput};
use std::collections::HashMap;

/// **Pre-processes the data for O(1) access speed.**
///
///This function converts the "Database IDs" (arbitrary numbers like 101, 90210) 
/// inside `course.group_ids` into "Vector Indices" (0, 1, 2...).
///
/// # Arguments
/// * `input` - The raw data loaded from JSON.
///
/// # Returns
/// A normalized `TimetableInput` where `course.group_ids` refers to the 
/// actual index in the `groups` vector.
///
/// # Note
/// The `input.groups[i].id` field is **NOT** changed. We keep the original ID 
/// stored there so we can map the results back to "Real IDs" when generating 
/// the final JSON output.    let mut group_id_to_index = HashMap::new();
pub fn normalize_data(mut input: TimetableInput) ->TimetableInput{
    let mut group_id_to_index = HashMap::new();
    for (index, group) in input.groups.iter().enumerate(){
        group_id_to_index.insert(group.id, index);
    }

    for course in &mut input.courses {
        let mut new_indices = Vec::new();
        for data_base_id in &course.group_ids{
            if let Some(&internal_index) = group_id_to_index.get(data_base_id){
                new_indices.push(internal_index);
            }
            else{
                panic!("Course refers to non existent id: {}", data_base_id);
            }
        }
        course.group_ids = new_indices;
    }
    input
}
