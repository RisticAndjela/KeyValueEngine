pub mod memtable_element;

#[cfg(test)]
mod tests {
    use crate::memtable_element::ElementMemtable;

    #[test]
    fn serialization() {
        let element = ElementMemtable {
            key: "example_key".to_string(),
            value: b"example_value".to_vec(),
            tombstone: false,
            timestamp: 1622547809,
        };

        let serialized = element.serialize();
        let deserialized = ElementMemtable::deserialize(&serialized);
        assert_eq!(deserialized,element);
    }
}
