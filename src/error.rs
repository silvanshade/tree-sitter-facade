#[cfg(not(target_arch = "wasm32"))]
mod native {
    #[derive(Debug, Eq, PartialEq)]
    pub struct IncludedRangesError {
        pub(crate) inner: tree_sitter::IncludedRangesError,
    }

    impl From<tree_sitter::IncludedRangesError> for IncludedRangesError {
        #[inline]
        fn from(error: tree_sitter::IncludedRangesError) -> Self {
            let inner = error;
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct QueryError {
        pub(crate) inner: tree_sitter::QueryError,
    }

    impl From<tree_sitter::QueryError> for QueryError {
        #[inline]
        fn from(error: tree_sitter::QueryError) -> Self {
            let inner = error;
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct LanguageError {
        pub(crate) inner: tree_sitter::LanguageError,
    }

    impl From<tree_sitter::LanguageError> for LanguageError {
        #[inline]
        fn from(error: tree_sitter::LanguageError) -> Self {
            let inner = error;
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
        fn from(error: js_sys::Error) -> Self {
            let inner = error;
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct LanguageError {
        pub(crate) inner: web_tree_sitter::LanguageError,
    }

    impl From<web_tree_sitter::LanguageError> for LanguageError {
        #[inline]
        fn from(error: web_tree_sitter::LanguageError) -> Self {
            let inner = error;
            Self { inner }
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct QueryError {
        pub(crate) inner: web_tree_sitter::QueryError,
    }

    impl From<web_tree_sitter::QueryError> for QueryError {
        #[inline]
        fn from(error: web_tree_sitter::QueryError) -> Self {
            let inner = error;
            Self { inner }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
