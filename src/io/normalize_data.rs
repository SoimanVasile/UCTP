use crate::domain::{input_wrapper::TimetableInput};
use std::collections::HashMap;

pub fn normalize_data(mut input: TimetableInput) ->TimetableInput{
    //This function gets the ids of the groups and normalize the data so the id equals to the index
    //they are in the vector of groups
    //parem: input - TimetableInput
    //return: Changed input where the id of the groups in the courses will be the index where they
    //are in the  vector of groups
    //The ids of the input.groups[i].id isnt changed so we can revert back to the original ids
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
