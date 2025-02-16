use super::{CreatureBundle, ItemBundle, TileBundle};
use crate::{
    on_click, Creature, CursorHighlight, Direction, GameState, Health, IsoGrid, Item, PathfindingSteps, Position,
    SpawnEntity, Tile, Viewshed, ViewshedHighlight,
};
use bevy::prelude::{
    warn, AssetServer, Commands, Component, Entity, EventWriter, Name, Over, Pointer, Query, Res, ResMut, Resource,
    Sprite, StateScoped, Transform, Trigger,
};
use std::collections::{HashMap, HashSet};
use std::ops::Neg;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum SpawnType {
    AtPosition { x: i32, y: i32, z: i32 },
    Equipped { by: Entity },
    Carried { by: Entity },
}

#[derive(Debug, Default)]
pub struct Raws {
    pub tiles: Vec<TileBundle>,
    pub creatures: Vec<CreatureBundle>,
    pub items: Vec<ItemBundle>,
}

#[derive(Default, Resource, Debug)]
pub struct RawMaster {
    pub raws: Raws,
    pub tile_index: HashMap<String, usize>,
    pub creature_index: HashMap<String, usize>,
    pub item_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn load(&mut self) {
        let mut used_names: HashSet<String> = HashSet::new();

        process_raws(
            &self.raws.tiles,
            &mut self.tile_index,
            &mut used_names,
            |tile| &tile.name,
            "Tile",
        );
        process_raws(
            &self.raws.creatures,
            &mut self.creature_index,
            &mut used_names,
            |creature| &creature.name,
            "Creature",
        );
        process_raws(
            &self.raws.items,
            &mut self.item_index,
            &mut used_names,
            |item| &item.name,
            "Item",
        );
    }

    pub fn spawn_named_tile(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        current_map: &mut ResMut<IsoGrid>,
        key: String,
        pos: SpawnType,
    ) -> Entity {
        let tile_template = &self.raws.tiles[self.tile_index[&key]].clone();

        let entity = commands.spawn_empty().id();
        // When we leave GameState::InGame it will despawn
        commands.entity(entity).insert(StateScoped(GameState::InGame));
        // Marker
        commands.entity(entity).insert(Tile {});
        // Name
        commands.entity(entity).insert(Name::new(tile_template.name.clone()));
        // Sprite
        commands
            .entity(entity)
            .insert(Sprite::from_image(asset_server.load(tile_template.sprite.clone())));
        // Position
        commands.entity(entity).insert(spawn_position(pos));
        // Transform
        if let SpawnType::AtPosition { x, y, z } = pos {
            let coord = current_map.layout.tile_to_world_pos(Position { x, y, z });
            current_map.tiles.insert(Position { x, y, z }, entity);
            //commands.entity(entity).with_children(|b| {
            //    b.spawn((
            //        Text2d(format!("{},{}", x, y)),
            //        TextColor(Color::BLACK),
            //        TextFont { font_size: 6.0, ..default() },
            //        Transform::from_xyz(0.0, 8.0, 10.0),
            //    ));
            //});
            if tile_template.blocker {
                current_map.blocked_coords.insert(Position { x, y, z });
            }
            if tile_template.name == "SelectedBlock" {
                commands.entity(entity).insert(Transform::from_xyz(
                    coord.x,
                    coord.y,
                    coord.y.neg() / 100.0 + coord.z + 0.002,
                ));
            } else if tile_template.name == "ViewshedFloor" {
                commands.entity(entity).insert(Transform::from_xyz(
                    coord.x,
                    coord.y,
                    ((coord.y.neg() as f32) / 100.0) + coord.z + 0.001,
                ));
            } else {
                commands.entity(entity).insert(Transform::from_xyz(
                    coord.x,
                    coord.y,
                    ((coord.y.neg() as f32) / 100.0) + coord.z,
                ));
            }
        }
        if tile_template.name == "SelectedBlock" {
            commands.entity(entity).insert(CursorHighlight {});
        } else {
            if tile_template.name == "ViewshedFloor" {
                commands.entity(entity).insert(ViewshedHighlight {});
            }
            // Pathfinding
            commands.entity(entity).insert(PathfindingSteps::new());
            // Hovering Observers
            commands.entity(entity).observe(
                |ev: Trigger<Pointer<Over>>,
                 mut commands: Commands,
                 mut spawn_event: EventWriter<SpawnEntity>,
                 pos_query: Query<&Position>,
                 highlighted_query: Query<(Entity, &Position, &CursorHighlight)>| {
                    // TODO this is bad we need something better
                    let mut new_selected = HashSet::new();
                    if let Ok(pos) = pos_query.get(ev.entity()) {
                        new_selected.insert(pos);
                        spawn_event.send(SpawnEntity {
                            name: "SelectedBlock".to_string(),
                            pos: SpawnType::AtPosition {
                                x: pos.x,
                                y: pos.y,
                                z: pos.z,
                            },
                        });
                    }
                    for (entity, pos, _) in &highlighted_query {
                        if !new_selected.contains(pos) {
                            commands.entity(entity).despawn();
                        }
                    }
                },
            );
        }
        // Picking Observers
        commands.entity(entity).observe(on_click);

        entity
    }

