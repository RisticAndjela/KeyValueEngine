use std::{fs, io};
use entry_element::entry_element::EntryElement;
use crate::write_ahead_log::WriteAheadLog;

pub mod write_ahead_log;
pub const DIR_PATH:&str="src/storage";
#[cfg(test)]
mod tests {
    use serial_test::serial;
    use entry_element::entry_element::EntryElement;
    use crate::{get_wal, remove_all_files};
    #[test]
    #[serial]
    fn test_add(){
        remove_all_files().expect("could not erase files");
        let mut wal=get_wal(4);
        println!("{:?}",wal.get_by_offset(0)); //13
        println!("{:?}",wal.offset);
        println!("{:?}",wal.get_where_stopped()); //14
        println!("{:?}",wal.get_where_stopped()); //15
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_by_offset(205)); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19 on 295?
        println!("{:?}",wal.get_where_stopped()); //13
        println!("{:?}",wal.get_where_stopped()); //14
        println!("{:?}",wal.get_where_stopped()); //15
        println!("{:?}",wal.get_where_stopped()); //16
        let record20 = EntryElement::new("key20".to_string(),vec![1,2,3],123456789);
        wal.add_record(record20.clone());
        println!("\n\nNEW ROUND ++1\n\n");
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        let record21 = EntryElement::new("key21".to_string(),vec![1,2,3],123456789);
        wal.add_record(record21.clone());
        println!("\n\nNEW ROUND ++1\n\n");
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none

        let record22 = EntryElement::new("key22".to_string(),vec![1,2,3],123456789);
        wal.add_record(record22.clone());
        println!("\n\nNEW ROUND ++1\n\n");
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none

    let record23 = EntryElement::new("key23".to_string(),vec![1,2,3],123456789);
        wal.add_record(record23.clone());
        println!("\n\nNEW ROUND ++1\n\n");
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none

        let record24 = EntryElement::new("key24".to_string(),vec![1,2,3],123456789);
        wal.add_record(record24.clone());
        println!("\n\nNEW ROUND ++1\n\n");
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none

        }
    #[test]
    #[serial]
    fn test_add1(){
        remove_all_files().expect("could not erase files");
        let mut wal=get_wal(10);
        println!("{:?}",wal.get_by_offset(0)); //13
        println!("{:?}",wal.offset);
        println!("{:?}",wal.get_where_stopped()); //14
        println!("{:?}",wal.get_where_stopped()); //15
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19 on 295?
        println!("{:?}",wal.get_where_stopped()); //13
        println!("{:?}",wal.get_where_stopped()); //14
        println!("{:?}",wal.get_where_stopped()); //15
        println!("{:?}",wal.get_where_stopped()); //16
        let record20 = EntryElement::new("key20".to_string(),vec![1,2,3],123456789);
        wal.add_record(record20.clone());
        println!("\n\nNEW ROUND ++1\n\n");
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        let record21 = EntryElement::new("key21".to_string(),vec![1,2,3],123456789);
        wal.add_record(record21.clone());
        println!("\n\nNEW ROUND ++1\n\n");
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none

        let record22 = EntryElement::new("key22".to_string(),vec![1,2,3],123456789);
        wal.add_record(record22.clone());
        println!("\n\nNEW ROUND ++1\n\n");
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none

    let record23 = EntryElement::new("key23".to_string(),vec![1,2,3],123456789);
        wal.add_record(record23.clone());
        println!("\n\nNEW ROUND ++1\n\n");
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none

        let record24 = EntryElement::new("key24".to_string(),vec![1,2,3],123456789);
        wal.add_record(record24.clone());
        println!("\n\nNEW ROUND ++1\n\n");
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none
        println!("{:?}",wal.get_where_stopped()); //16
        println!("{:?}",wal.get_where_stopped()); //17
        println!("{:?}",wal.get_where_stopped()); //18
        println!("{:?}",wal.get_where_stopped()); //19
        println!("{:?}",wal.get_where_stopped()); //20
        println!("{:?}",wal.get_where_stopped()); //none

        }

}

pub fn get_wal(a:u64)->WriteAheadLog{
    let mut wal = WriteAheadLog::new(DIR_PATH.to_string(), 102, a);

    let record1 = EntryElement::new("key1".to_string(),vec![1,2,3],123456789);
    wal.add_record(record1.clone());
    let record2 = EntryElement::new("key2".to_string(),vec![1,2,3],123456789);
    wal.add_record(record2.clone());
    let record3 = EntryElement::new("key3".to_string(),vec![1,2,3],123456789);
    wal.add_record(record3.clone());
    let record4 = EntryElement::new("key4".to_string(),vec![1,2,3],123456789);
    wal.add_record(record4.clone());
    let record5 = EntryElement::new("key5".to_string(),vec![1,2,3],123456789);
    wal.add_record(record5.clone());
    let record6 = EntryElement::new("key6".to_string(),vec![1,2,3],123456789);
    wal.add_record(record6.clone());
    let record7 = EntryElement::new("key7".to_string(),vec![1,2,3],123456789);
    wal.add_record(record7.clone());
    let record8 = EntryElement::new("key8".to_string(),vec![1,2,3],123456789);
    wal.add_record(record8.clone());
    let record9 = EntryElement::new("key9".to_string(),vec![1,2,3],123456789);
    wal.add_record(record9.clone());
    let record10 = EntryElement::new("key10".to_string(),vec![1,2,3],123456789);
    wal.add_record(record10.clone());
    let record11 = EntryElement::new("key11".to_string(),vec![1,2,3],123456789);
    wal.add_record(record11.clone());
    let record12 = EntryElement::new("key12".to_string(),vec![1,2,3],123456789);
    wal.add_record(record12.clone());
    let record13 = EntryElement::new("key13".to_string(),vec![1,2,3],123456789);
    wal.add_record(record13.clone());
    let record14 = EntryElement::new("key14".to_string(),vec![1,2,3],123456789);
    wal.add_record(record14.clone());
    let record15 = EntryElement::new("key15".to_string(),vec![1,2,3],123456789);
    wal.add_record(record15.clone());
    let record16 = EntryElement::new("key16".to_string(),vec![1,2,3],123456789);
    wal.add_record(record16.clone());
    let record17 = EntryElement::new("key17".to_string(),vec![1,2,3],123456789);
    wal.add_record(record17.clone());
    let record18 = EntryElement::new("key18".to_string(),vec![1,2,3],123456789);
    wal.add_record(record18.clone());
    let record19 = EntryElement::new("key19".to_string(),vec![1,2,3],123456789);
    wal.add_record(record19.clone());
    wal
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

