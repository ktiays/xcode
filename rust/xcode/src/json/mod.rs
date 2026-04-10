mod comments;
mod parser;
mod writer;

pub use parser::{parse, JsonParser};
pub use writer::{build, JsonWriter, WriterOptions};

pub type XcodeJson = serde_json::Value;
