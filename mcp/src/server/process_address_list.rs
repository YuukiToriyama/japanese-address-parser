use japanese_address_parser::experimental::parser::Parser;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;

use futures::StreamExt;
use std::sync::Arc;

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

    let max_concurrency: usize = std::env::var("JAPANESE_ADDRESS_PARSER_MCP_MAX_CONCURRENCY")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8);

    let parser = Arc::new(Parser::default());

    let stream = futures::stream::iter(params.address_list.into_iter().enumerate())
        .map(|(index, address)| {
            let parser = Arc::clone(&parser);
            async move { parse_entry(parser, index, address).await }
        })
        .buffer_unordered(max_concurrency);

    let mut entries = stream.collect::<Vec<(usize, Value)>>().await;
    entries.sort_by_key(|(index, _)| *index);
    let results: Vec<Value> = entries.into_iter().map(|(_, v)| v).collect();

    let json = serde_json::to_string_pretty(&results)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}

async fn parse_entry(parser: Arc<Parser>, index: usize, address: String) -> (usize, Value) {
    let result = parser.parse(&address).await;
    let entry = serde_json::json!({
        "input": address,
        "result": result,
    });
    (index, entry)
}
