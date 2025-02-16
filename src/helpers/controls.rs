use crate::{Creature, MoveTo, Position, Targets};
use bevy::prelude::*;

pub fn on_click(
    ev: Trigger<Pointer<Click>>,
    mut event: EventWriter<MoveTo>,
    pos: Query<&Position>,
    mut creature_query: Query<(Entity, &Name), With<Creature>>,
) {
    let Ok(destination) = pos.get(ev.entity()) else {
        return;
    };
    match ev.button {
        PointerButton::Primary => {
            for (entity, name) in creature_query.iter_mut() {
                if name.as_str() == "Dummy" {
                    event.send(MoveTo {
                        creator: Some(entity),
                        targets: Targets::Tile { tile: *destination },
                    });
                }
            }
        }
        PointerButton::Secondary => {}
        PointerButton::Middle => {}
    }
}
