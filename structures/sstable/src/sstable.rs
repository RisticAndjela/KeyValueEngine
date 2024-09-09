use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use bloom_filter::bloom_filter::BloomFilter;
use bloom_filter::serialization::{deserialize_bloom, serialize_bloom};
use merkle_tree::merkle_tree::{MerkleTree};
use merkle_tree::serialization::{deserialize_tree, serialize_tree};
use crate::data::Data;
use crate::index::Index;
use crate::summary::Summary;
use entry_element::entry_element::{extract, EntryElement as record, EntryElement};
use crate::global_sstable_functions::get_name;

#[derive(Clone,Debug)]
pub struct SSTable{
    pub dir_path:String,
    pub data:Data,
    pub index: Index,
    pub summary: Summary,
    pub bloom_filter: BloomFilter,
    pub merkle_tree: MerkleTree
}
impl SSTable{
    pub fn open(dir_path:String,volume_index:i32,volume_summary:i32)->Self{
        let data = Data{file_path:dir_path.clone()};
        let index = Index{file_path:dir_path.clone(),volume:volume_index};
        let summary = Summary{file_path:dir_path.clone(),volume:volume_summary};

        let mut filter_file = File::open(format!("{}/filter.bin", dir_path)).expect("Failed to open filter file");
        let mut filter_data = Vec::new();
        filter_file.read_to_end(&mut filter_data).expect("Failed to read filter file");
        let bloom_filter = deserialize_bloom(&filter_data);

        let mut merkle_file = File::open(format!("{}/merkle.bin", dir_path)).expect("Failed to open merkle file");
        let mut merkle_data = Vec::new();
        merkle_file.read_to_end(&mut merkle_data).expect("Failed to read merkle file");
        let merkle_tree = deserialize_tree(&merkle_data);

        SSTable{dir_path,data,index,summary,bloom_filter,merkle_tree}

    }
    pub fn new(storage_path:String,volume_of_index:i32,volume_of_summary:i32,expected_elements:i64,false_positive_rate:f64)->Self{
        let dir_path=get_name(storage_path);
        fs::create_dir_all(&dir_path).expect("Failed to create directory");

        let data=Data::new(dir_path.clone());
        let index=Index::new(dir_path.clone(),volume_of_index);
        let summary=Summary::new(dir_path.clone(),volume_of_summary);
        let bloom_filter=BloomFilter::new(expected_elements,false_positive_rate);
        let merkle_tree=MerkleTree::new();
        SSTable{dir_path,data, index, summary,bloom_filter,merkle_tree}
    }
    pub fn activate_flush(&mut self, records:Vec<record>){
        let data_file = OpenOptions::new().append(true).open(&self.data.file_path).expect("Failed to open data file for appending");
        let index_file = OpenOptions::new().append(true).open(&self.index.file_path).expect("Failed to open index file for appending");
        let summary_file = OpenOptions::new().append(true).open(&self.summary.file_path).expect("Failed to open summary file for appending");

        let mut data_writer = BufWriter::new(data_file);
        let mut index_writer = BufWriter::new(index_file);
        let mut summary_writer = BufWriter::new(summary_file);

        let mut position_counter_in_data:u64=0;
        let mut index_counter:u64=0;
        let mut position_counter_in_index:u64=0;

        for i in 0..records.len() {
            let record=records[i].clone();
            let serialized_record: Vec<u8> = record.serialize();
            let data_size = serialized_record.len() as u64;

            if (i as i32 % self.index.volume==0 ) || i==0 || i==records.len()-1{
                let key_size = record.key.as_bytes().len() as u64; // "abcdefg"=7
                let key_bytes = record.key.as_bytes(); // ["a","b","c"..."g"]
                let index_size = 8 + key_size + 8 ; // u 64 -> 8B

                index_writer.write_all(&index_size.to_be_bytes()).expect("Failed to write record size"); // how much will I read for all
                index_writer.write_all(&key_size.to_be_bytes()).expect("Failed to write record size"); // how much will I read for key
                index_writer.write_all(&key_bytes).expect("Failed to write record size"); // key
                index_writer.write_all(&position_counter_in_data.to_be_bytes()).expect("Failed to write record size"); // what position is in data file
                if (index_counter as i32 % (self.summary.volume))==0 || i==0 || i==records.len()-1{
                    summary_writer.write_all(&index_size.to_be_bytes()).expect("Failed to write record size"); // how much will I read for all
                    summary_writer.write_all(&key_size.to_be_bytes()).expect("Failed to write record size");  // how much will I read for key
                    summary_writer.write_all(&key_bytes).expect("Failed to write record size"); // key
                    summary_writer.write_all(&position_counter_in_index.to_be_bytes()).expect("Failed to write record size"); // what position is in index file
                }
                position_counter_in_index+=index_size;
                index_counter+=1;

            }
            position_counter_in_data+= 8+ data_size.clone();
            data_writer.write_all(&data_size.to_be_bytes()).expect("Failed to write record size");
            data_writer.write_all(&serialized_record).expect("Failed to write record to file");

            self.bloom_filter.add_element(record.key.as_bytes());
            self.merkle_tree.add(record.key.into_bytes());
        }

        data_writer.flush().expect("Failed to flush writer");// to ensure all data is written to the file
        index_writer.flush().expect("Failed to flush writer");// to ensure all indexes is written to the file
        summary_writer.flush().expect("Failed to flush writer");// to ensure all summary is written to the file


        let filter_file = File::create(format!("{}/filter.bin",self.dir_path.clone())).expect("Failed to create file");
        let mut filter_writer = BufWriter::new(filter_file);
        filter_writer.write_all(&serialize_bloom(&self.bloom_filter)).expect("Failed to write to Merkle file");
        filter_writer.flush().expect("Failed to flush Merkle file");let merkle_file = File::create(format!("{}/merkle.bin",self.dir_path.clone())).expect("Failed to create file");
        let mut merkle_writer = BufWriter::new(merkle_file);
        merkle_writer.write_all(&serialize_tree(&self.merkle_tree)).expect("Failed to write to Merkle file");
        merkle_writer.flush().expect("Failed to flush Merkle file");

    }
    pub fn search(&mut self, key:String) -> EntryElement {
        let possibly_exist=self.bloom_filter.check(key.as_bytes()); // true - it might exist
        if !possibly_exist{return record::empty()}
        // self.data.read_data_file(0);
        // self.index.read_index_file();
        // self.summary.read_summary_file();
        let (start_of_segment_in_index,end_of_segment_in_index)=self.summary.search_for_segment_in_index(key.clone());
        let (start_of_segment_in_data,_)=self.index.search_for_segment_in_data(start_of_segment_in_index,end_of_segment_in_index,key.clone());
        let result=self.data.search_from_segment(start_of_segment_in_data,key.clone());
        if result.is_irrelevant(){return record::empty()}
        result
    }

