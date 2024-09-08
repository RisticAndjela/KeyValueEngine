use std::{fs, io};
use serde_json::Value;
use crate::external_configuration::default_constants::take_from_json;
use crate::representation_of_program_as_structure::Program;

mod get;
mod put;
mod delete;
mod representation_of_program_as_structure;
mod script;

mod external_configuration { pub mod default_constants; }

pub const CONFIGURATION_PATH:&str= "startup/startup_app/src/external_configuration/config.json";

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
    let mut this=Program::new();
    if resume.to_lowercase()=="yes"{

    }
    else{
        this.script();
    }


    loop{
        println!("\n \n{}",take_from_json("text","use").unwrap().as_str().unwrap());
        let response=take_input("");
        if response.to_lowercase()=="exit"{break;}

    }

}
