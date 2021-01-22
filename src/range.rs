#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::point::Point;
    use std::convert::TryFrom;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Range {
        pub(crate) inner: tree_sitter::Range,
    }

    impl Range {
        #[inline]
        pub fn end_byte(&self) -> u32 {
            u32::try_from(self.inner.end_byte).unwrap()
        }

        #[inline]
        pub fn end_point(&self) -> Point {
            let inner = self.inner.end_point;
            Point { inner }
        }

        #[inline]
        pub fn start_byte(&self) -> u32 {
            u32::try_from(self.inner.start_byte).unwrap()
        }

        #[inline]
        pub fn start_point(&self) -> Point {
            let inner = self.inner.start_point;
            Point { inner }
        }
    }

    impl From<tree_sitter::Range> for Range {
        #[inline]
        fn from(inner: tree_sitter::Range) -> Self {
            Self { inner }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::point::Point;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Range {
        pub(crate) inner: web_tree_sitter::Range,
    }

    impl Range {
        #[inline]
        pub fn end_byte(&self) -> u32 {
            self.inner.end_index()
        }

        #[inline]
        pub fn end_point(&self) -> Point {
            let inner = self.inner.end_position();
            Point { inner }
        }

        #[inline]
        pub fn start_byte(&self) -> u32 {
            self.inner.start_index()
        }

        #[inline]
        pub fn start_point(&self) -> Point {
            let inner = self.inner.start_position();
            Point { inner }
        }
    }

    impl From<web_tree_sitter::Range> for Range {
        #[inline]
        fn from(inner: web_tree_sitter::Range) -> Self {
            Self { inner }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
