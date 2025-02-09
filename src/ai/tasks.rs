/// Represents either a primitive or compound task
#[derive(Debug, Clone)]
pub enum Task {
    Primitive(PrimitiveTask),
    Compound(CompoundTask),
}

/// Represents an action that can be executed
#[derive(Debug, Default, Clone, Copy)]
pub enum PrimitiveTask {
    #[default]
    WanderAround,
    GatherResources,
    BuildStructure,
    AttackEnemy,
}

/// Represents a higher-level task that decomposes into sub-tasks
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompoundTask {
    Survive,
    DefendBase,
}

impl Task {
    fn execute(&self) {
        if let Task::Primitive(task) = self {
            match task {
                PrimitiveTask::AttackEnemy => {}
                PrimitiveTask::WanderAround => {}
                PrimitiveTask::BuildStructure => {}
                PrimitiveTask::GatherResources => {}
            }
        }
    }

    // Here you declare the checks of the tasks
}
