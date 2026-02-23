use uavred_logic::{TargetScope, TaskPlanner};

struct MockPlanner;

impl TaskPlanner for MockPlanner {
    fn plan(&self, _scope: &TargetScope) -> Vec<String> {
        vec!["task-a".to_string(), "task-b".to_string()]
    }
}

#[test]
fn ui_launch_returns_planned_task_count() {
    assert_eq!(uavred_ui::launch(&MockPlanner), 2);
}
