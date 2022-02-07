use std::{fmt::format, collections::HashMap};




fn main() {

    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();
    println!("The key is {} and the value is {}", key, value);

    let contents = format!("{{{}: {}}} \n", key, value);
    std::fs::write("kv.db", contents).unwrap();
    
    
    
    // let write_result = std::fs::write("kv.db", contents).unwrap();

    // match write_result {
    //     Ok(()) => {

    //     }
    //     Err(e) => {

    //     }
    // }
    let database = Database::new();
}
/*
create_struct with its fields
*/
struct Database {
    map: std::collections::HashMap<String, String>,
}
// the implementation of the struct
impl Database {
    fn new() -> Database {
        Database {
            map: HashMap::new(),
        }
    }
}
