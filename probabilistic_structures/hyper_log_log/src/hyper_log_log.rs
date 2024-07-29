use std::cmp::{max, min};
use hash_with_seed::{Hash};
pub struct HyperLogLog{
    pub num_of_buckets:u64,
    pub buckets:Vec<u8>,
    pub hash_func:Hash,
}
impl HyperLogLog{
    pub fn new(num_of_buckets:u64)->Self{
        let buckets_len = 2usize.pow(num_of_buckets.clamp(4, 16) as u32) as u64;
        return HyperLogLog{num_of_buckets,buckets:vec![0u8; buckets_len as usize],hash_func:Hash::create_hash()}
    }
    pub fn add_element(&mut self, element: &[u8]) {
        let hash: u64 = self.hash_func.hash_function(element);
        let shift=64-self.num_of_buckets;
        let index = (hash >> shift )as usize;
        let trailing_zeros = min(shift, hash.trailing_zeros() as u64) + 1;
        self.buckets[index] = max(self.buckets[index], trailing_zeros as u8);
    }

    pub fn get_count(&self) -> f64 {
        let num_buckets = self.buckets.len() as f64;
        let mut sum = 0.0;
        for &bucket_value in &self.buckets {
            let value = 2u64.pow(bucket_value as u32) as f64;
            sum += 1.0 / value;
        }
        let estimate = 0.7213 / (1.0 + 1.079 / 16.0) * num_buckets * (num_buckets / sum);
        estimate.round()
    }

}
