use serde::{Deserialize, Serialize};

/// Represents a physical room in the university.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    /// Unique identifier (Index in the input vector).
    pub id: usize,
    
    /// The label on the door (e.g., "C309", "Amphitheater").
    pub name: String,
    
    /// Maximum number of students allowed in the room.
    pub capacity: u32,
    
    /// If true, this room has special equipment (Computers, Chemistry kits).
    /// Only courses with `required_lab = true` should be scheduled here.
    pub is_laboratory: bool,
    
    /// Tracks identifying ID of the building (e.g., 1 for Main, 2 for FSEGA).
    /// Used to calculate travel penalties.
    pub building_id: usize, 
    
    // (Optional: If you aren't using 'free' yet, you can mark it as such)
    /// Reserved for future optimization (pre-blocked slots).
    pub free: Vec<Vec<u32>>,
}
