pub fn read_json(){
    let file_json = File::open("dummy_data.json").expect("I couldnt open the file, make sure that u placed the file");
    let mut buf_reader = BufReader::new(file_json);

    let input_data: TimetableInput = from_reader(buf_reader).expect("Couldnt read the json");
}
