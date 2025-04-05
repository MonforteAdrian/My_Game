use super::*;

pub fn heal_entity(mut event: EventReader<Effect<Heal>>, mut health_query: Query<(&Name, &mut Health)>) {
    for ev in event.read() {
        match &ev.targets {
            Targets::Single { target } => {
                let Ok((name, mut hp)) = health_query.get_mut(*target) else { continue };
                if hp.current == hp.max {
                    info!(
                        "{} have consume a heal item. Gaining 0 hp because was already healthy. What about opening your eyes next time? Love you, mum.",
                        name
                    );
                    continue;
                }
                info!("{} have been healed. Gaining {} hp", name, ev.data.0);
                hp.current = (hp.current + ev.data.0).min(hp.max);
            }
            Targets::TargetList { targets } => {}
            Targets::Tile { tile } => {}
            Targets::Tiles { tiles } => {}
        }
    }
}
