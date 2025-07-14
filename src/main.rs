mod core;
mod handler;
mod tools;

use handler::MyServerHandler;
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, LATEST_PROTOCOL_VERSION, ServerCapabilities,
    ServerCapabilitiesTools,
};

use rust_mcp_sdk::{
    McpServer, StdioTransport, TransportOptions, error::SdkResult, mcp_server::server_runtime,
};

#[tokio::main]
async fn main() -> SdkResult<()> {
    let server_details = InitializeResult {
        server_info: Implementation {
            name: "Crux app MCP Server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            title: None,
        },
        capabilities: ServerCapabilities {
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default()
        },
        meta: None,
        instructions: Some("server instructions...".to_string()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    let server = server_runtime::create_server(
        server_details,
        StdioTransport::new(TransportOptions::default())?,
        MyServerHandler::new(),
    );

    server.start().await
}
