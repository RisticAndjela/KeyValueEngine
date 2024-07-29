use hash_with_seed::Hash;
use crate::count_min_sketch::CountMinSketch;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor, Read};

pub fn serialize_cms(cms: &CountMinSketch) -> Vec<u8> {
    let num_rows = cms.matrix.len() as u32;
    let mut buf = Vec::new();
    buf.write_u32::<BigEndian>(num_rows).unwrap();

    for row in &cms.matrix {
        let num_columns = row.len() as u32;
        buf.write_u32::<BigEndian>(num_columns).unwrap();
        for &value in row {
            buf.write_u64::<BigEndian>(value).unwrap();
        }
    }

    // Serialize hash functions
    let num_hash_funcs = cms.hash_funcs.len() as u32;
    buf.write_u32::<BigEndian>(num_hash_funcs).unwrap();

    for hash_func in &cms.hash_funcs {
        let seed_len = hash_func.seed.len() as u32;
        buf.write_u32::<BigEndian>(seed_len).unwrap();
        buf.extend_from_slice(&hash_func.seed);
    }

    buf
}

pub fn deserialize_cms(data: &[u8]) -> CountMinSketch {
    let mut cursor = Cursor::new(data);

    // Deserialize the matrix
    let num_rows = cursor.read_u32::<BigEndian>().unwrap() as usize;
    let mut matrix = Vec::with_capacity(num_rows);

    for _ in 0..num_rows {
        let num_columns = cursor.read_u32::<BigEndian>().unwrap() as usize;
        let mut row = Vec::with_capacity(num_columns);
        for _ in 0..num_columns {
            row.push(cursor.read_u64::<BigEndian>().unwrap());
        }
        matrix.push(row);
    }

    // Deserialize hash functions
    let num_hash_funcs = cursor.read_u32::<BigEndian>().unwrap() as usize;
    let mut hash_funcs = Vec::with_capacity(num_hash_funcs);

    for _ in 0..num_hash_funcs {
        let seed_len = cursor.read_u32::<BigEndian>().unwrap() as usize;
        let mut seed = vec![0u8; seed_len];
        cursor.read_exact(&mut seed).unwrap();
        hash_funcs.push(Hash { seed });
    }

    CountMinSketch {
        matrix,
        hash_funcs,
    }
}
