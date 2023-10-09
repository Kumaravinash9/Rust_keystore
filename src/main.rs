use std::{collections::HashMap, env, fs::File, io::Write};

fn main() {
    let mut args = env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    println!("key is {} --- > value is {}", key, value);
    //let database = Database::create(&key, &value);
    //let datahash = DataHash::new().expect(msg);
    let path = std::path::PathBuf::from("data.db").exists();
    let contents = format!("{} : {}\n", key, value);
    println!("{}", path);
    if !path {
        let mut f = std::fs::File::create("data.db");
        let result = f.expect("file will be created").write(contents.as_bytes());
        match result {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err)
            }
        }
    } else {
        let f = File::options().append(true).open("data.db");
        let mut datahash = DataHash::new().expect("map data has been initialized");
        datahash.insert(&key.to_uppercase(), &value);
        datahash.insert(key.as_str(), value.as_str());
        datahash.flush().expect("something wrong happendd");
        let result = f
            .expect("inserting data into db")
            .write(contents.as_bytes());
        match result {
            Ok(_) => {}
            Err(err) => {
                println!("{}", err)
            }
        }
    }
    /*
    let contents = Database::content(&database);
    let f = File::options().append(true).open("data.db");
    let result = f
        .expect("key value pair has been added")
        .write(contents.as_bytes());
    match result {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e)
        }
    }*/

    println!("data has been inserted! ");
}

struct Database {
    key: String,
    value: String,
}

impl Database {
    fn create(key: &str, value: &str) -> Self {
        Database {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
    fn content(database: &Database) -> String {
        return format!("{}:{}\n", database.key, database.value);
    }
}

struct DataHash {
    map: HashMap<String, String>,
    flush: bool,
}

impl DataHash {
    fn new() -> Result<DataHash, std::io::Error> {
        // same implementation as below
        /*let contents =  match std::fs::read_to_string("data.db") {
           Ok(c) => c,
           Err(e) => {
                return Err(e);
           }
        };*/

        let _contents = std::fs::read_to_string("data.db")?;
        let mut map = HashMap::new();
        for line in _contents.lines() {
            //let (key, value) = line.split_once(":").expect("corrupt database");
            let mut chunks = line.splitn(2, ':');
            let key = chunks.next().expect("no keys");
            let value = chunks.next().expect("no value exits");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(DataHash {
            map: map,
            flush: false,
        })
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(self)
    }
}

impl Drop for DataHash {
    fn drop(&mut self) {
        if !self.flush {
            do_flush(self);
        }
    }
}

fn do_flush(dataHash: &DataHash) -> std::io::Result<()> {
    let mut contents = String::new();
    for pairs in &dataHash.map {
        let keyvaluePairs = format!("{}:{}\n", pairs.0, pairs.1);
        contents.push_str(&keyvaluePairs);
    }
    std::fs::write("data.db", contents);
    Ok(())
}
