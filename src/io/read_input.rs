use crate::domain::input_wrapper::TimetableInput;
use std::io::BufReader;
use std::fs::File;

pub fn read_json() -> TimetableInput{
    let file_json = File::open("dummy_data.json").expect("I couldnt open the file, make sure that u placed the file");
    let buf_reader = BufReader::new(file_json);

    let _input_data: TimetableInput = serde_json::from_reader(buf_reader).expect("Couldnt read the json");

    _input_data
}
