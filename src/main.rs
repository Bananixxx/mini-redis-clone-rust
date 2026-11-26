pub mod database;
pub mod entry;
pub mod server;

use crate::database::Database;
use crate::entry::display_entry;
use crate::server::start_server;


fn main() {
    let mut database = database::Database::new();

    Database::set(&mut database, "greeting".to_string(), "Hello, World!".to_string(), 60);
    if let Some(entry) = Database::get(&database, "greeting") {
        display_entry(entry);
    }

    start_server();
}


