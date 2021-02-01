

use std::collections::HashMap;
use std::io::{self, Write, Read, stdin, stdout};

fn help(bad_command: &str) {
    println!("Did not understand your command! Got : `{}`.", bad_command);
    println!("Use either `get <k>`, `set <k> <v>` or `del <k>`");
    println!("Type `exit` to stop the program");
}

struct ParserRes<'a> {
    str: String,
    action: &'a str,
    key: &'a str,
    value: &'a str,
}

fn parser<'a>(arguments: &'a String) -> ParserRes<'a> {
    let mut args = arguments.clone().split(" ");
    let maybe_action = args.next();
    let maybe_key = args.next();
    let maybe_value = args.next();

    let action: &'a str = maybe_action.unwrap_or("");
    let key: &'a str  = maybe_key.unwrap_or("");
    let value: &'a str  = maybe_value.unwrap_or("");

    ParserRes{
        str : *arguments,
        action :action,
        key : key,
        value: value
    }
}

fn main() {
    let mut db: HashMap<&str, &str> = HashMap::new();
    let mut cin = io::stdin();
    loop {
        let mut input = "".to_string();
        cin.read_to_string(&mut input).unwrap().to_string(); // write to cmd string 
        let args= parser(&input);
        match args.action {
            "set" => {
                match args.key {
                    "" => println!("provide a key and value to set"),
                    _ => (),
                }
                db.insert(args.key, args.value);
            },
            "get" => {
                let value = db.get(args.key);
                match value {
                    Some(value) => println!("> {}", value),
                    _ => println!("> None")
                };
            }
            _ => ()
        }
    }
}
