use crate::ProvidesHeal;

use super::*;

pub fn pick_up_item(
    mut event: EventReader<Effect<PickUpItem>>,
    mut commands: Commands,
    query: Query<(&Name, &Position)>,
    mut backpack_query: Query<&mut Backpack>,
    mut current_map: ResMut<CurrentMap>,
) {
    for ev in event.read() {
        let Some(creature) = ev.creator else { continue };
        let Targets::Single { target } = ev.targets else { continue };
        if let Ok((creature_name, _)) = query.get(creature)
            && let Ok((target_name, target_pos)) = query.get(target)
        {
            info!("{} is picking up {}", creature_name, target_name);
            let Ok(mut backpack) = backpack_query.get_mut(creature) else {
                info!("{} don't have backpack", creature_name);
                continue;
            };
            let _ = current_map.items.remove(target_pos);
            backpack.content.insert(target);
            commands
                .entity(target)
                .remove::<Position>()
                .remove::<GlobalTransform>()
                .remove::<Transform>()
                .insert(InBackpack { owner: creature });
        }
    }
}

pub fn use_item(
    mut event: EventReader<Effect<UseItem>>,
    mut commands: Commands,
    mut heal_event: EventWriter<Effect<Heal>>,
    query: Query<(&Name, Option<&ProvidesHeal>)>,
    mut backpack_query: Query<&mut Backpack>,
) {
    for ev in event.read() {
        let Some(user) = ev.creator else { continue };
        let Targets::Single { target } = ev.targets else { continue };
        if let Ok((user_name, _)) = query.get(user)
            && let Ok((target_name, provides_heal)) = query.get(target)
        {
            let Ok(mut backpack) = backpack_query.get_mut(user) else {
                info!("{} don't have backpack", user_name);
                continue;
            };
            info!("{} is using {}", user_name, target_name);
            backpack.content.remove(&target);

            if let Some(heal) = provides_heal {
                heal_event.send(Effect::<Heal> {
                    data: Heal(heal.0),
                    creator: None,
                    targets: Targets::Single { target: user },
                });
            }
            commands.entity(target).despawn();
        }
    }
}

pub fn drop_item(
    mut event: EventReader<Effect<DropItem>>,
    mut commands: Commands,
    query: Query<(&Name, Option<&Position>, Option<&Direction>)>,
    mut backpack_query: Query<&mut Backpack>,
    mut current_map: ResMut<CurrentMap>,
) {
    for ev in event.read() {
        let Some(user) = ev.creator else { continue };
        let Targets::Single { target } = ev.targets else { continue };
        if let Ok((user_name, user_pos, user_dir)) = query.get(user)
            && let Ok((target_name, _, _)) = query.get(target)
            && let Some(user_pos) = user_pos
            && let Some(user_dir) = user_dir
        {
            info!("{} is dropping {}", user_name, target_name);
            let neighbors = user_pos.all_neighbors();
            dbg!(user_pos);
            dbg!(neighbors[user_dir.0 as usize]);
            let dropping_pos = neighbors[user_dir.0 as usize];
            dbg!(dropping_pos);
            current_map.items.insert(dropping_pos, target);
            let Ok(mut backpack) = backpack_query.get_mut(user) else {
                info!("{} don't have backpack", user_name);
                continue;
            };
            backpack.content.remove(&target);
            let coord = current_map.layout.tile_to_world_pos(dropping_pos);
            commands
                .entity(target)
                .insert(dropping_pos)
                .insert(Transform::from_xyz(
                    coord.x,
                    coord.y,
                    coord.y.neg() / 100.0 + coord.z + 0.003,
                ))
                .remove::<InBackpack>();
        }
    }
}

pub fn equip_item(
    mut event: EventReader<Effect<EquipItem>>,
    mut commands: Commands,
    query: Query<&Name>,
    mut backpack_query: Query<(&mut Backpack, &mut Equipment)>,
) {
    for ev in event.read() {
        let Some(user) = ev.creator else { continue };
        let Targets::Single { target } = ev.targets else { continue };
        if let Ok(user_name) = query.get(user)
            && let Ok(target_name) = query.get(target)
        {
            let Ok((mut backpack, mut equipment)) = backpack_query.get_mut(user) else {
                info!("{} don't have inventory", user_name);
                continue;
            };
            info!("{} has equipped {}", user_name, target_name);
            backpack.content.remove(&target);
            equipment.holding_right_hand = Some(target);
            commands
                .entity(target)
                .remove::<InBackpack>()
                .insert(EquippedBy { owner: user });
        }
    }
}
