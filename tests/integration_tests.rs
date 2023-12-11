#![feature(async_closure)]

use crate::common::read_test_data_from_csv;
use japanese_address_parser::Parser;
use std::panic;

mod common;

#[tokio::test]
#[ignore]
async fn parser_tests() {
    let records = read_test_data_from_csv("./tests/addresses.csv").unwrap();
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
