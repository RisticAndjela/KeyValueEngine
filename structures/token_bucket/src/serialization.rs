use crate::token_bucket::TokenBucket;

impl TokenBucket{
    pub fn serialize(&self) -> Vec<u8> {
        let mut serialized_data = Vec::with_capacity(8 + 8 + 8 + 8);

        serialized_data.extend(self.capacity.to_be_bytes());
        serialized_data.extend(self.tokens.to_be_bytes());
        serialized_data.extend(self.last_refill_time.to_be_bytes());
        serialized_data.extend(self.refill_rate.to_be_bytes());

        serialized_data
    }

    pub fn deserialize(bytes: &Vec<u8>) -> Self {
        let capacity = u64::from_be_bytes(bytes[0..8].try_into().unwrap());
        let tokens = u64::from_be_bytes(bytes[8..16].try_into().unwrap());
        let last_refill_time = i64::from_be_bytes(bytes[16..24].try_into().unwrap());
        let refill_rate = u64::from_be_bytes(bytes[24..32].try_into().unwrap());

        let mut tb=TokenBucket { capacity, tokens, last_refill_time, refill_rate, };
        tb.refill();
        tb
    }
}