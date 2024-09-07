//Bloom filter is a structure that uses hash functions to store values
// It can check whether a value is possibly in the set or definitely not in the set
use std::f64;
use hash_with_seed::{Hash, create_hash_funcs};

pub struct BloomFilter{
    pub bit_array: Vec<bool>, //set wit stored values
    pub hash_funcs: Vec<Hash>, //hash functions->seeds
    pub expected_elements:i64,
    pub false_positive_rate: f64,
}
impl BloomFilter{
    //constructor
    pub fn new(expected_elements:i64, false_positive_rate:f64)->Self{
        let m = (-(expected_elements as f64 * false_positive_rate.abs().ln()) /(2_f64.ln() * 2_f64.ln())) as usize;
        let k = ((m as f64 / expected_elements as f64) *  2_f64.ln()) as u32;
        BloomFilter{bit_array:vec![false; m], hash_funcs:create_hash_funcs(k),expected_elements,false_positive_rate}
    }
    //adding element by hashes in the structure
    pub fn add_element(&mut self, element: &[u8]) {
        for hash_func in &self.hash_funcs {
            let index = hash_func.hash_function(element) as usize % self.bit_array.len();
            self.bit_array[index] = true;
        }
    }

    //checking if element is definitely not in the set
    pub fn check(&mut self, element: &[u8]) -> bool {
        for hash_func in &self.hash_funcs {
            let index = (hash_func.hash_function(element) % self.bit_array.len() as u64) as usize;
            if !self.bit_array[index] {
                return false;
            }
        }
        true
    }
}

