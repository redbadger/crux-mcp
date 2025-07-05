mod core;
mod say_hello;

use rust_mcp_sdk::tool_box;

use core::CruxCoreTool;
use say_hello::SayHelloTool;

tool_box!(MyTools, [CruxCoreTool, SayHelloTool]);
