use crate::Position;
use bevy::prelude::{Component, Entity, Reflect};
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Tile {}

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Creature {}

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Item {}

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Consumable {}

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Renderable(
    pub String, //TODO now is the path. This should be the identifier of the texture
);

#[derive(Deserialize, Component, Reflect, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

impl Health {
    fn new(max: u32) -> Self {
        Self { current: max, max }
    }
}

#[derive(Deserialize, Component, Reflect, Debug, Clone, Eq, Hash, PartialEq)]
pub struct Chasing(pub Entity);

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct CursorHighlight {}

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct ViewshedHighlight {}

#[derive(Component, Debug, Reflect, Clone, Eq, PartialEq, Default)]
pub struct Viewshed {
    pub visible_tiles: HashSet<Position>,
    pub range: u32,
    pub angle: u32,
}

impl Viewshed {
    fn new(range: u32, angle: u32) -> Self {
        Self {
            range,
            angle,
            ..Default::default()
        }
    }
}

#[derive(Component, Debug, Reflect, Clone, Eq, PartialEq, Default)]
pub struct Backpack {
    pub content: HashSet<Entity>,
}

#[derive(Component, Debug, Reflect, Clone, Eq, PartialEq)]
pub struct InBackpack {
    pub owner: Entity,
}

#[derive(Component, Debug, Reflect, Clone, Eq, PartialEq, Default)]
pub struct Equipment {
    pub holding_right_hand: Option<Entity>,
}

#[derive(Component, Debug, Reflect, Clone, Eq, PartialEq)]
pub struct EquippedBy {
    pub owner: Entity,
}

#[derive(Component, Debug, Reflect, Clone, Eq, PartialEq)]
pub struct ProvidesHeal(pub u32);

#[derive(Component, Debug, Reflect, Clone, Eq, PartialEq)]
pub struct DoDamage(pub u32);

#[derive(Deserialize, Component, Debug, Reflect, Clone, Eq, PartialEq, Copy)]
pub enum Race {
    Human,
    BadHuman,
}

impl Race {
    pub fn get_attributes(&self) -> Attributes {
        match self {
            Self::Human => Attributes::new(10, 10, 10, 10, 10, 10, 10),
            Self::BadHuman => Attributes::new(12, 12, 12, 12, 1, 1, 1),
        }
    }

    pub fn get_health(&self) -> Health {
        match self {
            Self::Human => Health::new(100),
            Self::BadHuman => Health::new(101),
        }
    }
    pub fn get_viewshed(&self) -> Viewshed {
        match self {
            Self::Human => Viewshed::new(16, 120),
            Self::BadHuman => Viewshed::new(16, 120),
        }
    }
}

#[derive(Component, Debug, Reflect, Clone, Eq, PartialEq)]
pub struct Attributes {
    pub strength: u32,
    pub dexterity: u32,
    pub agility: u32,
    pub thoughness: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

impl Attributes {
    fn new(
        strength: u32,
        dexterity: u32,
        agility: u32,
        thoughness: u32,
        intelligence: u32,
        wisdom: u32,
        charisma: u32,
    ) -> Self {
        Self {
            strength,
            dexterity,
            agility,
            thoughness,
            intelligence,
            wisdom,
            charisma,
        }
    }
}
