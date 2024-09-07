use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Add;
use std::path::Path;

pub fn get_key_and_position(loaded_data:Vec<u8>) -> (String, u64) {
    if loaded_data.len() < 16 { panic!("loaded_data is too short to contain a record."); }

    let key_size_bytes: [u8; 8] = loaded_data[0..8].try_into().expect("Slice with incorrect length");
    let key_size: u64 = u64::from_be_bytes(key_size_bytes);
    let key_start = 8;
    let key_end = key_start + key_size as usize;

    if loaded_data.len() < key_end + 8 {panic!("loaded_data is too short to contain the complete record."); }

    let key_bytes = &loaded_data[key_start..key_end];
    let key = String::from_utf8(key_bytes.to_vec()).expect("Failed to convert key to String");

    let position_start = key_end;
    let position_bytes: [u8; 8] = loaded_data[position_start..position_start + 8].try_into().expect("Slice with incorrect length");
    let position = u64::from_be_bytes(position_bytes);

    (key, position)
}
pub fn get_name(storage_path:String) -> String {
    let binding = storage_path.clone();
    let path = Path::new(&binding);
    let ignore=9-1; //"sstable_".len=9
    let mut last_index=0;

    if path.is_dir() {
        for entry in fs::read_dir(path).expect("Failed to read directory") {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let ind=file_name.to_string_lossy()[ignore..].to_string().parse::<i32>();
                if ind.clone().unwrap()>last_index{last_index=ind.clone().unwrap();}
            }
        }
    }

    let mut new_filename=String::new().add(storage_path.as_str());
    new_filename.push_str("/sstable_");
    let index=last_index+1;
    match index.to_string().len() {
        4 => new_filename.push_str(&index.to_string()),
        3 => { new_filename.push_str("0");
            new_filename.push_str(&index.to_string()); },
        2 => { new_filename.push_str("00");
            new_filename.push_str(&index.to_string()); },
        1 => { new_filename.push_str("000");
            new_filename.push_str(&index.to_string()); },
        _ => {panic!("no more space")}
    }
    return new_filename;
}
