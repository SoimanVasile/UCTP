use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group{
    pub id: usize,
    pub name: String,
    pub numbers_of_students: u32,
    pub courses: Vec<usize>,
}


