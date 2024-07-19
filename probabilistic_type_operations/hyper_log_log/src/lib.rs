use crate::hyper_log_log::HyperLogLog;

mod hyper_log_log;
mod serialization;

#[cfg(test)]
mod tests {
    use crate::hyper_log_log::HyperLogLog;
    use crate::{equals, serialization};
    #[test]
    fn initialize_add_and_count() {
        let elements=100000;
        let tolerance=0.1; //less elements bigger tolerance
        let mut hll=HyperLogLog::new(10);
        for i in 0..elements{
            hll.add_element(format!("example{}", i).as_bytes());
        }
        println!("{}",((hll.get_count()-elements as f64).abs()/ elements as f64));
        assert!(((hll.get_count()-elements as f64).abs() / elements as f64) < tolerance);
    }
    #[test]
    fn serialization(){
        let mut hll=HyperLogLog::new(10);
        let serialize=serialization::serialize_hyper_log_log(&hll);
        assert!(equals(&hll,&serialization::deserialize_hyper_log_log(&serialize)));
    }
}
pub fn equals(hll1:&HyperLogLog,hll2:&HyperLogLog)->bool{
    if hll1.num_of_buckets!=hll2.num_of_buckets{return false;}
    for (i,&one) in hll1.buckets.iter().enumerate(){
        if hll2.buckets[i]!=one{return false;}
    }
    hash_with_seed::equals(&hll1.hash_func,&hll2.hash_func)
}