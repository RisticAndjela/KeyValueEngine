use std::io::{Cursor, Read};
use crate::hyper_log_log::HyperLogLog;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use hash_with_seed::Hash;

pub fn serialize_hyper_log_log(hll: &HyperLogLog) -> Vec<u8> {
    let mut buf = Vec::new();

    // Serialize num_of_buckets
    buf.write_u64::<BigEndian>(hll.num_of_buckets).unwrap();

    // Serialize buckets
    let buckets_len = hll.buckets.len() as u32;
    buf.write_u32::<BigEndian>(buckets_len).unwrap();
    buf.extend_from_slice(&hll.buckets);

    // Serialize hash function
    let seed_len = hll.hash_func.seed.len() as u32;
    buf.write_u32::<BigEndian>(seed_len).unwrap();
    buf.extend_from_slice(&hll.hash_func.seed);

    buf
}

pub fn deserialize_hyper_log_log(data: &[u8]) -> HyperLogLog {
    let mut cursor = Cursor::new(data);

    // Deserialize num_of_buckets
    let num_of_buckets = cursor.read_u64::<BigEndian>().unwrap();

    // Deserialize buckets
    let buckets_len = cursor.read_u32::<BigEndian>().unwrap() as usize;
    let mut buckets = vec![0u8; buckets_len];
    cursor.read_exact(&mut buckets).unwrap();

    // Deserialize hash function
    let seed_len = cursor.read_u32::<BigEndian>().unwrap() as usize;
    let mut seed = vec![0u8; seed_len];
    cursor.read_exact(&mut seed).unwrap();

    HyperLogLog {
        num_of_buckets,
        buckets,
        hash_func: Hash { seed },
    }
}
