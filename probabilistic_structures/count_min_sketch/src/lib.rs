use crate::count_min_sketch::CountMinSketch;

pub mod count_min_sketch;
pub mod serialization;

#[cfg(test)]
mod tests {
    use crate::count_min_sketch::{CountMinSketch,get_rows_hashes,get_columns};
    use crate::equals;
    use crate::serialization::{deserialize_cms, serialize_cms};

    #[test]
    fn add_and_check() {
        let cms=&mut CountMinSketch::new(get_rows_hashes(1.0),get_columns(0.01));
        let element1 = b"example1";
        let element2 = b"example2";
        assert!(cms.get_count(element1)==0);
        cms.add_element(element1);
        cms.add_element(element1);
        cms.add_element(element2);
        assert!(cms.get_count(element1)==2);
        assert!(cms.get_count(element2)==1);

    }
    #[test]
    fn serialization_and_deserialization(){
        let cms1=&mut CountMinSketch::new(get_rows_hashes(1.0),get_columns(0.01));
        let element1 = b"example1";
        cms1.add_element(element1);
        let serialized:Vec<u8>=serialize_cms(cms1);
        let cms2=deserialize_cms(&serialized);
        assert!(equals(&cms1,&cms2));
    }
}

pub fn equals(cms1: &CountMinSketch, cms2: &CountMinSketch) -> bool {
    if cms1.matrix.len() != cms2.matrix.len() {
        return false;
    }

    for (i, row1) in cms1.matrix.iter().enumerate() {
        let row2 = &cms2.matrix[i];
        if row1.len() != row2.len() {
            return false;
        }
        for (a, value1) in row1.iter().enumerate() {
            if row2[a] != *value1 {
                return false;
            }
        }
    }

    if cms1.hash_funcs.len() != cms2.hash_funcs.len() {
        return false;
    }

    for (i, hash1) in cms1.hash_funcs.iter().enumerate() {
        if !hash_with_seed::equals(hash1, &cms2.hash_funcs[i]) {
            return false;
        }
    }

    true
}
