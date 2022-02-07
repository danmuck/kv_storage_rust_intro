use std::{fmt::format, collections::HashMap};

fn main() {

    // skips the executable binary to the following commands
    let mut arguments = std::env::args().skip(1);
    // arguments.next() returns an OPTIONAL string, if no arguments passed we will return Err! because in rust there is no Null
    let key = arguments.next().expect("\n\t[Err! No key found!\n ");
    let value = arguments.next().unwrap();
    println!("The key is {} and the value is {}", key, value);

    // format the key/value as String and write to file
    // let contents = format!("{} \t{} \n", key, value);
    // std::fs::write("kv.db", contents).unwrap();
    
    let database = Database::new().expect("\n\t[Err! Database::new() -> crashed]\n  ");
    // database.insert(key, value);  //syntactic sugar for below line
    // Database::insert(database, key, value);
    database.insert(key, value);


    // long way to write it
    // let write_result = std::fs::write("kv.db", contents).unwrap();
    // match write_result {
    //     Ok(()) => {
    //     }
    //     Err(e) => {
    //     }
    // }
}

// create_struct with its fields
struct Database {
    map: HashMap<String, String>,
}

// the implementation of the struct
impl Database {
    fn new() -> Result<Database, std::io::Error> {
        /*
            Take a string from the file,
                take each line which contains (key, value) and split them into respective variables
        */
        // 1. read kv.db file 
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(content) => content,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        let mut map = HashMap::new();
                                //  ?  means if there is an error, 
                               //  an error will be returned from the function
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            // let (key, value) = line.split_once('\t').expect("Corrupt database");
            // let pair = line.split_once('\t').expect("\n\t[Err! Database -> corrupt]\n ");
            // split at the '\t' character into 2 chunks (which are also iterators)
            let mut map = HashMap::new();
            let mut chunks = line.splitn(2, '\t');
            // crash the program if there is no key or value
            let key = chunks.next().expect("\n\t[Err! No Key!]\n ");
            let value = chunks.next().expect("\n\t[Err! No Value!]\n ");
            // with .insert() if key already exists, returns its value 
            // .to_owned() to pass the value/memory to the parent so it can leave the function scope
            //      copy the memory to the views owner
            // there is also a .clone() method to duplicate the view
            // there is also a .to_string() to convert the &str (string pointer) to the string value
            //          **may be less performant (not confirmed)
            // .insert() to insert our values into the map
            map.insert(key.to_owned(), value.to_owned());

            // at this point, contents will go out of scope and its ownership will be dropped

        }
        // 2. parse the string 
        // 3. populate our map
        Ok(Database { map: map })
    }
    // METHODS are differentiated from FUNCTIONS with passing SELF
    fn insert(mut self, key: String, value: String) {
        self.map.insert(key, value);

    }
}
