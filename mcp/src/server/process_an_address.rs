use japanese_address_parser::experimental::parser::Parser;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct RequestParameters {
    /// 処理したい住所
    ///
    /// 例: 東京都千代田区丸ノ内1-1-1
    pub address: String,
}

pub(crate) async fn process_an_address(
    params: RequestParameters,
) -> Result<CallToolResult, ErrorData> {
    let parser = Parser::default();
    let result = parser.parse(&params.address).await;
    let json = serde_json::to_string_pretty(&result)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}
