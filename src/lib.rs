mod error;
mod input_edit;
mod language;
mod logger;
mod node;
mod parser;
mod point;
mod query;
mod range;
mod tree;
mod tree_cursor;

pub use error::*;
pub use input_edit::*;
pub use language::*;
pub use logger::*;
pub use node::*;
pub use parser::*;
pub use point::*;
pub use query::*;
pub use range::*;
pub use tree::*;
pub use tree_cursor::*;

pub struct TreeSitter;

impl TreeSitter {
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn init() -> anyhow::Result<()> {
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn init() -> anyhow::Result<()> {
        use js_sys::JsString;
        use wasm_bindgen::JsValue;
        web_tree_sitter::TreeSitter::init()
            .await
            .map_err(Into::<JsValue>::into)
            .map_err(JsString::from)
            .map_err(String::from)
            .map_err(anyhow::Error::msg)
    }
}
