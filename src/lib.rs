//use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Zone {
    name: String,
}

impl Zone {
    pub fn new(name: &str) -> Zone {
        println!("New bleh");
        Zone {
            name: name.to_string(),
        }
    }
}
/*
#[derive(Debug)]
struct Record {
    id: i32,
    zone_id: i32,
    hostname: String,
    ip: String,
}
*/
