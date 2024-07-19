use md5::{Digest, Md5};
use byteorder::{ByteOrder, BigEndian};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Hash {
    pub seed: Vec<u8>,
}
impl Hash {
    pub fn hash_function(&self, data: &[u8]) -> u64 {
        let mut hasher = Md5::new();
        hasher.update(data);
        hasher.update(&self.seed);
        let result = hasher.finalize();
        BigEndian::read_u64(&result)
    }
    pub fn serialize_seed(&self) -> &[u8] {
        &self.seed
    }
    pub fn deserialize_seed(data: &[u8]) -> Hash {
        let mut seed =  vec![0u8;64];
        seed.copy_from_slice(data);
        Hash { seed }
    }

}
pub fn create_hash_funcs(k: u32) -> Vec<Hash> {
    let mut hashes = Vec::new();
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as u32;

    for i in 0..k {
        let mut seed = vec![0u8;64];
        BigEndian::write_u32(&mut seed[0..8], (start_time + i) as u32);
        hashes.push(Hash { seed });
    }

    hashes
}