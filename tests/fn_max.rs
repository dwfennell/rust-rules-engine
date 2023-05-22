use engine::{
    conditions::{
        functions::Fn, operators::ComparisonOperator, Condition, ConditionValue, ReferenceValue,
        SingleCondition,
    },
    evaluation,
    facts::{json_facts::JsonFacts, FactValue, NumberFact},
};

#[test]
fn max_from_fixed_true() {
    let facts = JsonFacts::new(serde_json::json!({}));

    let max_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Max(vec![
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(2))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(2))),
        ),
    });
    let result = evaluation::evaluate(&max_condition, &facts);
    assert!(result);

    let max_same_value_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Max(vec![
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
        ),
    });
    let result = evaluation::evaluate(&max_same_value_condition, &facts);
    assert!(result);
}

#[test]
fn max_with_value_not_found() {
    let facts = JsonFacts::new(serde_json::json!({}));

    let max_with_value_not_found_condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Max(vec![
                ConditionValue::Json(ReferenceValue {
                    address: "does_not_exist".to_string(),
                }),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(1))),
        ),
    });

    let result = evaluation::evaluate(&max_with_value_not_found_condition, &facts);
    assert!(result);
}

#[test]
fn max_from_fixed_false() {
    let facts = JsonFacts::new(serde_json::json!({}));

    let condition = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Max(vec![
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(3))),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(2))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(2))),
        ),
    });

    let result = evaluation::evaluate(&condition, &facts);
    assert!(!result);
}

#[test]
fn max_from_json_true() {
    let facts = JsonFacts::new(serde_json::json!({
        "three": 3.0
    }));

    let single_condition = SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Transform(Fn::Max(vec![
                ConditionValue::Json(ReferenceValue {
                    address: "three".to_string(),
                }),
                ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(2))),
            ])),
            ConditionValue::Fixed(FactValue::Number(NumberFact::Integer(3))),
        ),
    };

    let condition = Condition::Single(single_condition);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result);
}
