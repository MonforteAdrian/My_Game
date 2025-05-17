use bevy::reflect::attributes;

use crate::Attributes;

use super::*;

pub fn attack_entity(
    mut event: EventReader<Effect<Attack>>,
    mut effect_event: EventWriter<Effect<Damage>>,
    query: Query<(&Name, &Attributes, &Equipment)>,
    weapon_query: Query<&DoDamage>,
) {
    for ev in event.read() {
        let Some(attacker) = ev.creator else { continue };
        let Targets::Single { target } = ev.targets else { continue };
        if let Ok((attacker_name, attributes, equipment)) = query.get(attacker)
            && let Ok((target_name, _, _)) = query.get(target)
        {
            info!("{} is attacking {}", attacker_name, target_name);
            let damage = if let Some(weapon) = equipment.holding_right_hand
                && let Ok(dmg) = weapon_query.get(weapon)
            {
                dmg.0 + attributes.strength / 2
            } else {
                attributes.strength
            };
            dbg!(&damage);
            effect_event.write(Effect::<Damage> {
                data: Damage(damage),
                creator: Some(attacker),
                targets: Targets::Single { target },
            });
        }
    }
}

pub fn inflict_damage(
    mut event: EventReader<Effect<Damage>>,
    mut death_event: EventWriter<Effect<Death>>,
    mut health_query: Query<(&Name, &mut Health)>,
) {
    for ev in event.read() {
        match &ev.targets {
            Targets::Single { target } => {
                let Ok((name, mut hp)) = health_query.get_mut(*target) else { continue };
                info!("{} have been hit. Losing {} hp", name, ev.data.0);
                hp.current = hp.current.checked_sub(ev.data.0).unwrap_or(0);
                if hp.current == 0 {
                    death_event.write(Effect::<Death> {
                        data: Death {},
                        creator: ev.creator,
                        targets: ev.targets.clone(),
                    });
                }
            }
            Targets::TargetList { targets } => {}
            Targets::Tile { tile } => {}
            Targets::Tiles { tiles } => {}
        }
    }
}

pub fn death(mut event: EventReader<Effect<Death>>, mut commands: Commands, names_query: Query<&Name>) {
    for ev in event.read() {
        match &ev.targets {
            Targets::Single { target } => {
                let Ok(mut entity) = commands.get_entity(*target) else { continue };
                entity.despawn();
                let Ok(victim) = names_query.get(*target) else { continue };
                if let Some(killer) = ev.creator
                    && let Ok(killer) = names_query.get(killer)
                {
                    info!("{} killed {}, By fart! Rest in pieces.", killer, victim);
                } else {
                    info!("{}, Just died! Rest in pieces.", victim);
                    continue;
                };
            }
            Targets::TargetList { targets } => {}
            Targets::Tile { tile } => {}
            Targets::Tiles { tiles } => {}
        }
    }
}
