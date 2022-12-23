use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SurfaceDef<'a> {
    pub label: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub texture_index: u32,
    pub support: f32,
}
