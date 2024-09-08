mod token_bucket;
mod serialization;

#[cfg(test)]
mod tests {
    use crate::token_bucket::TokenBucket;

    #[test]
    fn test() {
        let mut bucket = TokenBucket::new(3, 15);

        assert!(bucket.request());
        assert!(bucket.request());
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(bucket.request());
        assert!(bucket.request());
        assert!(bucket.request());
        assert!(!bucket.request());
        assert!(!bucket.request());
        std::thread::sleep(std::time::Duration::from_secs(2));
        assert!(bucket.request());
        assert!(bucket.request());


        let serialized = bucket.serialize();
        let mut deserialized_bucket = TokenBucket::deserialize(&serialized);
        println!("Deserialized tokens: {}", deserialized_bucket.tokens);
    }
}
