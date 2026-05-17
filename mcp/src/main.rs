use crate::server::ParseAddressServer;
use rmcp::ServiceExt;
use rmcp::transport::stdio;

mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = ParseAddressServer::new();
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
