use uavred_red::domain::TargetScope;

#[test]
fn target_scope_keeps_engagement_and_targets() {
    let scope = TargetScope::new("eng-001", vec!["10.0.0.1".into()]);
    assert_eq!(scope.engagement_id, "eng-001");
    assert_eq!(scope.targets.len(), 1);
}
