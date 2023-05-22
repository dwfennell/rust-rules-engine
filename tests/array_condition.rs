use engine::{
    conditions::{
        operators::{AggregationOperator, ComparisonOperator},
        ArrayCondition, Condition, ConditionValue, ReferenceValue, SingleCondition,
    },
    evaluation,
    facts::{json_facts::JsonFacts, FactValue, NumberFact},
};

#[test]
fn array_condition_every_true() {
    let facts = JsonFacts::new(serde_json::json!({
        "people": [
            { "age": 35, },
            { "age": 25, },
        ]
    }));

    let single_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "people[].age".to_string(),
            }),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(40))),
        ),
    });

    let array_condition = Condition::Array(ArrayCondition {
        condition: Box::new(single_condition),
        operator: AggregationOperator::Every,
        array: ReferenceValue {
            address: "people".to_string(),
        },
    });

    let result = evaluation::evaluate(&array_condition, &facts);
    assert!(result)
}

#[test]
fn array_condition_every_false() {
    let facts = JsonFacts::new(serde_json::json!({
        "people": [
            { "age": 35, },
            { "age": 25, },
        ]
    }));

    let single_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "people[].age".to_string(),
            }),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(30))),
        ),
    });

    let array_condition = Condition::Array(ArrayCondition {
        condition: Box::new(single_condition),
        operator: AggregationOperator::Every,
        array: ReferenceValue {
            address: "people".to_string(),
        },
    });

    let result = evaluation::evaluate(&array_condition, &facts);
    assert!(!result)
}

#[test]
fn array_condition_none_true() {
    let facts = JsonFacts::new(serde_json::json!({
        "people": [
            { "age": 35, },
            { "age": 25, },
        ]
    }));

    let single_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "people[].age".to_string(),
            }),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(20))),
        ),
    });

    let array_condition = Condition::Array(ArrayCondition {
        condition: Box::new(single_condition),
        operator: AggregationOperator::None,
        array: ReferenceValue {
            address: "people".to_string(),
        },
    });

    let result = evaluation::evaluate(&array_condition, &facts);
    assert!(result)
}

#[test]
fn array_condition_none_false() {
    let facts = JsonFacts::new(serde_json::json!({
        "people": [
            { "age": 35, },
            { "age": 25, },
        ]
    }));

    let single_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "people[].age".to_string(),
            }),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(40))),
        ),
    });

    let array_condition = Condition::Array(ArrayCondition {
        condition: Box::new(single_condition),
        operator: AggregationOperator::None,
        array: ReferenceValue {
            address: "people".to_string(),
        },
    });

    let result = evaluation::evaluate(&array_condition, &facts);
    assert!(!result)
}

#[test]
fn array_condition_any_true() {
    let facts = JsonFacts::new(serde_json::json!({
        "people": [
            { "age": 35, },
            { "age": 25, },
        ]
    }));

    let single_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "people[].age".to_string(),
            }),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(30))),
        ),
    });

    let array_condition = Condition::Array(ArrayCondition {
        condition: Box::new(single_condition),
        operator: AggregationOperator::Any,
        array: ReferenceValue {
            address: "people".to_string(),
        },
    });

    let result = evaluation::evaluate(&array_condition, &facts);
    assert!(result)
}

#[test]
fn array_condition_any_false() {
    let facts = JsonFacts::new(serde_json::json!({
        "people": [
            { "age": 35, },
            { "age": 25, },
        ]
    }));

    let single_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "people[].age".to_string(),
            }),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(20))),
        ),
    });

    let array_condition = Condition::Array(ArrayCondition {
        condition: Box::new(single_condition),
        operator: AggregationOperator::Any,
        array: ReferenceValue {
            address: "people".to_string(),
        },
    });

    let result = evaluation::evaluate(&array_condition, &facts);
    assert!(!result)
}

#[test]
fn nested_condition_true() {
    let facts = JsonFacts::new(serde_json::json!({
        "people": [
            { "children": [{ "age": 10 }, { "age": 5 }] },
            { "children": [{ "age": 7 }, { "age": 13 }] },
        ]
    }));

    let under_12_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "people[].children[].age".to_string(),
            }),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(12))),
        ),
    });

    let children_array_condition = Condition::Array(ArrayCondition {
        condition: Box::new(under_12_condition),
        operator: AggregationOperator::Any,
        array: ReferenceValue {
            address: "people[].children".to_string(),
        },
    });

    // Every person has a child under 12.
    let people_array_condition = Condition::Array(ArrayCondition {
        condition: Box::new(children_array_condition),
        operator: AggregationOperator::Every,
        array: ReferenceValue {
            address: "people".to_string(),
        },
    });

    let result = evaluation::evaluate(&people_array_condition, &facts);
    assert!(result)
}

#[test]
fn nested_condition_false() {
    let facts = JsonFacts::new(serde_json::json!({
        "people": [
            { "children": [{ "age": 10 }, { "age": 5 }] },
            { "children": [{ "age": 7 }, { "age": 13 }] },
        ]
    }));

    let under_12_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "people[].children[].age".to_string(),
            }),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(12))),
        ),
    });

    let children_array_condition = Condition::Array(ArrayCondition {
        condition: Box::new(under_12_condition),
        operator: AggregationOperator::None,
        array: ReferenceValue {
            address: "people[].children".to_string(),
        },
    });

    // Every person does not have a child under 12.
    let people_array_condition = Condition::Array(ArrayCondition {
        condition: Box::new(children_array_condition),
        operator: AggregationOperator::Every,
        array: ReferenceValue {
            address: "people".to_string(),
        },
    });

    let result = evaluation::evaluate(&people_array_condition, &facts);
    assert!(!result)
}

#[test]
fn array_with_scalar_values() {
    let facts = JsonFacts::new(serde_json::json!({
        "numbers": [5, 10, 15]
    }));

    let is_ten = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "numbers[]".to_string(),
            }),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(10))),
        ),
    });

    let any_number_is_ten = Condition::Array(ArrayCondition {
        condition: Box::new(is_ten),
        operator: AggregationOperator::Any,
        array: ReferenceValue {
            address: "numbers".to_string(),
        },
    });

    let result = evaluation::evaluate(&any_number_is_ten, &facts);
    assert!(result)
}
