use crate::{
    Attack, Chasing, Creature, CurrentMap, Direction, Effect, Move, PathfindingSteps, Position, Targets, find_path,
};
use bevy::prelude::{Entity, EventWriter, Query, ResMut, Transform, With, warn};
use rand::prelude::*;
use std::ops::Neg;

#[allow(clippy::type_complexity)]
pub fn move_system(
    mut mob_query: Query<
        (
            Entity,
            &mut Transform,
            &mut Position,
            &mut PathfindingSteps,
            &mut Direction,
            Option<&Chasing>,
        ),
        With<Creature>,
    >,
    mut move_entity_to_event: EventWriter<Effect<Move>>,
    mut attack_entity_event: EventWriter<Effect<Attack>>,
    mut grid: ResMut<CurrentMap>,
) {
    // this ideally should calculate the direction to go between the actual position with the
    // next step and then move the mob in that direction
    for (entity, mut mob_transform, mut mob_pos, mut mob_steps, mut direction, mob_chasing) in mob_query.iter_mut() {
        // If there is nothing in the qeue have and "idle" behavior
        // either don't move or move randomly to one of the neighbors
        if mob_steps.is_empty() {
            if let Some(destination) = find_random_valid_move(&grid, &mob_pos) {
                move_entity_to_event.write(Effect::<Move> {
                    data: Move {},
                    creator: Some(entity),
                    targets: Targets::Tile { tile: destination },
                });
            }
            continue;
        }
        // If the creature is chasing and the next step is the target
        // remove chasing and attack
        if let Some(chasing) = mob_chasing
            && mob_steps.len() == 1
        {
            attack_entity_event.write(Effect::<Attack> {
                data: Attack {},
                creator: Some(entity),
                targets: Targets::Single { target: chasing.0 },
            });
            continue;
        }

        // Get the next position to move
        let Some(next_step) = mob_steps.pop_front() else { continue };

        // Check if the next step is a blocked tile(can happen as we don't check every time a blocked tile is added)
        if grid.blocked_coords.contains(&next_step) {
            if let Some(destination) = mob_steps.pop_back() {
                move_entity_to_event.write(Effect::<Move> {
                    data: Move {},
                    creator: Some(entity),
                    targets: Targets::Tile { tile: destination },
                });
            };
            continue;
        }

        // TODO instead of changing to the block calculate the direction and move to the block in a fixed speed
        let step = grid.layout.tile_to_world_pos(next_step);
        mob_transform.translation.x = step.x;
        mob_transform.translation.y = step.y;
        mob_transform.translation.z = step.y.neg() / 100.0 + step.z + 0.003;

        // Update the direction of the creature
        if let Some(direction_index) = mob_pos.direction_to_neighbor(next_step) {
            direction.0 = direction_index;
        }

        // Update the entity position in the Current Map
        if grid.entities.remove(&*mob_pos).is_some() {
            grid.entities.insert(next_step, entity);
        } else {
            warn!("Tried to move non-existent entity at {:?}", mob_pos);
            continue;
        }
        // Update the creature position
        *mob_pos = next_step;
    }
}

/// Finds a valid random move that also has a valid path.
fn find_random_valid_move(grid: &CurrentMap, mob_pos: &Position) -> Option<Position> {
    let mut rng = rand::rng();
    if !rng.random_ratio(1, 20) {
        return None;
    }
    let neighbors = mob_pos.all_neighbors();
    let valid_moves: Vec<Position> = neighbors
        .into_iter()
        .filter(|pos| grid.tiles.contains_key(pos)) // Check if the tile exists
        .filter(|pos| find_path(mob_pos, pos, grid).is_some()) // Check if a path exists
        .collect();

    valid_moves.choose(&mut rng).cloned()
}
