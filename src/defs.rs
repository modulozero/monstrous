use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SurfaceDef {
    label: String,
    name: String,
    description: String,
    texture_index: u32,
    support: f32,
}
