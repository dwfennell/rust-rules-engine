use std::{fs::File, io::Write};

use crate::conditions::Condition;

pub fn serialize_condition(condition: &Condition) -> Result<String, serde_json::Error> {
    return serde_json::to_string(condition);
}

pub fn serialize_condition_to_file(
    condition: &Condition,
    file_path: &str,
) -> Option<SerializeToFileError> {
    let serialized_condition;
    match serialize_condition(condition) {
        Ok(serialized) => {
            serialized_condition = serialized;
        }
        Err(err) => return Some(SerializeToFileError::SerializationError(err)),
    }

    let mut file;
    match File::create(&file_path) {
        Ok(created_file) => file = created_file,
        Err(err) => return Some(SerializeToFileError::IOError(err)),
    }

    match file.write_all(serialized_condition.as_bytes()) {
        Ok(_) => return None,
        Err(why) => return Some(SerializeToFileError::IOError(why)),
    }
}

pub enum SerializeToFileError {
    SerializationError(serde_json::Error),
    IOError(std::io::Error),
}

#[cfg(test)]
mod tests {
    use crate::{
        conditions::{
            operators::{AggregationOperator, ComparisonOperator},
            Condition, ConditionGroup, ConditionValue, ReferenceValue, SingleCondition,
        },
        facts::FactValue,
    };

    use super::serialize_condition;

    #[test]
    fn serialization() {
        // TODO: better test.

        let big_condition: Condition = Condition::Group(ConditionGroup {
            conditions: vec![Condition::Single(SingleCondition {
                operator: ComparisonOperator::Equals,
                values: (
                    ConditionValue::Json(ReferenceValue {
                        address: "one".to_string(),
                    }),
                    ConditionValue::Fixed(FactValue::String("str_value".to_string())),
                ),
            })],
            operator: AggregationOperator::Any,
        });

        let serialized = serialize_condition(&big_condition).unwrap();
        assert_eq!(serialized, "{\"Group\":{\"operator\":\"Any\",\"conditions\":[{\"Single\":{\"operator\":\"Equals\",\"values\":[{\"Json\":{\"address\":\"one\"}},{\"Fixed\":{\"String\":\"str_value\"}}]}}]}}");
    }
}
