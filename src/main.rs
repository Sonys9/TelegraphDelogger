use std::io::stdin;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use serde_json::{Result, Value};

fn open_from_file() -> HashMap<String, String> {
    let file = File::open("todo.json");

    if let Ok(mut f) = file {
        let mut content = String::new();
        f.read_to_string(&mut content).expect("Cant read the file!");
        let v = serde_json::from_str(content.as_str()).expect("Cant parse the file!");
        return v;
    }

    let mut f = File::create("todo.json").expect("Cant create the file!");
    f.write_all(b"{}").expect("Cant write in the file!");
    HashMap::new()
}

fn get_todos() {
    let mut dict: HashMap<String, String> = open_from_file();
    for (name, value) in dict {
        println!("[{value}] {name}");
    };
}

fn update_todo(dict: &HashMap<String, String>)  {
    let mut f = File::create("todo.json").expect("Cant create the file!");
    let in_str_data = serde_json::to_string(&dict).expect("Cant parse JSON data!");
    f.write_all(in_str_data.as_bytes()).expect("Cant write in the file!");
}

//#[tokio::main]
fn main() {
    
    get_todos();

    println!("Simple TODO in Rust!");
    loop {
        let mut dict: HashMap<String, String> = open_from_file();

        println!("[1] Add new target\n[2] Delete old target\n[3] Show TODO\n[4] Change TODO status\nEnter choice number:\t");
        let mut user_input = String::new();
        let _ = stdin().read_line(&mut user_input);

        if user_input.trim() == String::from("1") {
            println!("Enter TODO:\t");
            let mut user_input = String::new();
            let _ = stdin().read_line(&mut user_input);
            if user_input.trim() != "" {
                dict.insert(user_input.trim().to_string(), "Undone".to_string());
            };
            update_todo(&dict);
        } else if user_input.trim() == String::from("2") {
            println!("Enter TODO:\t");
            let mut user_input = String::new();
            let _ = stdin().read_line(&mut user_input);
            if user_input.trim() != "" {
                match &dict.contains_key(user_input.trim()) {
                    true => {
                        &dict.remove(user_input.trim());
                        println!("Removed successful");
                    },
                    false => println!("Element is not found")
                };
            };
            update_todo(&dict);
        } else if user_input.trim() == String::from("3") {
            for (name, status) in &dict {
                println!("[{}] {}", status, name);
            }
        } else if user_input.trim() == String::from("4") {
            println!("Enter TODO:\t");
            let mut user_input = String::new();
            let _ = stdin().read_line(&mut user_input);
            if user_input.trim() != "" {
                match &dict.contains_key(user_input.trim()) {
                    true => {
                        if dict[user_input.trim()] == "Done" {
                            dict.insert(user_input.trim().to_string(), "Undone".to_string());
                        } else {
                            dict.insert(user_input.trim().to_string(), "Done".to_string());
                        }
                    },
                    false => println!("Element is not found")
                };
            }; 
            update_todo(&dict);
        };
    }
}
