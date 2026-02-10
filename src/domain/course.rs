use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course{
    pub id: u32,
    pub subject_name: usize,
    pub professor_id: usize,
    pub group_ids: Vec<usize>,
    pub required_hourse: u32,
    pub required_lab: bool,
}
