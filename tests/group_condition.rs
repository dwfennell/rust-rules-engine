use engine::{
    conditions::{
        operators::{AggregationOperator, ComparisonOperator},
        Condition, ConditionGroup, ConditionValue, ReferenceValue, SingleCondition,
    },
    evaluation,
    facts::json_facts::JsonFacts,
};

#[test]
fn group_condition_every_true() {
    let facts = build_facts();

    let single_condition_one = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    });
    let single_condition_two = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "two_point_one".to_string(),
            }),
        ),
    });

    let condition_group = ConditionGroup {
        conditions: vec![single_condition_one, single_condition_two],
        operator: AggregationOperator::Every,
    };

    let condition = Condition::Group(condition_group);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result)
}

#[test]
fn group_condition_every_false() {
    let facts = build_facts();

    let single_condition_one = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    });
    let single_condition_two = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    });

    let condition_group = ConditionGroup {
        conditions: vec![single_condition_one, single_condition_two],
        operator: AggregationOperator::Every,
    };

    let condition = Condition::Group(condition_group);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(!result)
}

#[test]
fn group_condition_any_true() {
    let facts = build_facts();

    let single_condition_one = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    });
    let single_condition_two = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "two_point_one".to_string(),
            }),
        ),
    });

    let condition_group = ConditionGroup {
        conditions: vec![single_condition_one, single_condition_two],
        operator: AggregationOperator::Any,
    };

    let condition = Condition::Group(condition_group);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result)
}

#[test]
fn group_condition_any_false() {
    let facts = build_facts();

    let single_condition_one = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "two_point_one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    });
    let single_condition_two = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "two_point_one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    });

    let condition_group = ConditionGroup {
        conditions: vec![single_condition_one, single_condition_two],
        operator: AggregationOperator::Any,
    };

    let condition = Condition::Group(condition_group);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(!result)
}

#[test]
fn group_condition_none_true() {
    let facts = build_facts();

    let single_condition_one = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "two_point_one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    });
    let single_condition_two = Condition::Single(SingleCondition {
        operator: ComparisonOperator::LessThan,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    });

    let condition_group = ConditionGroup {
        conditions: vec![single_condition_one, single_condition_two],
        operator: AggregationOperator::None,
    };

    let condition = Condition::Group(condition_group);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(result)
}

#[test]
fn group_condition_none_false() {
    let facts = build_facts();

    let single_condition_one = Condition::Single(SingleCondition {
        operator: ComparisonOperator::Equals,
        values: (
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
            ConditionValue::Json(ReferenceValue {
                address: "one".to_string(),
            }),
        ),
    });

    let condition_group = ConditionGroup {
        conditions: vec![single_condition_one],
        operator: AggregationOperator::None,
    };

    let condition = Condition::Group(condition_group);
    let result = evaluation::evaluate(&condition, &facts);
    assert!(!result)
}

fn build_facts() -> JsonFacts {
    let data = serde_json::json!({
        "one".to_string(): 1,
        "two_point_one".to_string(): 2.1,
    });

    JsonFacts::new(data)
}
