use engine::{
    conditions::{
        operators::ComparisonOperator, Condition, ConditionValue, ReferenceValue, SingleCondition,
    },
    evaluation,
    facts::{json_facts::JsonFacts, FactValue, NumberFact},
};

#[test]
fn single_condition_equals_true() {
    let facts = build_facts();
    let single_condition = SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result)
}

#[test]
fn single_condition_equals_false() {
    let facts = build_facts();
    let single_condition = SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "two_point_one".to_string(),
            }),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(!result)
}

#[test]
fn single_condition_greater_than_true() {
    let facts = build_facts();
    let single_condition = SingleCondition {
        operator: ComparisonOperator::GreaterThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "two_point_one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result)
}

#[test]
fn single_condition_greater_than_false() {
    let facts = build_facts();
    let single_condition = SingleCondition {
        operator: ComparisonOperator::GreaterThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "two_point_one".to_string(),
            }),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(!result)
}

#[test]
fn single_condition_less_than_true() {
    let facts = build_facts();
    let single_condition = SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "two_point_one".to_string(),
            }),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result)
}

#[test]
fn single_condition_less_than_false() {
    let facts = build_facts();
    let single_condition = SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "two_point_one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(!result)
}

#[test]
fn single_condition_fixed_and_json_data_comparison() {
    let facts = build_facts();

    let single_condition = SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result)
}

#[test]
fn float_and_integer_equivalence() {
    let facts = build_facts();

    let single_condition = SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Float(1.0))),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result)
}

fn build_facts() -> JsonFacts {
    let data = serde_json::json!({
        "one".to_string(): 1,
        "two_point_one".to_string(): 2.1,
    });

    JsonFacts::new(data)
}
