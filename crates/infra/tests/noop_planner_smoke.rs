use uavred_infra::NoopPlanner;
use uavred_logic::{TargetScope, TaskPlanner};

#[test]
fn noop_planner_respects_scope_validation() {
    let planner = NoopPlanner::default();
    let scope = TargetScope::new("eng-003", vec!["127.0.0.1".into()]);
    assert_eq!(planner.plan(&scope).len(), 1);
}
