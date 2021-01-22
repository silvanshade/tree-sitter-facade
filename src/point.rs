#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::convert::TryFrom;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Point {
        pub(crate) inner: tree_sitter::Point,
    }

    impl Point {
        #[inline]
        pub fn column(&self) -> u32 {
            u32::try_from(self.inner.column).unwrap()
        }

        #[inline]
        pub fn row(&self) -> u32 {
            u32::try_from(self.inner.row).unwrap()
        }
    }

    impl From<tree_sitter::Point> for Point {
        #[inline]
        fn from(inner: tree_sitter::Point) -> Self {
            Self { inner }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Point {
        pub(crate) inner: web_tree_sitter::Point,
    }

    impl Point {
        #[inline]
        pub fn column(&self) -> u32 {
            self.inner.column()
        }

        #[inline]
        pub fn row(&self) -> u32 {
            self.inner.row()
        }
    }

    impl From<web_tree_sitter::Point> for Point {
        #[inline]
        fn from(inner: web_tree_sitter::Point) -> Self {
            Self { inner }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
