use std::time::{SystemTime, UNIX_EPOCH};

pub struct TokenBucket {
    pub capacity: u64,          // max tokens
    pub tokens: u64,            // current no. tokens
    pub last_refill_time: i64,
    pub refill_rate: u64,       // number of tokens added per second
}

impl TokenBucket {
    pub fn new(capacity: u64, refill_rate: u64) -> Self {
        TokenBucket {
            capacity,
            tokens: capacity,
            last_refill_time: get_now_in_seconds(),
            refill_rate,
        }
    }

    pub fn request(&mut self) -> bool {
        self.refill();
        if self.tokens > 0 {
            self.tokens -= 1;
            true
        } else {
            false
        }
    }

    pub fn refill(&mut self) {
        let now = get_now_in_seconds();
        let elapsed_time = now - self.last_refill_time;

        if elapsed_time > 0 {

            let new_tokens = (elapsed_time as u64) * self.refill_rate; // how many tokens to add
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_refill_time = now;
        }
    }
}

pub fn get_now_in_seconds() -> i64 {
    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}