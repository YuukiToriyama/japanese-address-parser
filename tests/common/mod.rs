use csv::Reader;
use serde::Deserialize;
use std::fs::File;
use std::panic;
use japanese_address_parser::Parser;

#[derive(Deserialize)]
pub struct Record {
    pub address: String,
    pub prefecture: String,
    pub city: String,
    pub town: String,
    pub rest: String,
}

fn read_test_data_from_csv(file_path: &str) -> Result<Vec<Record>, &str> {
    let file = File::open(file_path).unwrap();
    let mut reader = Reader::from_reader(file);
    let mut records: Vec<Record> = vec![];
    for result in reader.deserialize() {
        let record: Record = result.unwrap();
        records.push(record)
    }
    Ok(records)
}

pub async fn run_data_driven_tests(file_path: &str) {
    let records = read_test_data_from_csv(file_path).unwrap();
    let mut success_count = 0;
    for record in &records {
        let parser = Parser();
        let result = parser.parse(&record.address).await;

        let test_result = panic::catch_unwind(|| {
            assert_eq!(result.address.prefecture, record.prefecture);
            assert_eq!(result.address.city, record.city);
            assert_eq!(result.address.town, record.town);
            assert_eq!(result.address.rest, record.rest);
        });
        match test_result {
            Ok(_) => {
                println!("Success: {}", record.address);
                success_count += 1;
            }
            Err(_) => println!("Failed: {}", record.address),
        };
    }
    assert_eq!(
        success_count,
        records.len(),
        "{} of {} cases failed.",
        records.len() - success_count,
        records.len()
    );
}
