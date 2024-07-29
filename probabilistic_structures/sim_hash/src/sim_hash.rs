use std::collections::HashMap;
use hash_with_seed::Hash;

pub fn is_in_repetitive_words(word:&str) ->bool{
    let repetitive_word=["and","to","i","you","he","she","it","they","we","as","is","am","are",
        "on","in","for","but","or","nor","for","so","yet"];
    return repetitive_word.contains(&word)
}
pub fn hash_and_count_words(text:&str)->HashMap<u64,i32>{
    let hash = Hash::create_hash();
    let mut result: HashMap<u64,i32> = HashMap::new();
    for word_original in text.split_whitespace(){
        let word_cleaned: String = word_original.chars().filter(|c| !",.!:;?".contains(*c)).collect();
        let word_lower = word_cleaned.to_lowercase();
        let word = word_lower.as_str();
        if !is_in_repetitive_words(word){
            let hashed=hash.hash_function(word.as_bytes());
            result.entry(hashed).and_modify(|e| *e += 1).or_insert(1);
        }
    }
    return result;
}

pub fn sim_hash(text: &str) -> u64 {
    let mapped = hash_and_count_words(text);
    let mut hash_sum: Vec<i64> = vec![0;64] ;
    for (word, count) in mapped.iter() {
        for i in 0..64{
            let byte = (word >> i) & 1 as u64;
            if byte == 1 as u64 {
                hash_sum[i] += *count as i64;
            } else {
                hash_sum[i] -= *count as i64;
            }
        }
    }
    let mut bit_sum:Vec<u8>=vec![0;64];
    for (i,value) in hash_sum.iter().enumerate(){
        if *value>=0 { bit_sum[i]=1}
        else { bit_sum[i]=0}
    }
    return vector_to_u64(&bit_sum)
}
fn vector_to_u64(bits: &Vec<u8>) -> u64 {
    let mut result: u64 = 0;
    for (i, &bit) in bits.iter().enumerate() {
        if bit == 1 {
            result |= 1 << i;
        }
    }
    result
}
pub fn hamming_distance(hash1: u64, hash2: u64) -> u32 {
    (hash1 ^ hash2).count_ones()
}
