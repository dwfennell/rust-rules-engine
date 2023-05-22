use std::collections::HashMap;

use crate::facts::NumberFact;
use serde_json::{self, Value as JsonValue};

use super::{FactValue, GetFact};

pub struct JsonFacts {
    fact_object: JsonValue,
}

impl JsonFacts {
    pub fn new(facts: JsonValue) -> JsonFacts {
        JsonFacts { fact_object: facts }
    }
}

impl GetFact for JsonFacts {
    fn get_fact(
        &self,
        fact_address: &str,
        array_context: &HashMap<&str, i64>,
    ) -> Option<FactValue> {
        let mut value: &JsonValue = &self.fact_object;

        let hydrated_address = hydrate_address_collection_indicies(fact_address, array_context);
        for path_component in hydrated_address.split('.') {
            match value {
                JsonValue::Object(obj_value) => {
                    let possible_value = obj_value.get(path_component);
                    if possible_value.is_none() {
                        return None;
                    }

                    value = &possible_value.unwrap();
                }
                JsonValue::Array(array_value) => {
                    let parsed_index = path_component.parse::<usize>();
                    if parsed_index.is_err() {
                        return None;
                    }
                    let index = parsed_index.unwrap();

                    let value_at_index = array_value.get(index);
                    if value_at_index.is_none() {
                        return None;
                    }
                    value = value_at_index.unwrap();
                }
                _ => return None,
            }
        }

        match value {
            JsonValue::Bool(bool_value) => return Some(FactValue::Boolean(*bool_value)),
            JsonValue::String(string_value) => {
                return Some(FactValue::String(string_value.to_string()))
            }
            JsonValue::Number(number_value) => {
                if number_value.is_f64() {
                    let number_fact = NumberFact::Float(number_value.as_f64().unwrap());
                    return Some(FactValue::Number(number_fact));
                } else if number_value.is_i64() {
                    let number_fact = NumberFact::Integer(number_value.as_i64().unwrap());
                    return Some(FactValue::Number(number_fact));
                } else if number_value.is_u64() {
                    let i64_value = number_value.as_i64();
                    if i64_value.is_none() {
                        return None;
                    }

                    let number_fact = NumberFact::Integer(i64_value.unwrap());
                    return Some(FactValue::Number(number_fact));
                }

                panic!("Should be unreachable (value is {number_value})")
            }
            _ => return None,
        }
    }

    fn get_array_len(&self, array_address: &str, array_context: &HashMap<&str, i64>) -> i64 {
        let mut value: &JsonValue = &self.fact_object;

        let hydrated_address = hydrate_address_collection_indicies(array_address, array_context);

        for path_component in hydrated_address.split('.') {
            match value {
                JsonValue::Object(obj_value) => {
                    let possible_value = obj_value.get(path_component);
                    if possible_value.is_none() {
                        return 0;
                    }

                    value = &possible_value.unwrap();
                }
                JsonValue::Array(array_value) => {
                    let parsed_index = path_component.parse::<usize>();
                    if parsed_index.is_err() {
                        return 0;
                    }
                    let index = parsed_index.unwrap();

                    let value_at_index = array_value.get(index);
                    if value_at_index.is_none() {
                        return 0;
                    }
                    value = value_at_index.unwrap();
                }
                _ => return 0,
            }
        }

        match value {
            JsonValue::Array(array_value) => return array_value.len() as i64,
            _ => return 0,
        }
    }
}

/// Generate an address string with array indicies inserted.
fn hydrate_address_collection_indicies<'a>(
    address: &str,
    array_context: &HashMap<&str, i64>,
) -> String {
    if array_context.is_empty() {
        return address.to_string();
    }

    let mut hydrated_address = address.to_string();
    for context in array_context.iter() {
        let to_replace = format!("{}[]", context.0);
        let replacement = format!("{}.{}", context.0, context.1);

        hydrated_address = hydrated_address.replace(&to_replace, &replacement);
    }

    return hydrated_address.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_fact() {
        let data = serde_json::json!({ "one": 1, "two_point_one": 2.1, });

        let array_context: HashMap<&str, i64> = HashMap::new();
        let facts = JsonFacts::new(data);
        let fact_one = facts.get_fact("one", &array_context);

        assert_eq!(fact_one.unwrap(), FactValue::Number(NumberFact::Integer(1)));
    }

    #[test]
    fn missing_fact_return_none() {
        let data = serde_json::json!({ "one": 1, "two_point_one": 2.1, });

        let facts = JsonFacts::new(data);
        let array_context: HashMap<&str, i64> = HashMap::new();

        let does_not_exist = facts.get_fact("no", &array_context);
        assert!(does_not_exist.is_none());
    }

    #[test]
    fn get_nested_fact() {
        let data = serde_json::json!({
            "numbers": {
                "one": 1,
                "two_point_one": 2.1,
            }
        });

        let facts = JsonFacts::new(data);
        let array_context: HashMap<&str, i64> = HashMap::new();

        let fact_one = facts.get_fact("numbers.one", &array_context);
        assert_eq!(fact_one.unwrap(), FactValue::Number(NumberFact::Integer(1)));
    }

    #[test]
    fn get_fact_from_array() {
        let data = serde_json::json!({
            "people": [
                { "age": 25 },
                { "age": 33 }
            ]
        });
        let facts = JsonFacts::new(data);

        let mut array_context: HashMap<&str, i64> = HashMap::new();
        array_context.insert("people", 1);

        let fact_one = facts.get_fact("people[].age", &array_context);
        assert_eq!(
            fact_one.unwrap(),
            FactValue::Number(NumberFact::Integer(33))
        );
    }

    #[test]
    fn get_fact_from_nested_array() {
        let data = serde_json::json!({
            "people": [
                { "age": 45, "children": [{ "child_age": 10 }, { "child_age": 12 }] },
                { "age": 33, "children": [{ "child_age": 5 }, { "child_age": 7 }] }
            ]
        });
        let facts = JsonFacts::new(data);

        let mut array_context: HashMap<&str, i64> = HashMap::new();
        array_context.insert("people", 0);
        array_context.insert("children", 1);

        let fact_one = facts.get_fact("people[].children[].child_age", &array_context);
        assert_eq!(
            fact_one.unwrap_or(FactValue::Number(NumberFact::Integer(0))),
            FactValue::Number(NumberFact::Integer(12))
        );
    }

    #[test]
    fn get_none_fact_from_array() {
        let data = serde_json::json!({
            "people": [
            ]
        });
        let facts = JsonFacts::new(data);

        let mut array_context: HashMap<&str, i64> = HashMap::new();
        array_context.insert("people", 1);

        let fact_one = facts.get_fact("people[].age", &array_context);
        assert!(fact_one.is_none());
    }
}
