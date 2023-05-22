use std::collections::HashMap;

use serde_json;
use wasm_bindgen::prelude::wasm_bindgen;
extern crate console_error_panic_hook;

use crate::{
    conditions::Condition, evaluation::evaluate_with_array_context, facts::json_facts::JsonFacts,
    serialization::deserialize::deserialize_condition,
};

// A very inefficient (because facts and the condition must be deserialized) condition evaluation function that can be compiled into webassembly
#[wasm_bindgen]
pub fn evaluate_condition(condition_serialized: &str, facts_serialized: &str) -> bool {
    console_error_panic_hook::set_once();

    let condition_result = deserialize_condition(condition_serialized);
    if condition_result.is_err() {
        panic!("Could not parse condition: {:?}", condition_result.err());
    }

    let facts_result = serde_json::from_str(facts_serialized);
    if facts_result.is_err() {
        panic!("could not parse facts: {:?}", facts_result.err())
    }

    let condition: Condition = condition_result.unwrap();
    let facts = facts_result.unwrap();

    let array_context: HashMap<&str, i64> = HashMap::new();
    return evaluate_with_array_context(&condition, &JsonFacts::new(facts), &array_context);
}
