use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum AggregationOperator {
    Every,
    Any,
    None,
}

#[derive(Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equals,
    GreaterThan,
    GreaterThanEqualTo,
    LessThan,
    LessThanEqualTo,
}
