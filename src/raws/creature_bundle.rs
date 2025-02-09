use serde::Deserialize;

// TODO maybe in the future we can use bundles and optionals
#[derive(Deserialize, Debug, Clone)]
pub struct CreatureBundle {
    pub name: String,
    pub sprite: String,
    pub view_range: u32,
    pub view_angle: u32,
    pub max_health: i32,
}
