use crate::{find_path, Creature, EntityName, IsoGrid, PathfindingSteps, Position};
use bevy::prelude::*;
use std::collections::VecDeque;

pub fn on_click(
    ev: Trigger<Pointer<Click>>,
    pos: Query<&Position>,
    grid: Res<IsoGrid>,
    mut creature_query: Query<(&EntityName, &Position, &mut PathfindingSteps), With<Creature>>,
) {
    let Ok(pos) = pos.get(ev.entity()) else {
        return;
    };
    match ev.button {
        PointerButton::Primary => {
            for (creature_name, creature_pos, mut creature_steps) in creature_query.iter_mut() {
                if creature_name.0 != "Dummy" {
                    continue;
                }
                let Some(path) = find_path(creature_pos, pos, &grid) else {
                    warn!("No path found");
                    continue;
                };
                creature_steps.steps = VecDeque::from(path);
            }
        }
        PointerButton::Secondary => {}
        PointerButton::Middle => {}
    }
}
