use super::*;
use std::collections::HashMap;

/// Defines a method that decomposes a compound task into subtasks
pub struct Method {
    task: CompoundTask,
    subtasks: Vec<Task>,
}

/// Domain is the structure used to describe the entire task hierarchy.
pub struct Domain {
    methods: HashMap<CompoundTask, Vec<Method>>, // Maps compound tasks to their possible methods
}

impl Domain {
    pub fn new() -> Self {
        let mut planner = Self {
            methods: HashMap::new(),
        };

        // Define how "Survive" task decomposes into actions
        planner.add_method(
            CompoundTask::Survive,
            vec![
                Task::Primitive(PrimitiveTask::GatherResources),
                Task::Primitive(PrimitiveTask::BuildStructure),
            ],
        );

        // Define how "DefendBase" task decomposes into actions
        planner.add_method(
            CompoundTask::DefendBase,
            vec![
                Task::Primitive(PrimitiveTask::GatherResources),
                Task::Primitive(PrimitiveTask::AttackEnemy),
            ],
        );

        planner
    }

    /// Adds a method for decomposing a compound task
    fn add_method(&mut self, task: CompoundTask, subtasks: Vec<Task>) {
        let method = Method {
            task: task.clone(),
            subtasks,
        };
        self.methods.entry(task).or_insert_with(Vec::new).push(method);
    }

    /// Decomposes a task into executable steps, reevaluating dynamically
    pub fn decompose(&self, task: &Task, world_state: &WorldState) -> Vec<PrimitiveTask> {
        let mut plan = Vec::new();
        self.decompose_recursive(task, &mut plan, world_state);
        plan
    }

    fn decompose_recursive(&self, task: &Task, plan: &mut Vec<PrimitiveTask>, world_state: &WorldState) {
        match task {
            Task::Primitive(action) => {
                if world_state.is_action_valid(*action) {
                    plan.push(*action);
                }
            }
            Task::Compound(compound) => {
                if let Some(methods) = self.methods.get(compound) {
                    for method in methods {
                        let mut temp_plan = Vec::new();
                        for subtask in &method.subtasks {
                            self.decompose_recursive(subtask, &mut temp_plan, world_state);
                        }
                        if !temp_plan.is_empty() {
                            plan.extend(temp_plan);
                            break;
                        }
                    }
                }
            }
        }
    }
}
