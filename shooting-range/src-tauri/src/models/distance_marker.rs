use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceMarker {
    pub id: Option<i32>,
    pub marker_number: i32,
    pub distance: f64,
}

