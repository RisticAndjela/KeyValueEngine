use std::{fs, io};
use serde_json::Value;

mod get;
mod put;
mod delete;

pub const CONFIGURATION_PATH:&str= "startup/startup_app/src/external_configuration/config.json";
fn take_from_json(object: &str, key: &str)-> Option<Value>{
    let data = fs::read_to_string(CONFIGURATION_PATH.to_string()).expect("Unable to read file");
    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    json.get(object).and_then(|obj| obj.get(key)).cloned()
}
fn initialize_program(){

}
fn resume_program(){

}
fn take_input(msg:&str)->String{
    let mut input = String::new();
    println!("{msg}");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();
    println!("You entered: {}", input);
    input.to_string()
}
fn main() {
    println!("{}\n \n{}\n \n{}\n \nCONFIGURATION FILE:\n{}",take_from_json("text","welcoming").unwrap().as_str().unwrap(),
             take_from_json("text","description").unwrap().as_str().unwrap(),
             take_from_json("text","options").unwrap().as_str().unwrap(),
             take_from_json("text","config_file").unwrap().as_str().unwrap());

    println!("\nYou have options to continue with previously saved data, or to initialize scripts again.\nWould you like to resume? [YES/NO] ",);
    let resume=take_input("");
    if resume.to_lowercase()=="yes"{
        resume_program();
    }
    else{
        initialize_program();
    }
    loop{
        println!("\n \n{}",take_from_json("text","use").unwrap().as_str().unwrap());
        let response=take_input("");
        if response.to_lowercase()=="exit"{break;}

    }

}
