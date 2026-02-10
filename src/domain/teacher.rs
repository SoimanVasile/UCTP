use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teacher{
    pub id: usize,
    pub name: String,
    pub course_id: Vec<usize>,
}
