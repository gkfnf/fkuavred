pub mod domain {
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct TargetScope {
        pub engagement_id: String,
        pub targets: Vec<String>,
    }

    impl TargetScope {
        pub fn new(engagement_id: impl Into<String>, targets: Vec<String>) -> Self {
            Self {
                engagement_id: engagement_id.into(),
                targets,
            }
        }
    }
}
