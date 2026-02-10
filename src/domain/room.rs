use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room{
    pub id: usize,
    pub name: String,
    pub capacity: u32,
    pub is_laboratory: bool,
    pub free: Vec<Vec<u32>>,
}

