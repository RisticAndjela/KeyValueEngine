extern crate core;
mod hash_with_seed;

pub use hash_with_seed::{Hash, create_hash_funcs};
#[cfg(test)]
mod tests{
    use crate::{equals};

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
    #[test]
    fn neutral(){
        let hash_funcs = crate::create_hash_funcs(3);
        let data = vec![0u8,32];
        for hash_func in hash_funcs {
            let result = hash_func.hash_function(data.as_slice());
            println!("Hash result: {}", result);
            assert_ne!(result, 0);
        }
    }
    #[test]
    fn test_serialize_deserialize(){
        let data:&[u8;32] = b"test data                       ";
        let hash1 = crate::Hash{ seed: Vec::from(*data) };
        let serialized=hash1.serialize_seed();
        let hash2=crate::Hash::deserialize_seed(serialized);
        assert!(equals(&hash1,&hash2));
    }
}

pub fn equals(hash1:&Hash,hash2:&Hash)->bool{
    for (i,&seed) in hash1.seed.iter().enumerate(){
        if hash2.seed[i]!=seed {
            return false;
        }
    }
    true
}