use std::collections::VecDeque;

#[derive(Default)]
pub struct SessionMemory {
    entries: VecDeque<String>,
}

impl SessionMemory {
    pub fn push(&mut self, note: impl Into<String>) {
        self.entries.push_back(note.into());
    }

    pub fn recent(&self, limit: usize) -> Vec<String> {
        self.entries.iter().rev().take(limit).cloned().collect()
    }
}
