# japanese-address-parser-mcp

[![Crates.io (latest)](https://img.shields.io/crates/v/japanese-address-parser-mcp)](https://crates.io/crates/japanese-address-parser-mcp)

[japanese-address-parser](https://github.com/YuukiToriyama/japanese-address-parser)をModel Context Protocol(MCP)サーバーとして提供するためのクレートです。

このクレートを利用することで、Claude DesktopなどのMCP対応クライアントを介して、日本の住所の正規化機能をAIアシスタントから直接呼び出すことが可能になります。  
RustやJavaScriptのコードを実装することなく、自然言語による指示のみで高度な住所解析機能を利用できます。  

## 主な特徴

- **正確な住所分割**: 複雑な日本の住所体系を「都道府県」「市区町村」「町名」「番地以降」に分割します。
- **バッチ処理対応**: 最大100件までの住所を一度の操作で一括処理可能です。
- **シームレスなAI統合**: Claude Desktop やその他の MCP 対応ツールと即座に連携できます。

## 公開ツール

本サーバーは以下のツールをAIアシスタントに提供します。

| ツール名 | 機能概要                                    |
| :--- |:----------------------------------------|
| `process_an_address` | 単一の住所文字列を解析し、構成要素(都道府県・市区町村・町名等)に分割します。 |
| `process_address_list` | 複数の住所を一括で解析します(一回の呼び出しで最大100件まで)。       |

## 導入手順

### 1. MCPサーバーのインストール

#### crates.ioからインストールする

```bash
cargo install japanese-address-parser-mcp
```

インストールが完了すると、`$HOME/.cargo/bin/japanese-address-parser-mcp`に実行バイナリが配置されます。

#### ソースコードからビルドする

```bash
git clone git@github.com:YuukiToriyama/japanese-address-parser.git
cd japanese-address-parser
cargo build --release -p japanese-address-parser-mcp
```

ビルドが完了すると、`./target/release/japanese-address-parser-mcp` に実行バイナリが生成されます。

### 2. 設定(Claude Desktop)

ClaudeDesktopの設定ファイル(`~/Library/Application Support/Claude/claude_desktop_config.json`)に本サーバーの情報を追記してください。

```json
{
  "mcpServers": {
    "japanese-address-parser": {
      "command": "/path/to/japanese-address-parser-mcp"
    }
  }
}
```

`command` の値は、ビルドしたバイナリの**絶対パス**に置き換えてください。設定の反映には Claude Desktop の再起動が必要です。

## 活用例

AI アシスタントに対して、以下のようなプロンプトを入力することで機能を活用できます。

- **住所の正規化依頼**
  > 「東京都千代田区丸の内1-1-1 を都道府県、市区町村、町名に分解して、結果をテーブル形式で表示してください。」

- **複数住所の整理依頼**
  > 「（表記揺れを含む住所のリストを添付して）これらの住所を正規化し、CSV形式で出力してください。」

## アーキテクチャ

本ツールは、Model Context Protocol (MCP) に基づいたクライアント/サーバー構成で動作します。

```text
[MCP クライアント]
        │
        │ MCP (Standard I/O)
        ↓
[japanese-address-parser-mcp (MCP サーバー)]
        │
        │ 依存
        ↓
[japanese-address-parser (コアライブラリ)]
```

### 各コンポーネントの役割
- **MCP クライアント**: Claude Desktop などの MCP に対応したアプリケーションです。ユーザーの指示内容を解析し、住所パースが必要な場合に本サーバーのツールを呼び出します。
- **japanese-address-parser-mcp (MCP サーバー)**: MCP プロトコルを実装した実行バイナリです。標準入出力を介してAIアシスタントからのリクエストを受け取り、コアライブラリに処理を委譲します。
- **japanese-address-parser (コアライブラリ)**: 住所の正規化および分割処理を行うRust製のコアロジックです。詳細は[こちら](https://github.com/YuukiToriyama/japanese-address-parser)をご覧ください。


## ライセンス

本ソフトウェアは MIT ライセンスの下で配布されています。
