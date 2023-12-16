#![feature(async_closure)]

use crate::common::run_data_driven_tests;

mod common;

#[tokio::test]
#[ignore]
async fn 県庁所在地のデータテスト() {
    run_data_driven_tests("./tests/test_data/県庁所在地の住所データ.csv").await
}
