#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::{error::QueryError, language::Language};

    pub struct Query {
        pub inner: tree_sitter::Query,
    }

    impl Query {
        #[inline]
        pub fn new(language: &Language, source: &str) -> Result<Self, QueryError> {
            let inner = tree_sitter::Query::new(language.inner, source)?;
            Ok(Self { inner })
        }
    }

    impl From<tree_sitter::Query> for Query {
        #[inline]
        fn from(inner: tree_sitter::Query) -> Self {
            Self { inner }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::{error::QueryError, language::Language};

    pub struct Query {
        pub(crate) inner: web_tree_sitter::Query,
    }

    impl Query {
        #[inline]
        pub fn new(language: &Language, source: &str) -> Result<Self, QueryError> {
            let inner = language.inner.query(&source.into())?;
            Ok(Self { inner })
        }
    }

    impl Drop for Query {
        #[inline]
        fn drop(&mut self) {
            self.inner.delete();
        }
    }

    impl From<web_tree_sitter::Query> for Query {
        #[inline]
        fn from(inner: web_tree_sitter::Query) -> Self {
            Self { inner }
        }
    }

    unsafe impl Send for Query {
    }

    unsafe impl Sync for Query {
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
