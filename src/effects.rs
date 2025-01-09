use crate::Position;
use bevy::prelude::{App, Entity, Event, EventReader, Plugin, Update, World};

mod damage;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EffectEvent>()
            .add_systems(Update, run_effects);
    }
}

#[derive(Event)]
pub struct EffectEvent(EffectSpawner);

pub enum EffectType {
    Damage { amount: i32 },
}

pub struct EffectSpawner {
    pub creator: Option<Entity>,
    pub effect_type: EffectType,
    pub targets: Targets,
}

#[derive(Clone)]
pub enum Targets {
    Single { target: Entity },
    TargetList { targets: Vec<Entity> },
    Tile { tile_pos: Position },
    Tiles { tiles: Vec<Position> },
}

// TODO check if using simply commands instead of world would be enough
pub fn run_effects(ecs: &mut World, mut events: EventReader<EffectEvent>) {
    for ev in events.read() {
        target_applicator(ecs, &ev.0);
    }
}

fn target_applicator(ecs: &mut World, effect: &EffectSpawner) {
    match &effect.targets {
        Targets::Tile { tile_pos } => affect_tile(ecs, effect, *tile_pos),
        Targets::Tiles { tiles } => tiles
            .iter()
            .for_each(|tile_pos| affect_tile(ecs, effect, *tile_pos)),
        Targets::Single { target } => affect_entity(ecs, effect, *target),
        Targets::TargetList { targets } => targets
            .iter()
            .for_each(|entity| affect_entity(ecs, effect, *entity)),
    }
}

fn tile_effect_hits_entities(effect: &EffectType) -> bool {
    match effect {
        EffectType::Damage { .. } => true,
        _ => false,
    }
}

fn affect_tile(ecs: &mut World, effect: &EffectSpawner, tile_pos: Position) {
    if tile_effect_hits_entities(&effect.effect_type) {
        //let content = ecs.fetch::<Map>().tile_content[tile_idx as usize].clone();
        //content
        //    .iter()
        //    .for_each(|entity| affect_entity(ecs, effect, *entity));
    }
    // TODO: Run the effect
}

fn affect_entity(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    match &effect.effect_type {
        EffectType::Damage { .. } => damage::inflict_damage(ecs, effect, target),
        _ => {}
    }
}
