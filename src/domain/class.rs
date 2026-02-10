pub struct class{
    id: usize,
    name: String,
    capacity: u32,
    is_laboratory: bool,
    free: Vec<Vec<u32>>,
}

