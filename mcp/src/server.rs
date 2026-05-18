mod process_address_list;
mod process_an_address;

use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, Implementation, ServerCapabilities, ServerInfo};
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};

#[derive(Clone)]
pub(crate) struct ParseAddressServer {
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl ParseAddressServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "日本の住所を都道府県・市区町村・町名・それ以降に分割できます")]
    async fn process_an_address(
        &self,
        Parameters(params): Parameters<process_an_address::RequestParameters>,
    ) -> Result<CallToolResult, ErrorData> {
        process_an_address::process_an_address(params).await
    }

    #[tool(description = "最大100件までの複数の住所を一括で処理できます")]
    async fn process_address_list(
        &self,
        Parameters(params): Parameters<process_address_list::RequestParameters>,
    ) -> Result<CallToolResult, ErrorData> {
        process_address_list::process_address_list(params).await
    }
}

#[tool_handler]
impl ServerHandler for ParseAddressServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "日本の住所を都道府県・市区町村・町名・それ以降に分割できるMCPサーバーです。 \
                process_an_address: 1件の住所を解析 \
                process_address_list: 複数の住所を一括で解析",
            )
    }
}
