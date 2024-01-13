use crate::common::run_data_driven_tests;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

mod common;

#[tokio::test]
#[ignore]
async fn 県庁所在地のデータテスト() {
    run_data_driven_tests("./tests/test_data/県庁所在地の住所データ.csv").await
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

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn hoge() {
    run_data_driven_tests("https://raw.githubusercontent.com/YuukiToriyama/japanese-address-parser/main/tests/test_data/%E7%95%B0%E5%AD%97%E4%BD%93%E6%97%A7%E5%AD%97%E4%BD%93%E3%81%B8%E3%81%AE%E5%AF%BE%E5%BF%9C.csv?token=GHSAT0AAAAAACLCZZXTHCMCW2BYK5AEW7USZNCMJUA").await
}
