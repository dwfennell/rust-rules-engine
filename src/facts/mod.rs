use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod json_facts;

pub trait GetFact {
    fn get_fact(&self, fact_address: &str, array_context: &HashMap<&str, i64>)
        -> Option<FactValue>;
    fn get_array_len(&self, address: &str, array_context: &HashMap<&str, i64>) -> i64;
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum FactValue {
    Number(NumberFact),
    // Date(chrono::DateTime),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NumberFact {
    Integer(i64),
    Float(f64),
}

impl PartialEq for NumberFact {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NumberFact::Integer(a), NumberFact::Integer(b)) => a.eq(b),
            (NumberFact::Float(a), NumberFact::Float(b)) => a.eq(b),
            (NumberFact::Integer(a), NumberFact::Float(b)) => {
                let a_as_float = *a as f64;
                return a_as_float.eq(b);
            }
            (NumberFact::Float(a), NumberFact::Integer(b)) => {
                let b_as_float = *b as f64;
                return a.eq(&b_as_float);
            }
        }
    }
}

impl PartialOrd for NumberFact {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (NumberFact::Integer(a), NumberFact::Integer(b)) => a.partial_cmp(b),
            (NumberFact::Float(a), NumberFact::Float(b)) => a.partial_cmp(b),
            (NumberFact::Integer(a), NumberFact::Float(b)) => {
                let a_as_float = *a as f64;
                return a_as_float.partial_cmp(b);
            }
            (NumberFact::Float(a), NumberFact::Integer(b)) => {
                let b_as_float = *b as f64;
                return a.partial_cmp(&b_as_float);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_int_partial_eq() {
        assert_eq!(NumberFact::Integer(1), NumberFact::Float(1.0));
        assert_eq!(NumberFact::Float(1.0), NumberFact::Integer(1));
        assert_eq!(NumberFact::Integer(1), NumberFact::Integer(1));
        assert_eq!(NumberFact::Float(1.0), NumberFact::Float(1.0));
    }

    #[test]
    fn float_int_partial_order() {
        assert!(NumberFact::Integer(2) > NumberFact::Float(1.5));
        assert!(NumberFact::Float(1.5) < NumberFact::Integer(2));
        assert!(NumberFact::Float(1.0) <= NumberFact::Integer(1));
    }
}
