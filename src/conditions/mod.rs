use self::functions::Fn;
use self::operators::{AggregationOperator, ComparisonOperator};
use crate::facts::FactValue;
use serde::{Deserialize, Serialize};

pub mod functions;
pub mod operators;

#[derive(Serialize, Deserialize)]
pub enum Condition {
    Group(ConditionGroup),
    Single(SingleCondition),
    Array(ArrayCondition),
}

#[derive(Serialize, Deserialize)]
pub struct ConditionGroup {
    pub operator: AggregationOperator,
    pub conditions: Vec<Condition>,
}

#[derive(Serialize, Deserialize)]
pub struct SingleCondition {
    pub operator: ComparisonOperator,
    pub values: (ConditionValue, ConditionValue),
}

#[derive(Serialize, Deserialize)]
pub struct ArrayCondition {
    pub operator: AggregationOperator,
    pub array: ReferenceValue,
    pub condition: Box<Condition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConditionValue {
    Fixed(FactValue),
    Json(ReferenceValue),
    Transform(Fn),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceValue {
    pub address: String,
}
