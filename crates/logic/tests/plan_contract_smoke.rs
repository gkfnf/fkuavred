use uavred_logic::{scope_is_valid, TargetScope, TaskPlanner};

struct StaticPlanner;

impl TaskPlanner for StaticPlanner {
    fn plan(&self, scope: &TargetScope) -> Vec<String> {
        if scope_is_valid(scope) {
            vec!["discovery".to_string()]
        } else {
            vec![]
        }
    }
}

#[test]
fn planner_returns_task_for_valid_scope() {
    let planner = StaticPlanner;
    let scope = TargetScope::new("eng-002", vec!["192.168.1.20".into()]);
    assert_eq!(planner.plan(&scope).len(), 1);
}
