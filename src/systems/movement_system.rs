use crate::{find_path, Creature, Direction, IsoGrid, PathfindingSteps, Position};
use bevy::prelude::{warn, Query, ResMut, Transform, With};
use rand::prelude::*;
use std::{collections::VecDeque, ops::Neg};

pub fn move_system(
    mut mob_query: Query<(&mut Transform, &mut Position, &mut PathfindingSteps, &mut Direction), With<Creature>>,
    mut grid: ResMut<IsoGrid>,
) {
    // this ideally should calculate the direction to go between the actual position with the
    // next step and then move the mob in that direction
    for (mut mob_transform, mut mob_pos, mut mob_step, mut direction) in mob_query.iter_mut() {
        // If there is nothing in the qeue have and "idle" behavior
        // either don't move or move randomly to one of the neighbors
        {
            let mut rng = rand::rng();
            if mob_step.steps.is_empty() && rng.random_ratio(1, 20) {
                let neighbors = mob_pos.all_neighbors();
                let valid_moves: Vec<Position> = neighbors
                    .into_iter()
                    .filter(|pos| grid.tiles.contains_key(pos))
                    .collect();
                if let Some(&next_pos) = valid_moves.choose(&mut rng) {
                    mob_step.steps.push_front(next_pos);
                }
            }
        }
        // Get the next position to move
        let Some(mut next_step) = mob_step.steps.pop_front() else {
            continue;
        };

        // Check if the next step is a blocked tile(can happen as we don't check every time a blocked tile is added)
        if grid.blocked_coords.contains(&next_step) {
            let Some(destination) = mob_step.steps.pop_back() else {
                continue;
            };
            if let Some(new_path) = find_path(&mob_pos, &destination, &grid) {
                mob_step.steps = VecDeque::from(new_path);
                next_step = mob_step
                    .steps
                    .pop_front()
                    .expect("the pathfinding should return something");
            } else {
                mob_step.steps.clear(); // No path found, stop movement
                warn!("No path found for {:?}", mob_pos);
                continue;
            }
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
        if let Some(mob_entity) = grid.entities.remove(&*mob_pos) {
            grid.entities.insert(next_step, mob_entity);
        } else {
            warn!("Tried to move non-existent entity at {:?}", mob_pos);
            continue;
        }
        // Update the creature position
        *mob_pos = next_step;
    }
}
