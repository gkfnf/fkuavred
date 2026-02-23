use uavred_logic::{scope_is_valid, TargetScope, TaskPlanner};

#[derive(Default)]
pub struct NoopPlanner;

impl TaskPlanner for NoopPlanner {
    fn plan(&self, scope: &TargetScope) -> Vec<String> {
        if scope_is_valid(scope) {
            vec!["noop-discovery".to_string()]
        } else {
            vec![]
        }
    }
}
