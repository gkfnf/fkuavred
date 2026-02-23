use uavred_logic::{TargetScope, TaskPlanner};

pub fn launch<P: TaskPlanner>(planner: &P) -> usize {
    let scope = TargetScope::new("demo-engagement", vec!["127.0.0.1".to_string()]);
    planner.plan(&scope).len()
}
