# Changelog

マイナーバージョンアップやメジャーバージョンアップの際に発生する破壊的変更についてこのファイルに記載します。  
それ以外の変更については[リリースノート](https://github.com/YuukiToriyama/japanese-address-parser/releases)を参照ください。

## v0.2.0

### 非推奨に指定していたモジュール、関数を削除しました([#532](https://github.com/YuukiToriyama/japanese-address-parser/pull/532))。

- `japanese_address_parser::parser::parse`や`japanese_address_parser::parser::parse_blocking`を使用しているコードは動かなくなります。
  `japanese_address_parser::parser::Parser`の使用を検討してください。
- また、`japanese_address_parser::entity::Address`は`japanese_address_parser::domain::geolonia::Address`に移動しました。
  そのままだと動かないので、use文を修正してください。
