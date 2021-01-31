

use std::{collections::HashMap};
use std::io::{stdin, stdout, Write};


fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn show_help(bad_command: &str) {
    println!("Did not understand your command! Got : `{}`.", bad_command);
    println!("Use either `get <k>`, `set <k> <v>` or `del <k>`, `exit` to stop the program");
}

struct Database {
    data: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        Database{
            data : HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value:String) {
        &self.data.insert(key, value);
    }

    fn get(&mut self, key: String) -> Option<&String>{
        self.data.get(&key)
    }
}

fn execute(command: String, db : &mut Database) -> bool {
    match command.split(" ").nth(0).expect("did not find first argument") {
        "get" => {
            // get the key that comes after get
            let maybe_key = command.split(" ").nth(1);
            // check if the key is specified
            match maybe_key {
                // if a key is specified we use it to query the db
                Some(key) => {
                    // if the db finds a value it prints it to stdout
                    if let Some(value) =  db.get(key.to_string()) {
                        println!("{}", value);
                    } else {
                        // otherwise stdout key not found 
                        println!("key : `{}` not found", key);
                    }
                },
                _ => println!("specify the key when using get") // remind `get` usage 
            }
            false
        },
        "exit" => true,
        _ => {
            show_help(&command);
            false
        },
    }
}

fn main() {
    let mut db = Database::new();
    loop {
        let mut cmd = String::new();
        read(&mut cmd); // write to 
        cmd = cmd.as_str().trim().to_string(); // trim command
        let stop  = execute(cmd, &mut db);
        if stop { break; };
    }
}
