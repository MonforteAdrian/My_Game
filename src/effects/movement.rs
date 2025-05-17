use super::*;

pub fn chase_entity(
    mut event: EventReader<Effect<Chase>>,
    mut commands: Commands,
    mut move_entity_to_event: EventWriter<Effect<Move>>,
    target_query: Query<&Position>,
    entity_query: Query<Option<&Chasing>>,
    names_query: Query<&Name>,
) {
    for ev in event.read() {
        let Some(chaser) = ev.creator else { continue };
        // Get if the entity is already chasing
        // If the chasing is called on a entity that already have it just ignore it
        let Ok(None) = entity_query.get(chaser) else { continue };
        // Get the position of the target
        let Targets::Single { target } = ev.targets else { continue };
        if let Ok(chaser_name) = names_query.get(chaser)
            && let Ok(target_name) = names_query.get(target)
        {
            info!("{} is chasing {}", chaser_name, target_name);
        }
        let Ok(target_pos) = target_query.get(target) else { continue };
        // Add the Chasing component and send the move entity to event
        commands.entity(chaser).try_insert_if_new(Chasing(target));
        move_entity_to_event.write(Effect::<Move> {
            data: Move {},
            creator: Some(chaser),
            targets: Targets::Tile { tile: *target_pos },
        });
    }
}

pub fn move_entity_to(
    mut event: EventReader<Effect<Move>>,
    current_map: Res<CurrentMap>,
    mut query: Query<(&Position, &mut PathfindingSteps)>,
) {
    for ev in event.read() {
        let Targets::Tile { tile } = ev.targets else { continue };
        let Some(entity) = ev.creator else { continue };
        // Get the data for the specific entity to move
        let Ok((pos, mut steps)) = query.get_mut(entity) else { continue };

        // Calculate the path and asign, clearing the data
        steps.create_path(&pos, &tile, &current_map);
    }
}
