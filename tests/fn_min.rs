use engine::{
    conditions::{
        functions::Fn, operators::ComparisonOperator, Condition, ConditionValue, ReferenceValue,
        SingleCondition,
    },
    evaluation,
    facts::{json_facts::JsonFacts, FactValue, NumberFact},
};

#[test]
fn min_from_fixed_true() {
    let facts = JsonFacts::new(serde_json::json!({}));

    let min_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Min(vec![
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(2))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
        ),
    });
    let result = evaluation::evaluate(&min_condition, &facts);
    assert!(result);

    let min_same_value_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Min(vec![
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
        ),
    });
    let result = evaluation::evaluate(&min_same_value_condition, &facts);
    assert!(result);
}

#[test]
fn min_with_value_not_found() {
    let facts = JsonFacts::new(serde_json::json!({}));

    let min_with_value_not_found_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Min(vec![
                ConditionValue::Json(ReferenceValue {
                    address: "does_not_exist".to_string(),
                }),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
        ),
    });
    let result = evaluation::evaluate(&min_with_value_not_found_condition, &facts);
    assert!(result);
}

#[test]
fn min_from_fixed_false() {
    let facts = JsonFacts::new(serde_json::json!({}));

    let single_condition = SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Min(vec![
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(3))),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(2))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(!result)
}

#[test]
fn min_from_json_true() {
    let facts = JsonFacts::new(serde_json::json!({
        "one": 1
    }));

    let single_condition = SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Min(vec![
                ConditionValue::Json(ReferenceValue {
                    address: "one".to_string(),
                }),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(2))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result)
}

#[test]
fn nested_min() {
    let facts = JsonFacts::new(serde_json::json!({}));

    let single_condition = SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Min(vec![
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(3))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result)
}
