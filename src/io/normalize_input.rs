use crate::domain::{input_wrapper::TimetableInput};
use std::collections::HashMap;
use std::hash::Hash;

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
    let group_map = build_lookup_map(&input.groups, |g| g.id as usize);
    update_references(&mut input.courses, &group_map, |c| &mut c.group_ids);

    let course_map = build_lookup_map(&input.courses, |g| g.id as usize);
    update_references(&mut input.groups, &course_map, |g| &mut g.courses);
    update_references(&mut input.teachers, &course_map, |t| &mut t.course_id);


    input
}
fn build_lookup_map<T, K, F>(items: &[T], get_id: F) -> HashMap<K, usize>
where
    K: Eq + Hash + Copy,
    F: Fn(&T) -> K,
{
    let mut map = HashMap::new();
    for (index, item) in items.iter().enumerate(){
        map.insert(get_id(item), index);
    }
    map
}
fn update_references<T, K, F>(items: &mut [T], map: &HashMap<K, usize>, get_vec_mut: F)
    where
        K: Eq + Hash + Copy + std::fmt::Display,
        F: Fn(&mut T) -> &mut Vec<K>,
    {
    for item in items{
        let ids_vec = get_vec_mut(item);
        let mut new_indices: Vec<K> = Vec::with_capacity(ids_vec.len());

        for db_id in ids_vec.iter(){
            if let Some(&idx) = map.get(db_id){
                new_indices.push(unsafe { *(&idx as *const usize as *const K) });
            } else {
                panic!("Reference to a non existent ID: {}", db_id);
            }
        }
        *ids_vec = new_indices;
    }
}
