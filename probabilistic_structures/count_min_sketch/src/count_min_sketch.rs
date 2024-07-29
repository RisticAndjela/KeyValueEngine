use std::cmp::max;
use hash_with_seed::{Hash, create_hash_funcs};

pub struct CountMinSketch{
    pub matrix :Vec<Vec<u64>>,
    pub hash_funcs: Vec<Hash>,
}
impl CountMinSketch{
    pub fn new(rows_hashes:u32, columns:u32) -> Self {
        let hash_funcs=create_hash_funcs(rows_hashes);
        let matrix = vec![vec![0u64; columns as usize]; rows_hashes as usize];
        return CountMinSketch{matrix,hash_funcs};
    }
    pub fn add_element(&mut self, element: &[u8]){
        for (i,row) in self.matrix.iter_mut().enumerate(){
            let index=self.hash_funcs[i].hash_function(element) as usize % row.len();
            if index <=row.len() { row[index] += 1; }
        }
    }
    pub fn get_count(&self, element:&[u8])->u64{
        let mut min = u64::MAX;
        for (i,row) in self.matrix.iter().enumerate(){
            let index=self.hash_funcs[i].hash_function(element)as usize % row.len();
            if index <=row.len() {
                if min>row[index] {min=row[index];}
            }
        }
        if min == u64::MAX {0}else {min}
    }
}

pub fn get_rows_hashes(tol_on_errors: f64)->u32{
    let tolerance=((2.71 / tol_on_errors).round() as usize).checked_next_power_of_two();
    return max(1,tolerance.unwrap()) as u32;
}
pub fn get_columns(prob_of_collisions:f64)->u32{
    let probability=((2.71 / prob_of_collisions).round() as usize).checked_next_power_of_two();
    return max(2,probability.unwrap()) as u32;
}
