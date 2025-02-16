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
pub struct Renderable(
    pub String, //TODO now is the path. This should be the identifier of the texture
);

#[derive(Deserialize, Component, Reflect, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Health {
    pub current: i32,
    pub max: i32,
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
