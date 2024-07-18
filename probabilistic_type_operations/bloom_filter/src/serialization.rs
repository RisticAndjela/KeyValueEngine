use byteorder::{BigEndian, ByteOrder};
use crate::bloom_filter::BloomFilter;
use hash_with_seed::{Hash};

pub fn serialize_bloom(bloom: &BloomFilter) -> Vec<u8> {
    let len_bit_array = bloom.bit_array.len() as u32;
    let len_hash_funcs = bloom.hash_funcs.len() as u32;

    let buf_size = 4 + len_bit_array as usize + 4 + (len_hash_funcs as usize * 64);
    let mut buf = Vec::with_capacity(buf_size);

    buf.extend_from_slice(&len_bit_array.to_be_bytes());

    for &bit in &bloom.bit_array {
        buf.push(bit as u8);
    }

    buf.extend_from_slice(&len_hash_funcs.to_be_bytes());

    for hash_func in &bloom.hash_funcs {
        buf.extend_from_slice(hash_func.serialize_seed());
    }

    buf
}

pub fn deserialize_bloom(data: &[u8]) -> BloomFilter {
    let len_bit_array = u32::from_be_bytes(data[0..4].try_into().unwrap()) as usize;
    let bit_array_start = 4;
    let bit_array_end = bit_array_start + len_bit_array;
    let bit_array: Vec<bool> = data[bit_array_start..bit_array_end]
        .iter()
        .map(|&b| b != 0)
        .collect();

    let len_hash_funcs = u32::from_be_bytes(data[bit_array_end..bit_array_end + 4].try_into().unwrap()) as usize;
    let hash_funcs_start = bit_array_end + 4;
    let hash_funcs: Vec<Hash> = (0..len_hash_funcs)
        .map(|i| {
            let start = hash_funcs_start + i * 64;
            let end = start + 64;
            Hash::deserialize_seed(&data[start..end])
        })
        .collect();

    BloomFilter {
        bit_array,
        hash_funcs,
    }
}