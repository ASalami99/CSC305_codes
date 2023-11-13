mod greetings;
use greetings::{english, french, spanish};

extern crate hello_world_lib;
//use greetings::default_greeting;
//use greetings::default_greeting2;
//use greetings::*;   --This imports all the functions

fn main() {
    //let mut greeting: &str = "Hello there boy!";
    let greeting = "Hello there again";
    println!("{}", greeting);

    println!("Hello, world!");
    // println!("{}", default_greeting());
    // println!("{}", default_greeting2());
    // println!("My first greeting is '{}' and the second is '{}'", default_greeting(), default_greeting2());
    println!(
        "Hello in spanish is {}, in french is {} and in english is {}.",
        spanish::default_greeting(),
        french::default_greeting(),
        english::default_greeting()
    );
    println!("{}", hello_world_lib::greeting_from_lib());
}
