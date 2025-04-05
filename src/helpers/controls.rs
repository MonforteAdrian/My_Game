use crate::{Creature, DropItem, Effect, EquipItem, Item, Move, Position, Targets, UseItem};
use bevy::prelude::*;

pub fn on_click(
    ev: Trigger<Pointer<Click>>,
    mut event: EventWriter<Effect<Move>>,
    mut equip_item_event: EventWriter<Effect<EquipItem>>,
    mut drop_item_event: EventWriter<Effect<DropItem>>,
    mut use_item_event: EventWriter<Effect<UseItem>>,
    pos: Query<&Position>,
    sword_query: Query<(Entity, &Name), With<Item>>,
    mut creature_query: Query<(Entity, &Name), With<Creature>>,
) {
    let Ok(destination) = pos.get(ev.target()) else {
        return;
    };
    match ev.button {
        PointerButton::Primary => {
            for (entity, name) in creature_query.iter_mut() {
                if name.as_str() == "Dummy" {
                    event.send(Effect::<Move> {
                        data: Move {},
                        creator: Some(entity),
                        targets: Targets::Tile { tile: *destination },
                    });
                }
            }
        }
        PointerButton::Secondary => {
            // TODO check first if its in the inventory
            for (entity, name) in creature_query.iter_mut() {
                if name.as_str() == "Dummy" {
                    for (item_entity, item_name) in sword_query.iter() {
                        if item_name.as_str() == "RustySword" {
                            equip_item_event.send(Effect::<EquipItem> {
                                data: EquipItem {},
                                creator: Some(entity),
                                targets: Targets::Single { target: item_entity },
                            });
                        }
                    }
                }
            }
        }
        PointerButton::Middle => {
            // TODO check first if its in the inventory
            for (entity, name) in creature_query.iter_mut() {
                if name.as_str() == "Dummy" {
                    for (item_entity, item_name) in sword_query.iter() {
                        if item_name.as_str() == "Heart" {
                            //drop_item_event.send(Effect::<DropItem> {
                            //    data: DropItem {},
                            //    creator: Some(entity),
                            //    targets: Targets::Single { target: item_entity },
                            //});
                            use_item_event.send(Effect::<UseItem> {
                                data: UseItem {},
                                creator: Some(entity),
                                targets: Targets::Single { target: item_entity },
                            });
                        }
                    }
                }
            }
        }
    }
}
