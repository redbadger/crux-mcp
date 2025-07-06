pub mod core;

use rust_mcp_sdk::tool_box;

use core::{Resolve, Schema, Update, View};

tool_box!(MyTools, [Schema, Update, Resolve, View]);
