use crate::{
    Backpack, Chasing, CurrentMap, Direction, DoDamage, Equipment, EquippedBy, Health, InBackpack, PathfindingSteps,
    Position,
};
use bevy::prelude::{
    App, Commands, Entity, Event, EventReader, EventWriter, GlobalTransform, Name, Plugin, PreUpdate, Query, Res,
    ResMut, Transform, info,
};
use std::ops::Neg;

mod combat;
use combat::*;
mod movement;
use movement::*;
mod inventory;
use inventory::*;
mod healthy;
use healthy::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Effect<Move>>()
            .add_event::<Effect<Chase>>()
            .add_event::<Effect<Attack>>()
            .add_event::<Effect<Damage>>()
            .add_event::<Effect<Heal>>()
            .add_event::<Effect<Death>>()
            .add_event::<Effect<PickUpItem>>()
            .add_event::<Effect<UseItem>>()
            .add_event::<Effect<DropItem>>()
            .add_event::<Effect<EquipItem>>()
            .add_systems(
                PreUpdate,
                (
                    move_entity_to,
                    chase_entity,
                    attack_entity,
                    inflict_damage,
                    death,
                    pick_up_item,
                    use_item,
                    drop_item,
                    equip_item,
                    heal_entity,
                ),
            );
    }
}

#[derive(Event)]
pub struct Effect<T> {
    pub data: T,
    pub creator: Option<Entity>,
    pub targets: Targets,
}

#[derive(Clone)]
pub enum Targets {
    Single { target: Entity },
    TargetList { targets: Vec<Entity> },
    Tile { tile: Position },
    Tiles { tiles: Vec<Position> },
}

pub struct Move {}
pub struct Chase {}
pub struct Attack {}
pub struct Damage(pub u32);
pub struct Death {}
pub struct PickUpItem {}
pub struct UseItem {}
pub struct DropItem {}
pub struct EquipItem {}
pub struct Heal(pub u32);
