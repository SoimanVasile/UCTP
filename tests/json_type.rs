#[cfg(test)]
mod tests {
    use UCTP::domain::input_wrapper::TimetableInput;

    #[test]
    fn test_json_contract_compatibility() {
        let incoming_json = r#"
        {
            "rooms": [
                {
                    "id": 1,
                    "name": "C309",
                    "capacity": 100,
                    "is_laboratory": false,
                    "free": []
                }
            ],
            "teachers": [
                {
                    "id": 10,
                    "name": "Prof. Popescu",
                    "course_id": [101]
                }
            ],
            "courses": [
                {
                    "id": 101,
                    "subject_name": "Sisteme de Operare",
                    "professor_id": 10,
                    "group_ids": [1],
                    "required_hours": 2,
                    "required_lab": true
                }
            ],
            "groups": [
                {
                    "id": 1,
                    "name": "Grupa 911",
                    "numbers_of_students": 25,
                    "courses": [101]
                }
            ]
        }
        "#;

        let result: Result<TimetableInput, serde_json::Error> = serde_json::from_str(incoming_json);

        match result {
            Ok(data) => {
                assert_eq!(data.rooms.len(), 1);
                assert_eq!(data.rooms[0].name, "C309");
                assert_eq!(data.teachers.len(), 1);
            },
            Err(e) => {
                panic!("❌ TEST FAILED: The rust structure is not the same with the json: {}", e);
            }
        }
    }
}
