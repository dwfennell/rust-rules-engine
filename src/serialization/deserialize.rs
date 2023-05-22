use std::{fs::File, io::Read, path::Path};

use crate::conditions::Condition;

pub fn deserialize_condition(serialized: &str) -> Result<Condition, serde_json::Error> {
    let result: Result<Condition, serde_json::Error> = serde_json::from_str(serialized);
    return result;
}

pub fn deserialize_condition_from_file(
    file_path: &str,
) -> Result<Condition, DeserializeFromFileError> {
    let path = Path::new(file_path);

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(err) => return Err(DeserializeFromFileError::IOError(err)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => return Err(DeserializeFromFileError::IOError(err)),
        Ok(_) => {}
    };

    match deserialize_condition(&s) {
        Err(err) => return Err(DeserializeFromFileError::SerializationError(err)),
        Ok(condition) => return Ok(condition),
    };
}

pub enum DeserializeFromFileError {
    SerializationError(serde_json::Error),
    IOError(std::io::Error),
}

#[test]
fn deserialize() {
    let serialized = "{\"Group\":{\"operator\":\"Any\",\"conditions\":[{\"Single\":{\"operator\":\"Equals\",\"values\":[{\"Json\":{\"address\":\"one\"}},{\"Fixed\":{\"String\":\"str_value\"}}]}}]}}";

    let result = deserialize_condition(serialized);
    let deserialized_condition = result.unwrap();

    assert!(match deserialized_condition {
        Condition::Group(_) => true,
        _ => false,
    })
}
