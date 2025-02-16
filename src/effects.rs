use crate::{Chasing, Health, IsoGrid, PathfindingSteps, Position};
use bevy::{
    ecs::system::RunSystemOnce,
    prelude::{App, Commands, Entity, Event, EventReader, EventWriter, Plugin, PreUpdate, Query, Res, World},
};

//mod combat;
//use combat::*;
//mod movement;
//use movemennt::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DoDamage>()
            .add_event::<MoveTo>()
            .add_event::<Chase>()
            .add_event::<Attack>()
            .add_systems(PreUpdate, move_entity_to)
            .add_systems(PreUpdate, chase_entity)
            .add_systems(PreUpdate, attack_entity)
            .add_systems(PreUpdate, inflict_damage);
    }
}

#[derive(Clone)]
pub enum Targets {
    Single { target: Entity },
    TargetList { targets: Vec<Entity> },
    Tile { tile: Position },
    Tiles { tiles: Vec<Position> },
}

#[derive(Event)]
pub struct DoDamage {
    pub creator: Option<Entity>,
    pub amount: i32,
    pub targets: Targets,
}

pub fn inflict_damage(mut event: EventReader<DoDamage>, mut health_query: Query<&mut Health>) {
    for ev in event.read() {
        match &ev.targets {
            Targets::Single { target } => {
                let Ok(mut hp) = health_query.get_mut(*target) else {
                    continue;
                };
                hp.current -= ev.amount;
            }
            Targets::TargetList { targets } => {}
            Targets::Tile { tile } => {}
            Targets::Tiles { tiles } => {}
        }
    }
}

#[derive(Event)]
pub struct Chase {
    pub creator: Option<Entity>,
    pub targets: Targets,
}

pub fn chase_entity(
    mut event: EventReader<Chase>,
    mut commands: Commands,
    mut move_entity_to_event: EventWriter<MoveTo>,
    target_query: Query<&Position>,
    entity_query: Query<Option<&Chasing>>,
) {
    for ev in event.read() {
        let Targets::Single { target } = ev.targets else {
            continue;
        };
        // Get the position of the target
        let Ok(target_pos) = target_query.get(target) else {
            continue;
        };
        let Some(chaser) = ev.creator else {
            continue;
        };
        // Get if the entity is already chasing
        if let Ok(chasing) = entity_query.get(chaser) {
            // If the chasing is called on a entity that already have it just ignore it
            if chasing.is_some() {
                continue;
            }
        } else {
            continue;
        };
        // Add the Chasing component and send the move entity to event
        commands.entity(chaser).try_insert_if_new(Chasing(target));
        move_entity_to_event.send(MoveTo {
            creator: Some(chaser),
            targets: Targets::Tile { tile: *target_pos },
        });
    }
}

#[derive(Event)]
pub struct Attack {
    pub creator: Option<Entity>,
    pub targets: Targets,
}

pub fn attack_entity(mut event: EventReader<Attack>, mut effect_event: EventWriter<DoDamage>) {
    for ev in event.read() {
        let Targets::Single { target } = ev.targets else {
            continue;
        };
        let Some(attacker) = ev.creator else {
            continue;
        };
        effect_event.send(DoDamage {
            creator: Some(attacker),
            amount: 5,
            targets: Targets::Single { target },
        });
    }
}

#[derive(Event)]
pub struct MoveTo {
    pub creator: Option<Entity>,
    pub targets: Targets,
}

pub fn move_entity_to(
    mut event: EventReader<MoveTo>,
    current_map: Res<IsoGrid>,
    mut query: Query<(&Position, &mut PathfindingSteps)>,
) {
    for ev in event.read() {
        let Targets::Tile { tile } = ev.targets else {
            continue;
        };
        let Some(entity) = ev.creator else {
            continue;
        };
        // Get the data for the specific entity to move
        let Ok((pos, mut steps)) = query.get_mut(entity) else {
            continue;
        };

        // Calculate the path and asign, clearing the data
        steps.create_path(&pos, &tile, &current_map);
    }
}
