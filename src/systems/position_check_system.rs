use crate::{Creature, CurrentMap, Effect, PickUpItem, Position, Targets};
use bevy::prelude::{info, Entity, EventWriter, Name, Query, Res, With};

pub fn position_check_system(
    mob_query: Query<(Entity, &Name, &Position), With<Creature>>,
    mut pick_up_item_event: EventWriter<Effect<PickUpItem>>,
    curren_map: Res<CurrentMap>,
) {
    for (entity, name, pos) in mob_query.iter() {
        if let Some(item_entity) = curren_map.items.get(pos) {
            info!("Uh! {} have found a goodie", name);
            pick_up_item_event.send(Effect::<PickUpItem> {
                data: PickUpItem {},
                creator: Some(entity),
                targets: Targets::Single { target: *item_entity },
            });
        } else {
            continue;
        };
    }
}
