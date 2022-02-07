use std::{collections::HashMap};

fn main() {

    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("\n\t[Err! No key found!\n ");
    let value = arguments.next().unwrap();
    println!("  New Key Compared with Database...\n  Key value updated successfully -- \n\tKey:\t {} \n\tValue:\t {} \n", key, value);
    let mut database = Database::new().expect("\n\t[Err! Database::new() -> creating database failed]\n  ");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    database.flush_meth().unwrap();
    }

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {

    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("key_value_storage.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("\n\t[Err! No Key!]\n ");
            let value = chunks.next().expect("\n\t[Err! No Value!]\n ");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map, flush: false })
    }
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);

    }
    fn flush_meth(&mut self) -> std::io::Result<()> {
        self.flush = true;
        flush_func(&self)
    }
}

impl Drop for Database {

    fn drop(&mut self) {
        if !self.flush {
            let _ = flush_func(self);
        } 
    }
}

fn flush_func(database: &Database) -> std::io::Result<()> {
    println!("  Database was flushed -- \n");
    let mut string_buffer = String::new();
    for (key, value) in &database.map {
            string_buffer.push_str(key);
            string_buffer.push('\t');
            string_buffer.push_str(&value);
            string_buffer.push('\n');
    }
    std::fs::write("key_value_storage.db", string_buffer)
}