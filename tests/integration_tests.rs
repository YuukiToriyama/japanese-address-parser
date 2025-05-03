use crate::common::run_data_driven_tests;

mod common;

#[tokio::test]
#[ignore]
async fn 県庁所在地のデータテスト() {
    run_data_driven_tests("./test_data/県庁所在地の住所データ.csv").await
}

#[tokio::test]
async fn 市区町村名レベルでの表記ゆれ() {
    run_data_driven_tests("./test_data/市区町村名レベルでの表記ゆれ.csv").await
}

#[tokio::test]
async fn 異字体旧字体への対応テスト() {
    run_data_driven_tests("./test_data/異字体旧字体への対応.csv").await
}

#[tokio::test]
async fn 異字体ではない表記ゆれへの対応テスト() {
    run_data_driven_tests("./test_data/異字体ではない表記ゆれへの対応.csv").await
}

#[tokio::test]
async fn 丁目が算用数字の場合への対応テスト() {
    run_data_driven_tests("./test_data/丁目が算用数字の場合への対応.csv").await
}

#[tokio::test]
async fn 住居表示実施済みの住所において正式でない表記への対応テスト() {
    run_data_driven_tests("./test_data/住居表示実施済みの住所において正式でない表記への対応.csv")
        .await
}

#[tokio::test]
async fn 字大字表記省略への対応テスト() {
    run_data_driven_tests("./test_data/字・大字表記省略への対応.csv").await
}

#[tokio::test]
async fn 郡が省略されている場合への対応テスト() {
    run_data_driven_tests("./test_data/郡が省略されている場合への対応.csv").await
}

#[tokio::test]
async fn 郡名と町名が一致している場合() {
    run_data_driven_tests("./test_data/郡名と町名が一致している場合.csv").await
}

#[tokio::test]
async fn 異字体セレクタを含む場合への対応() {
    run_data_driven_tests("./test_data/異字体セレクタを含む場合への対応.csv").await
}

#[tokio::test]
async fn 類似する町名候補が複数ある場合への対応() {
    run_data_driven_tests("./test_data/類似する町名候補が複数ある場合への対応.csv").await
}
