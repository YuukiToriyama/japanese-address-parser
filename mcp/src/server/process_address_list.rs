use japanese_address_parser::experimental::parser::Parser;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub(crate) struct RequestParameters {
    /// 処理したい住所のリスト(最大100件)
    pub address_list: Vec<String>,
}

pub(crate) async fn process_address_list(
    params: RequestParameters,
) -> Result<CallToolResult, ErrorData> {
    if params.address_list.is_empty() {
        return Err(ErrorData::invalid_params(
            "Address list cannot be empty",
            None,
        ));
    }
    if params.address_list.len() > 100 {
        return Err(ErrorData::invalid_params(
            "More than 100 addresses cannot be entered",
            None,
        ));
    }

    let mut results = Vec::with_capacity(params.address_list.len());
    let parser = Parser::default();
    for address in &params.address_list {
        let result = parser.parse(address).await;
        results.push(serde_json::json!({
            "input": address,
            "result": result,
        }));
    }

    let json = serde_json::to_string_pretty(&results)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}
