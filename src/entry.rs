
pub struct Entry {
    value: String,
    ttl: u32,
}

pub fn create_entry(value: String, ttl: u32) -> Entry {
    Entry { value, ttl }
}

pub fn update_entry(entry: &mut Entry, new_value: String, new_ttl: u32) {
    entry.value = new_value;
    entry.ttl = new_ttl;
}

pub fn display_entry(entry: &Entry) {
    println!("Entry value: {}", entry.value);
    println!("Entry TTL: {}", entry.ttl);
}

pub fn delete_entry(entry: &mut Option<Entry>) {
    *entry = None;
}