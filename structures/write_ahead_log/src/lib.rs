use std::{fs, io};

mod write_ahead_log;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::ops::Add;
    use std::time::{SystemTime, UNIX_EPOCH};
    use memtable_element::entry_element::EntryElement;
    use crate::remove_all;
    use crate::write_ahead_log::WriteAheadLog;

    #[test]
    fn indexing(){
        remove_all().expect("cannot remove all");
        let mut wal=WriteAheadLog::new(1024);
        wal.add_new_file();
        assert_eq!(wal.filename,"src/storage/wal_0002.txt");
        for _ in 0..8{wal.add_new_file();}
        let files=wal.get_all_files();
        let count = fs::read_dir("src/storage").unwrap().filter_map(Result::ok).count();
        assert_eq!(files.len(),count);
        wal.remove_file(3);
        let files_removed_one=wal.get_all_files();
        assert_eq!(files_removed_one.len(),count-1);
    }

    #[test]
    fn add(){
        remove_all().expect("cannot remove all");
        let mut wal=WriteAheadLog::new(120);
        let element=EntryElement{key:String::new().add("key1"),value:"hi hello whats up".as_bytes().to_vec(),tombstone:false,timestamp:SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64 };
        wal.add_element(&element);
        wal.add_element(&element);
        wal.add_element(&element);
        wal.add_element(&element);
        wal.add_element(&element);
        wal.add_element(&element);
        wal.add_element(&element);
        //one element size is 50B and because one segment can hold 100B i need to have 4 files for 7 elements
        let count = fs::read_dir("src/storage").unwrap().filter_map(Result::ok).count();
        assert_eq!(count,4);

    }
}


pub fn remove_all() -> io::Result<()> {
    let entries = fs::read_dir("src/storage")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}