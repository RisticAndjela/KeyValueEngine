extern crate core;

//tests for Bloom filter
mod hash_with_seed;
pub use hash_with_seed::{Hash, create_hash_funcs};
#[cfg(test)]
mod tests{
    use core::hash;

    #[test]
    fn test_hash_function() {
        let hash_funcs = crate::create_hash_funcs(3);
        let data = b"test data";
        for hash_func in hash_funcs {
            let result = hash_func.hash_function(data);
            println!("Hash result: {}", result);
            assert_ne!(result, 0);
        }
    }
    fn test_serialize_deserialize(){
        let data = b"test data";
        let hash1 = hash::Hash{data};

    }
}
