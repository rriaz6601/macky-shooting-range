use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub id: Option<i32>,
    pub node_id: i32,
    pub distance: f64,
    pub image_num: i32,
}

impl Target {
    pub fn image_path(&self) -> String {
        format!("assets/ShootingTarget_graphics{}.png", self.image_num)
    }
}

