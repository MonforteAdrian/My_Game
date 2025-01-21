use crate::Position;
use bevy::prelude::Component;
use serde::Deserialize;
use std::collections::{HashSet, VecDeque};

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Tile {}

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Creature {}

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Item {}

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct EntityName(pub String);

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct Renderable(
    pub String, //TODO now is the path. This should be the identifier of the texture
);

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct CursorHighlight {}

#[derive(Deserialize, Component, Debug, Clone, Eq, Hash, PartialEq, Default)]
pub struct ViewshedHighlight {}

#[derive(Component, Debug, Clone, Eq, PartialEq, Default)]
pub struct PathfindingSteps {
    pub steps: VecDeque<Position>,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Default)]
pub struct Viewshed {
    pub visible_tiles: HashSet<Position>,
    pub range: u32,
    pub angle: u32,
}
