use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("Key is not provided");
    let value = arguments.next().expect("Value is not provided");
    println!("key is {} and value is {}", key, value);

    let mut database = Database::new().expect("Failed to create database");

    database.insert(key, value);
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let file_content: String = std::fs::read_to_string("kv.db")?;
        // let contents = std::fs::read_to_string("kv.db");
        for line in file_content.lines() {
            let (key, value) = line
                .split_once('\t')
                .expect("line.split_once('\t') Crashed");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }
        let _ = std::fs::write("kv.db", contents);
    }
}
