use crate::Race;
use serde::Deserialize;

// TODO maybe in the future we can use bundles and optionals
#[derive(Deserialize, Debug, Clone)]
pub struct CreatureBundle {
    pub name: String,
    pub sprite: String,
    pub race: Race,
}
