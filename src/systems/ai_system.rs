use crate::{CompoundTask, Domain, PrimitiveTask, Task, WorldState, AI};
use bevy::prelude::Query;

/// System to process AI planning and execution dynamically
fn ai_system(mut query: Query<(&AI, &WorldState)>) {
    for ai in query.iter_mut() {
        let creature_state = WorldState {
            resources_available: true,
            enemies_nearby: false,
        };
        let planner = Domain::new();
        let task = Task::Compound(CompoundTask::Survive);
        let plan = planner.decompose(&task, &creature_state);

        println!("Generated dynamic plan: {:?}", plan);
        execute_plan(plan);
    }
}

/// Simulates execution of a dynamically generated HTN plan
fn execute_plan(plan: Vec<PrimitiveTask>) {
    for action in plan {
        println!("Executing: {:?}", action);
    }
}
