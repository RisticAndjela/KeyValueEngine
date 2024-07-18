use crate::bloom_filter::BloomFilter;
pub mod bloom_filter;
mod serialization;
#[cfg(test)]
mod tests {
    use crate::bloom_filter::{BloomFilter};
    use crate::equals;
    use crate::serialization::{deserialize_bloom, serialize_bloom};
    #[test]
    fn test_bloom_filter_add_and_check() {
        let mut bloom_filter = BloomFilter::new(100, 0.01);
        let element = &b"test_element"[..];

        let supposed_false=bloom_filter.check(&element);
        assert!(!supposed_false);

        bloom_filter.add_element(&element);

        let supposed_true=bloom_filter.check(&element);
        assert!(supposed_true);
    }
    #[test]
    fn test_bloom_filter_serialization_and_deserialization() {
        let mut bloom_filter = BloomFilter::new(100, 0.01);
        let element = &b"test_element"[..];
        bloom_filter.add_element(&element);

        let serialized=serialize_bloom(&bloom_filter);
        let deserialized=deserialize_bloom(&serialized);

        assert!(equals(&bloom_filter,&deserialized))
    }

}

fn equals(b1:&BloomFilter,b2:&BloomFilter)->bool{
    for hash in b2.hash_funcs.iter(){
        if b2.hash_funcs.iter().any(|a|a.seed==hash.seed){
            continue;
        }else{
            return false;
        }
    }
    b1.bit_array == b2.bit_array
}