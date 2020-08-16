use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    // cargo run hello John -> ["target/debug/hello", "hello", "John"]

    let command = args[1].clone();
    println!("Command: {}", command);

    let name = "Brad";
    let status = "100%";

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("Not valid command.")
    }
}