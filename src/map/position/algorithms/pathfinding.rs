use crate::{position, CurrentMap, Position};
use bevy::prelude::{warn, Component, Reflect};
use std::collections::VecDeque;

mod a_star;
use a_star::a_star;

#[derive(Component, Reflect, Debug, Clone, Eq, PartialEq, Default)]
pub struct PathfindingSteps(VecDeque<Position>);

impl PathfindingSteps {
    pub fn new() -> Self {
        Self(VecDeque::default())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn pop_front(&mut self) -> Option<Position> {
        self.0.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<Position> {
        self.0.pop_back()
    }

    pub fn back(&self) -> Option<&Position> {
        self.0.back()
    }

    pub fn create_path(&mut self, o_pos: &Position, d_pos: &Position, grid: &CurrentMap) {
        if let Some(path) = find_path(o_pos, d_pos, grid) {
            self.0 = VecDeque::from(path);
        } else {
            warn!("No path found  from {:?} to {:?}", o_pos, d_pos);
        };
    }
}

pub fn find_path(o_pos: &Position, d_pos: &Position, grid: &CurrentMap) -> Option<Vec<Position>> {
    a_star(*o_pos, *d_pos, |o, h| {
        // Implementation of blocked_coords
        if h.x == 0 || h.y == 0 {
            // Neighbor
            (grid.tiles.contains_key(&h) && !grid.blocked_coords.contains(&h)).then_some(100)
        } else {
            // Diagonal
            (grid.tiles.contains_key(&h)
                && !grid.blocked_coords.contains(&h)
                && !grid.blocked_coords.contains(&position(h.x, o.y, o.z))
                && !grid.blocked_coords.contains(&position(o.x, h.y, o.z)))
            // The diagonal move is 1.41 times the move distance to a neighbor
            // We use 100 times bigger to use u32 instead of float
            .then_some(141)
        }
    })
}
