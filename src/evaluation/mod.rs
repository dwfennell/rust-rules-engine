use std::collections::HashMap;

mod evaluate_function;

use crate::{
    conditions::{
        operators::{AggregationOperator, ComparisonOperator},
        ArrayCondition, Condition, ConditionGroup, ConditionValue, SingleCondition,
    },
    facts::{self, FactValue, GetFact},
};

use self::evaluate_function::evaluate_fn;

pub fn evaluate(condition: &Condition, facts: &impl facts::GetFact) -> bool {
    let array_context: HashMap<&str, i64> = HashMap::new();
    return evaluate_with_array_context(condition, facts, &array_context);
}

pub fn evaluate_with_array_context(
    condition: &Condition,
    facts: &impl facts::GetFact,
    array_context: &HashMap<&str, i64>,
) -> bool {
    match condition {
        Condition::Single(condition) => evaluate_single_condition(condition, facts, array_context),
        Condition::Group(condition_group) => {
            evaluate_condition_group(condition_group, facts, array_context)
        }
        Condition::Array(condition) => evaluate_array_condition(condition, facts, array_context),
    }
}

fn evaluate_single_condition(
    condition: &SingleCondition,
    facts: &impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> bool {
    let a = get_fact_from_condition_value(&condition.values.0, facts, &array_context);
    let b = get_fact_from_condition_value(&condition.values.1, facts, &array_context);

    match condition.operator {
        ComparisonOperator::Equals => a == b,
        ComparisonOperator::GreaterThan => a > b,
        ComparisonOperator::LessThan => a < b,
        ComparisonOperator::GreaterThanEqualTo => a >= b,
        ComparisonOperator::LessThanEqualTo => a <= b,
    }
}

fn evaluate_condition_group(
    condition_group: &ConditionGroup,
    facts: &impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> bool {
    let mut condition_group_iter = condition_group.conditions.iter();

    match condition_group.operator {
        AggregationOperator::Any => condition_group_iter
            .any(|condition| evaluate_with_array_context(&condition, facts, array_context)),
        AggregationOperator::Every => condition_group_iter
            .all(|condition| evaluate_with_array_context(&condition, facts, array_context)),
        AggregationOperator::None => condition_group_iter
            .all(|condition| !evaluate_with_array_context(&condition, facts, array_context)),
    }
}

fn evaluate_array_condition<'a>(
    condition: &'a ArrayCondition,
    facts: &impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> bool {
    let array_len = facts.get_array_len(&condition.array.address, array_context);

    for i in 0..array_len {
        let mut item_context: HashMap<&str, i64> = HashMap::new();
        item_context.insert(&condition.array.address, i);
        item_context.extend(array_context);

        let item_result = evaluate_with_array_context(&condition.condition, facts, &item_context);

        match condition.operator {
            AggregationOperator::Every => {
                if !item_result {
                    return false;
                }
            }
            AggregationOperator::Any => {
                if item_result {
                    return true;
                }
            }
            AggregationOperator::None => {
                if item_result {
                    return false;
                }
            }
        }
    }

    match condition.operator {
        AggregationOperator::Every => true,
        AggregationOperator::Any => false,
        AggregationOperator::None => true,
    }
}

fn get_fact_from_condition_value<'a>(
    condition_value: &ConditionValue,
    facts: &'a impl GetFact,
    array_context: &HashMap<&str, i64>,
) -> Option<FactValue> {
    match condition_value {
        ConditionValue::Fixed(value) => Some(value.to_owned()),
        ConditionValue::Json(reference) => facts.get_fact(&reference.address, array_context),
        ConditionValue::Transform(function) => evaluate_fn(function, facts, array_context),
    }
}
