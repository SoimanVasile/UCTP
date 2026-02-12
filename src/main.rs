use UCTP::domain::{course::Course, teacher::Teacher, group::Group, room::Room, input_wrapper::TimetableInput};
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file_json = File::open("dummy_data.json").expect("I couldnt open the file, make sure that u placed the file");
    let buf_reader = BufReader::new(file_json);

    let input_data: TimetableInput = from_reader(buf_reader).expect("Couldnt read the json");

    println!("I could succesfully read the input");
    println!("{:?}", input_data);
}
