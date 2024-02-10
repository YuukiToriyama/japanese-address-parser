use crate::common::run_data_driven_tests;

mod common;

#[tokio::test]
#[ignore]
async fn 県庁所在地のデータテスト() {
    run_data_driven_tests("./tests/test_data/県庁所在地の住所データ.csv").await
}

#[tokio::test]
async fn 市区町村名レベルでの表記ゆれ() {
    run_data_driven_tests("./tests/test_data/市区町村名レベルでの表記ゆれ.csv").await
}

#[tokio::test]
async fn 異字体旧字体への対応テスト() {
    run_data_driven_tests("./tests/test_data/異字体旧字体への対応.csv").await
}

#[tokio::test]
async fn 丁目が算用数字の場合への対応テスト() {
    run_data_driven_tests("./tests/test_data/丁目が算用数字の場合への対応.csv").await
}

#[tokio::test]
async fn 住居表示実施済みの住所において正式でない表記への対応テスト() {
    run_data_driven_tests(
        "./tests/test_data/住居表示実施済みの住所において正式でない表記への対応.csv",
    )
    .await
}

#[tokio::test]
async fn 大字表記省略への対応テスト() {
    run_data_driven_tests("./tests/test_data/大字表記省略への対応.csv").await
}