    // delete is making new sstable
    pub fn delete(&self, key_to_delete: String) -> SSTable {
        let mut path_buf = PathBuf::from(self.dir_path.clone());
        path_buf.pop();
        let mut new_sstable = SSTable::new(path_buf.to_string_lossy().into_owned(), self.index.volume, self.summary.volume, self.bloom_filter.expected_elements, self.bloom_filter.false_positive_rate);
        let data_file = OpenOptions::new().read(true).open(&self.data.file_path).expect("Failed to open data file for reading");
        let mut data_reader = BufReader::new(data_file);
        let new_data_file = OpenOptions::new().append(true).open(&new_sstable.data.file_path).expect("Failed to open new data file for writing");
        let mut new_data_writer = BufWriter::new(new_data_file);

        loop{
            let mut size_buffer = [0u8; 8];
            if data_reader.read_exact(&mut size_buffer).is_err() {
                break;
            }
            let record_size = u64::from_be_bytes(size_buffer) as usize;
            let mut record_buffer = vec![0u8; record_size];
            data_reader.read_exact(&mut record_buffer).expect("wrong");
            let mut record = record::deserialize(&record_buffer);
            if record.extract_number_from_key()==extract(key_to_delete.clone().as_str()){
                record.tombstone=true;
            }
            let serialized_record = record.serialize();
            new_data_writer.write_all(&serialized_record.len().to_be_bytes()).expect("Failed to write record size");
            new_data_writer.write_all(&serialized_record).expect("Failed to write to new data writer");
        }
        new_data_writer.flush().expect("Failed to flush new data writer");

        fs::copy(&self.index.file_path, &new_sstable.index.file_path).expect("Failed to copy index");
        fs::copy(&self.summary.file_path, &new_sstable.summary.file_path).expect("Failed to copy summary");
        fs::copy(format!("{}/merkle.bin",&self.dir_path),format!("{}/merkle.bin",&new_sstable.dir_path)).expect("Failed to copy merkle");
        fs::copy(format!("{}/filter.bin",&self.dir_path),format!("{}/filter.bin",&new_sstable.dir_path)).expect("Failed to copy bloom");
        new_sstable.index.file_path=format!("{}/index.bin",path_buf.to_string_lossy().into_owned());
        new_sstable.summary.file_path=format!("{}/summary.bin",path_buf.to_string_lossy().into_owned());

        new_sstable
    }
    pub fn compare_similarity(&self, other:SSTable)->bool{
        let distance=self.merkle_tree.compare(other.merkle_tree);
        if distance <= 0.4 { true } else { false }
    }
}

