use crate::raws::*;
use crate::CurrentMap;
use bevy::prelude::{App, AssetServer, Commands, Event, EventReader, Plugin, PreUpdate, Res, ResMut};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEntity>().add_systems(PreUpdate, spawn_entity);
    }
}

#[derive(Event)]
pub struct SpawnEntity {
    pub name: String,
    pub pos: SpawnType,
}

pub fn spawn_entity(
    mut events: EventReader<SpawnEntity>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_map: ResMut<CurrentMap>,
    raw_master: Res<RawMaster>,
) {
    for ev in events.read() {
        if let Some(_entity) =
            raw_master.spawn_named_entity(&mut commands, &asset_server, &mut current_map, ev.name.clone(), ev.pos)
        {
            // TODO put the entity somewhere?
        }
    }
}
