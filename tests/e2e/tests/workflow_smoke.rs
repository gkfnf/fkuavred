use uavred_infra::NoopPlanner;

#[test]
fn desktop_flow_smoke_test() {
    let planner = NoopPlanner::default();
    let planned = uavred_ui::launch(&planner);
    assert!(planned > 0);
}
