pub mod entry_element;
pub mod constants;

#[cfg(test)]
mod tests {
    use crate::entry_element::EntryElement;

    #[test]
    fn serialization() {
        let element = EntryElement {
            key: "example_key".to_string(),
            value: b"example_value".to_vec(),
            tombstone: false,
            timestamp: 1622547809,
        };

        let serialized = element.serialize();
        let deserialized = EntryElement::deserialize(&serialized);
        assert_eq!(deserialized,element);
    }
}
