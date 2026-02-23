use std::collections::HashMap;

use uavred_red::domain::TargetScope;

pub trait ScopeRepository {
    fn save(&mut self, scope: TargetScope);
    fn load(&self, engagement_id: &str) -> Option<TargetScope>;
}

#[derive(Default)]
pub struct InMemoryScopeRepository {
    data: HashMap<String, TargetScope>,
}

impl ScopeRepository for InMemoryScopeRepository {
    fn save(&mut self, scope: TargetScope) {
        self.data.insert(scope.engagement_id.clone(), scope);
    }

    fn load(&self, engagement_id: &str) -> Option<TargetScope> {
        self.data.get(engagement_id).cloned()
    }
}
