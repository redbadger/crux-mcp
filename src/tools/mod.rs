pub mod core;
mod say_hello;

use rust_mcp_sdk::tool_box;

use core::{Resolve, Update, View};
use say_hello::SayHelloTool;

tool_box!(MyTools, [Update, Resolve, View, SayHelloTool]);
