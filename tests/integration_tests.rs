#![feature(async_closure)]

use crate::common::run_data_driven_tests;

mod common;

#[tokio::test]
async fn parser_tests() {
    run_data_driven_tests("./tests/addresses.csv").await
}
