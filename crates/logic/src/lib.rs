pub use uavred_red::domain::TargetScope;

pub trait TaskPlanner {
    fn plan(&self, scope: &TargetScope) -> Vec<String>;
}

pub fn scope_is_valid(scope: &TargetScope) -> bool {
    !scope.targets.is_empty()
}
