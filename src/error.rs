#[cfg(not(target_arch = "wasm32"))]
mod native {
    #[derive(Debug, Eq, PartialEq)]
    pub struct IncludedRangesError {
        pub(crate) inner: tree_sitter::IncludedRangesError,
    }

    impl From<tree_sitter::IncludedRangesError> for IncludedRangesError {
        #[inline]
        fn from(inner: tree_sitter::IncludedRangesError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct QueryError {
        pub(crate) inner: tree_sitter::QueryError,
    }

    impl From<tree_sitter::QueryError> for QueryError {
        #[inline]
        fn from(inner: tree_sitter::QueryError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct LanguageError {
        pub(crate) inner: tree_sitter::LanguageError,
    }

    impl From<tree_sitter::LanguageError> for LanguageError {
        #[inline]
        fn from(inner: tree_sitter::LanguageError) -> Self {
            Self { inner }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    #[derive(Debug, Eq, PartialEq)]
    pub struct IncludedRangesError {
        pub(crate) inner: js_sys::Error,
    }

    impl From<js_sys::Error> for IncludedRangesError {
        #[inline]
        fn from(inner: js_sys::Error) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct LanguageError {
        pub(crate) inner: web_tree_sitter::LanguageError,
    }

    impl From<web_tree_sitter::LanguageError> for LanguageError {
        #[inline]
        fn from(inner: web_tree_sitter::ParserError) -> Self {
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct QueryError {
        pub(crate) inner: web_tree_sitter::QueryError,
    }

    impl From<web_tree_sitter::QueryError> for QueryError {
        #[inline]
        fn from(inner: web_tree_sitter::QueryError) -> Self {
            Self { inner }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
