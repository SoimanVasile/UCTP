use crate::domain::input_wrapper::TimetableInput;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;

pub fn read_json(file_name: &String) -> Result<TimetableInput, Box<dyn Error>>{
    let file_json = File::open(file_name).expect("I couldnt open the file, make sure that u placed the file");
    let buf_reader = BufReader::new(file_json);

    let input_data: TimetableInput = serde_json::from_reader(buf_reader)?;

    Ok(input_data)
}
