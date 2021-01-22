#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::point::Point;
    use std::convert::TryFrom;

    pub struct InputEdit {
        pub(crate) inner: tree_sitter::InputEdit,
    }

    impl InputEdit {
        #[inline]
        pub fn new_end_byte(&self) -> u32 {
            u32::try_from(self.inner.new_end_byte).unwrap()
        }

        #[inline]
        pub fn new_end_position(&self) -> Point {
            let inner = self.inner.new_end_position;
            Point { inner }
        }

        #[inline]
        pub fn old_end_byte(&self) -> u32 {
            u32::try_from(self.inner.old_end_byte).unwrap()
        }

        #[inline]
        pub fn old_end_position(&self) -> Point {
            let inner = self.inner.old_end_position;
            Point { inner }
        }

        #[inline]
        pub fn start_byte(&self) -> u32 {
            u32::try_from(self.inner.start_byte).unwrap()
        }

        #[inline]
        pub fn start_position(&self) -> Point {
            let inner = self.inner.start_position;
            Point { inner }
        }
    }

    impl From<tree_sitter::InputEdit> for InputEdit {
        #[inline]
        fn from(inner: tree_sitter::InputEdit) -> Self {
            Self { inner }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::point::Point;

    pub struct InputEdit {
        pub(crate) inner: web_tree_sitter::Edit,
    }

    impl InputEdit {
        #[inline]
        pub fn new_end_byte(&self) -> u32 {
            self.inner.new_end_index()
        }

        #[inline]
        pub fn new_end_position(&self) -> Point {
            let inner = self.inner.new_end_position();
            Point { inner }
        }

        #[inline]
        pub fn old_end_byte(&self) -> u32 {
            self.inner.old_end_index()
        }

        #[inline]
        pub fn old_end_position(&self) -> Point {
            let inner = self.inner.old_end_position();
            Point { inner }
        }

        #[inline]
        pub fn start_byte(&self) -> u32 {
            self.inner.start_index()
        }

        #[inline]
        pub fn start_position(&self) -> Point {
            let inner = self.inner.start_position();
            Point { inner }
        }
    }

    impl From<web_tree_sitter::Edit> for InputEdit {
        #[inline]
        fn from(inner: web_tree_sitter::Edit) -> Self {
            Self { inner }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
