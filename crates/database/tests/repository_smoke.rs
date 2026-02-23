use uavred_database::{InMemoryScopeRepository, ScopeRepository};
use uavred_red::domain::TargetScope;

#[test]
fn repository_can_round_trip_scope() {
    let mut repo = InMemoryScopeRepository::default();
    repo.save(TargetScope::new("eng-004", vec!["drone.local".into()]));
    assert!(repo.load("eng-004").is_some());
}
