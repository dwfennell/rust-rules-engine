use std::collections::HashMap;

use super::get_fact_from_condition_value;
use crate::conditions::{functions::Fn, ConditionValue};
use crate::facts::{FactValue, GetFact};

pub fn evaluate_fn<'a>(
    function: &Fn,
    facts: &'a impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> Option<FactValue> {
    match function {
        Fn::Min(values) => evaluate_min(values, facts, array_context),
        Fn::Max(values) => evaluate_max(values, facts, array_context),

        Fn::And(values) => evaluate_and(values, facts, array_context),
        Fn::Or(values) => evaluate_or(values, facts, array_context),
        Fn::Not(value) => evaluate_not(value, facts, array_context),

        Fn::GreaterThan(values) => evaluate_greater_than(values, facts, array_context),
        Fn::LessThan(values) => evaluate_less_than(values, facts, array_context),
        Fn::Equal(values) => evaluate_equal(values, facts, array_context),
    }
}

fn evaluate_min<'a>(
    values: &Vec<ConditionValue>,
    facts: &'a impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> Option<FactValue> {
    let mut min_value: Option<FactValue> = None;

    for value in values {
        let fact = get_fact_from_condition_value(value, facts, array_context);
        if fact.is_some() && (min_value.is_none() || fact < min_value) {
            min_value = fact;
        }
    }

    return min_value;
}

fn evaluate_max<'a>(
    values: &Vec<ConditionValue>,
    facts: &'a impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> Option<FactValue> {
    let mut max_value: Option<FactValue> = None;

    for value in values {
        let fact = get_fact_from_condition_value(value, facts, array_context);
        if fact.is_some() && (max_value.is_none() || fact > max_value) {
            max_value = fact;
        }
    }

    return max_value;
}

fn evaluate_and<'a>(
    values: &Vec<ConditionValue>,
    facts: &'a impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> Option<FactValue> {
    for value in values {
        let fact = get_fact_from_condition_value(value, facts, array_context);

        match fact.unwrap_or(FactValue::Boolean(false)) {
            FactValue::Boolean(value) => {
                if !value {
                    return Some(FactValue::Boolean(false));
                }
            }
            _ => return Some(FactValue::Boolean(false)),
        }
    }

    return Some(FactValue::Boolean(true));
}

fn evaluate_or<'a>(
    values: &Vec<ConditionValue>,
    facts: &'a impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> Option<FactValue> {
    for value in values {
        let fact = get_fact_from_condition_value(value, facts, array_context);

        match fact.unwrap_or(FactValue::Boolean(false)) {
            FactValue::Boolean(value) => {
                if value {
                    return Some(FactValue::Boolean(true));
                }
            }
            _ => {}
        }
    }

    return Some(FactValue::Boolean(false));
}

fn evaluate_not<'a>(
    value: &Box<ConditionValue>,
    facts: &'a impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> Option<FactValue> {
    let fact = get_fact_from_condition_value(value, facts, array_context);

    if fact.is_none() {
        return None;
    }

    match fact.unwrap() {
        FactValue::Boolean(value) => Some(FactValue::Boolean(!value)),
        _ => return None,
    }
}

fn evaluate_greater_than<'a>(
    values: &Vec<ConditionValue>,
    facts: &'a impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> Option<FactValue> {
    if values.len() < 2 {
        return None;
    }

    let first_fact = get_fact_from_condition_value(&values[0], facts, array_context);

    // Slightly inefficient since we compare the first value to itself.
    for value in values {
        let fact = get_fact_from_condition_value(value, facts, array_context);
        if first_fact < fact {
            return Some(FactValue::Boolean(false));
        }
    }

    return Some(FactValue::Boolean(true));
}

fn evaluate_less_than<'a>(
    values: &Vec<ConditionValue>,
    facts: &'a impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> Option<FactValue> {
    if values.len() < 2 {
        return None;
    }

    let first_fact = get_fact_from_condition_value(&values[0], facts, array_context);

    // Slightly inefficient since we compare the first value to itself.
    for value in values {
        let fact = get_fact_from_condition_value(value, facts, array_context);
        if first_fact > fact {
            return Some(FactValue::Boolean(false));
        }
    }

    return Some(FactValue::Boolean(true));
}

fn evaluate_equal<'a>(
    values: &Vec<ConditionValue>,
    facts: &'a impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> Option<FactValue> {
    if values.len() < 2 {
        return None;
    }

    let first_fact = get_fact_from_condition_value(&values[0], facts, array_context);

    // Slightly inefficient since we compare the first value to itself.
    for value in values {
        let fact = get_fact_from_condition_value(value, facts, array_context);
        if first_fact != fact {
            return Some(FactValue::Boolean(false));
        }
    }

    return Some(FactValue::Boolean(true));
}

// fn evaluate_<'a>(
//     values: &Vec<ConditionValue>,
//     facts: &'a impl GetFact,
//     array_context: &HashMap<&str, i64>,
// ) -> Option<FactValue> {
// }
