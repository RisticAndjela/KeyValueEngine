use std::{fs, io};

mod write_ahead_log;

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::fs;
    use std::ops::Add;
    use std::time::{SystemTime, UNIX_EPOCH};
    use entry_element::entry_element::EntryElement;
    use crate::remove_all_files;
    use crate::write_ahead_log::WriteAheadLog;

    #[test]
    #[serial]
    fn indexing(){
        remove_all_files().expect("cannot remove all in indexing");
        let mut wal=WriteAheadLog::new(String::new().add("src/storage"),1024);
        wal.add_new_file();
        for _ in 0..8{wal.add_new_file();}
        let files=wal.get_all_files();
        let count = fs::read_dir("src/storage").unwrap().filter_map(Result::ok).count();
        assert_eq!(files.len(),count);
        wal.remove_file(3);
        let files_removed_one=wal.get_all_files();
        assert_eq!(files_removed_one.len(),count-1);
    }

    #[test]
    #[serial]
    fn add_and_read_at_certain_position(){
        remove_all_files().expect("cannot remove all in add and read");
        let mut wal=WriteAheadLog::new("src/storage".to_string(),120);
        let element1=EntryElement::new("key1".to_string(),"hi hello whats up".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 );
        wal.add_element(&element1);
        let element2=EntryElement::new("key2".to_string(),"hi hello whats up".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 );
        wal.add_element(&element2);
        let element3=EntryElement::new("key3".to_string(),"hi hello whats up".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 );
        wal.add_element(&element3);
        let element4=EntryElement::new("key4".to_string(),"hi hello whats up".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 );
        wal.add_element(&element4);
        let element5=EntryElement::new("key5".to_string(),"hi hello whats up".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 );
        wal.add_element(&element5);
        let element6=EntryElement::new("key6".to_string(),"hi hello whats up".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 );
        wal.add_element(&element6);
        let element7=EntryElement::new("key7".to_string(),"hi hello whats up".as_bytes().to_vec(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 );
        wal.add_element(&element7);
        //one element size is 50B and because one segment can hold 100B I need to have 4 files for 7 elements
        let count = fs::read_dir("src/storage").unwrap().filter_map(Result::ok).count();
        assert_eq!(count,4);

        let read_element=wal.read_element_at("src/storage/wal_0001.txt",0);
        assert_eq!(read_element.unwrap(),element1);

        for _ in 0..9{
            let entry=wal.read_where_stopped();
            println!("seek to :{}",wal.offset);
            println!("{:?}",entry);
        }
        wal.add_element(&element5);
        for _ in 0..9{
            let entry=wal.read_where_stopped();
            println!("seek to :{}",wal.offset);
            println!("{:?}",entry);
        }
    }
}


pub fn remove_all_files() -> io::Result<()> {
    if let Ok(entries) = fs::read_dir("src/storage") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if path.exists() {
                        fs::remove_file(&path)?;
                    }
                }
            }
        }
    }
    Ok(())
}

