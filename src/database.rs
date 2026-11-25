use std::collections::HashMap;
use crate::entry;

pub struct Database {
    entries: HashMap<String, entry::Entry>,
}

impl Database {
    pub fn new() -> Self {
        Database { entries: HashMap::new() }
    }

    pub fn set(&mut self, key: String, value: String, ttl: u32) {
        let entry = entry::create_entry(value, ttl);
        self.entries.insert(key, entry);
    }

    pub fn get(&self, key: &str) -> Option<&entry::Entry> {
        self.entries.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.entries.remove(key);
    }

    pub fn exists(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }   

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}