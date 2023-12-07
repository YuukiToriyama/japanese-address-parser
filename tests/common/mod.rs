use csv::Reader;
use serde::Deserialize;
use std::fs::File;

#[derive(Deserialize)]
pub struct Record {
    pub address: String,
    pub prefecture: String,
    pub city: String,
    pub town: String,
    pub rest: String,
}

pub fn read_test_data_from_csv(file_path: &str) -> Result<Vec<Record>, &str> {
    let file = File::open(file_path).unwrap();
    let mut reader = Reader::from_reader(file);
    let mut records: Vec<Record> = vec![];
    for result in reader.deserialize() {
        let record: Record = result.unwrap();
        records.push(record)
    }
    Ok(records)
}