    pub fn spawn_named_creature(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        current_map: &mut ResMut<IsoGrid>,
        key: String,
        pos: SpawnType,
    ) -> Entity {
        let creature_template = &self.raws.creatures[self.creature_index[&key]].clone();

        let entity = commands.spawn_empty().id();
        // When we leave GameState::InGame it will despawn
        commands.entity(entity).insert(StateScoped(GameState::InGame));
        // Marker
        commands.entity(entity).insert(Creature {});
        // Name
        commands
            .entity(entity)
            .insert(Name::new(creature_template.name.clone()));
        // Sprite
        commands
            .entity(entity)
            .insert(Sprite::from_image(asset_server.load(creature_template.sprite.clone())));
        // Position
        commands.entity(entity).insert(spawn_position(pos));

        // Place in the world
        if let SpawnType::AtPosition { x, y, z } = pos {
            let coord = current_map.layout.tile_to_world_pos(Position { x, y, z });
            current_map.entities.insert(Position::new(x, y, z), entity);
            // Transform
            commands.entity(entity).insert(Transform::from_xyz(
                coord.x,
                coord.y,
                coord.y.neg() / 100.0 + coord.z + 0.003,
            ));
            // Viewshed only if the creature is AtPosition for now think about this later
            commands.entity(entity).insert(Viewshed {
                // TODO put the range fov in the data files
                visible_tiles: HashSet::new(),
                range: creature_template.view_range,
                angle: creature_template.view_angle,
            });
        };
        // Health
        commands.entity(entity).insert(Health {
            current: creature_template.max_health,
            max: creature_template.max_health,
        });
        // Pathfinding
        commands.entity(entity).insert(Direction::default());
        // Pathfinding
        commands.entity(entity).insert(PathfindingSteps::new());

        entity
    }

    pub fn spawn_named_item(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        current_map: &ResMut<IsoGrid>,
        key: String,
        pos: SpawnType,
    ) -> Entity {
        let item_template = &self.raws.items[self.item_index[&key]].clone();

        let entity = commands.spawn_empty().id();
        // When we leave GameState::InGame it will despawn
        commands.entity(entity).insert(StateScoped(GameState::InGame));
        // Marker
        commands.entity(entity).insert(Item {});
        // Name
        commands.entity(entity).insert(Name::new(item_template.name.clone()));
        // Sprite
        commands
            .entity(entity)
            .insert(Sprite::from_image(asset_server.load(item_template.sprite.clone())));
        // Position
        commands.entity(entity).insert(spawn_position(pos));
        // Transform
        if let SpawnType::AtPosition { x, y, z } = pos {
            let coord = current_map.layout.tile_to_world_pos(Position { x, y, z });
            commands
                .entity(entity)
                .insert(Transform::from_xyz(coord.x, coord.y, coord.y.neg() + coord.z + 2.));
        }

        entity
    }

    pub fn spawn_named_entity(
        &self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        current_map: &mut ResMut<IsoGrid>,
        key: String,
        pos: SpawnType,
    ) -> Option<Entity> {
        if self.tile_index.contains_key(&key) {
            return Some(self.spawn_named_tile(commands, asset_server, current_map, key, pos));
        }
        if self.creature_index.contains_key(&key) {
            return Some(self.spawn_named_creature(commands, asset_server, current_map, key, pos));
        }
        if self.item_index.contains_key(&key) {
            return Some(self.spawn_named_item(commands, asset_server, current_map, key, pos));
        }
        None
    }
}

fn spawn_position(pos: SpawnType) -> impl Component {
    // Spawn in the specified location
    match pos {
        SpawnType::AtPosition { x, y, z } => Position { x, y, z },
        _ => Position { x: 0, y: 0, z: 0 },
        //    SpawnType::Carried { by } => InBackpack { owner: by },
        //    SpawnType::Equipped { by } => {
        //        let slot = find_slot_for_equippable_item(tag, raws);
        //        Equipped { owner: by, slot }
        //    }
    }
}

fn process_raws<T, F>(
    raws: &[T],
    index: &mut HashMap<String, usize>,
    used_names: &mut HashSet<String>,
    get_name: F,
    raw_type: &str,
) where
    F: Fn(&T) -> &String,
{
    for (i, raw) in raws.iter().enumerate() {
        let name = get_name(raw);
        if used_names.contains(name) {
            warn!("{}: {} is duplicated in the data files", raw_type, name);
        }
        index.insert(name.clone(), i);
        used_names.insert(name.clone());
    }
}
