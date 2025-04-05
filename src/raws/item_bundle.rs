use serde::Deserialize;

// TODO maybe in the future we can use bundles and optionals
#[derive(Deserialize, Debug, Clone)]
pub struct ItemBundle {
    pub name: String,
    pub sprite: String,
    pub heal: Option<u32>,
    pub damage: Option<u32>,
}
