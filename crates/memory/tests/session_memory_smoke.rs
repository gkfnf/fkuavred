use uavred_memory::SessionMemory;

#[test]
fn memory_returns_recent_entries() {
    let mut memory = SessionMemory::default();
    memory.push("first");
    memory.push("second");
    assert_eq!(memory.recent(1), vec!["second".to_string()]);
}
