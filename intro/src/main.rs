#![allow(dead_code, unused_variables)]
use std::{collections::HashMap, fs::File, path::PathBuf};

fn main() {
    let mut arguments = std::env::args().skip(1);
    let path = PathBuf::from("kv.db");

    let key = arguments.next().unwrap();
    let value = arguments.next().unwrap();

    println!("Got key {} and value {}", key, value);
    let contents = format!("{}={}", key, value);

    std::fs::write("kv.db", contents).unwrap();

    let mut database = Database::new(path).expect("Creating the database failed");
    database.insert(&key, &value);
    database.insert(&String::from("hello"), &String::from("world"));
    database.flush().unwrap()
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new(path: PathBuf) -> Result<Self, std::io::Error> {
        if !path.exists() {
            File::create(path)?;
        }

        let contents = std::fs::read_to_string("kv.db")?;
        let mut map: HashMap<String, String> = HashMap::new();

        for line in contents.lines() {
            let chunks: Vec<&str> = line.splitn(2, '=').collect();
            let key = chunks[0];
            let value = chunks[1];
            map.insert(chunks[0].to_owned(), chunks[1].to_owned());
        }

        Ok(Database { map })
    }

    fn insert(&mut self, key: &str, value: &str) -> () {
        self.map.insert(key.to_owned(), value.to_owned());
    }

    fn flush(&self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('=');
            contents.push_str(value);
            contents.push('\n');
        }
        std::fs::write("kv.db", contents)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        self.flush().unwrap()
    }
}
