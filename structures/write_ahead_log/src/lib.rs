mod write_ahead_log;

#[cfg(test)]
mod tests {
    use memtable_element::memtable_element::ElementMemtable;
    use crate::write_ahead_log::WriteAheadLog;

    #[test]
    fn test_append_and_iter() {
        if std::fs::remove_file("test_log.txt").is_ok() {
            println!("Removed existing file.");
        } else {
            println!("No existing file to remove.");
        }

        let mut log = WriteAheadLog {
            filename: "test_log.txt".to_string(),
            segment_length: 1024,
            current_offset: 0,
            // current_index: 0,
        };

        let elements = vec![
            ElementMemtable {
                key: "test_key1".to_string(),
                value: vec![1, 2, 3, 4],
                tombstone: false,
                timestamp: 1624047392,
            },
            ElementMemtable {
                key: "test_key2".to_string(),
                value: vec![5, 6, 7, 8],
                tombstone: true,
                timestamp: 1624047393,
            },
            ElementMemtable {
                key: "test_key3".to_string(),
                value: vec![9, 10, 11, 12],
                tombstone: false,
                timestamp: 1624047394,
            },
        ];

        println!("Appending elements:");
        for elem in &elements {
            println!("Appending: {:?}", elem);
            log.append(elem).unwrap();
        }

        println!("Reading elements:");
        let mut read_elements = log.iter().collect::<Vec<_>>();

        read_elements.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        for elem in &read_elements {
            println!("Read: {:?}", elem);
        }

        assert_eq!(elements, read_elements);
    }

}
