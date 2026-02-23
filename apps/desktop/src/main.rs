use uavred_infra::NoopPlanner;

fn main() {
    let planner = NoopPlanner::default();
    let task_count = uavred_ui::launch(&planner);
    println!("uavred desktop bootstrapped with {task_count} planned task(s)");
}
