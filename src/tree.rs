#[cfg(not(target_arch = "wasm32"))]
mod native {
    use crate::{input_edit::InputEdit, language::Language, node::Node, range::Range, tree_cursor::TreeCursor};

    #[derive(Clone, Debug)]
    pub struct Tree {
        pub(crate) inner: tree_sitter::Tree,
    }

    impl Tree {
        pub fn edit(&mut self, edit: &InputEdit) {
            self.inner.edit(&edit.inner);
        }

        pub fn changed_ranges(&self, other: &Tree) -> impl ExactSizeIterator<Item = Range> {
            self.inner.changed_ranges(&other.inner).map(|inner| Range { inner })
        }

        pub fn language(&self) -> Language {
            self.inner.language().into()
        }

        pub fn root_node(&self) -> Node<'_> {
            self.inner.root_node().into()
        }

        pub fn walk(&self) -> TreeCursor<'_> {
            self.inner.walk().into()
        }
    }

    impl From<tree_sitter::Tree> for Tree {
        #[inline]
        fn from(inner: tree_sitter::Tree) -> Self {
            Self { inner }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use crate::{input_edit::InputEdit, language::Language, node::Node, tree_cursor::TreeCursor};

    #[derive(Debug)]
    pub struct Tree {
        pub(crate) inner: web_tree_sitter::Tree,
    }

    impl Tree {
        pub fn edit(&mut self, edit: &InputEdit) {
            let edit = {
                let start_index = edit.start_byte();
                let old_end_index = edit.old_end_byte();
                let new_end_index = edit.new_end_byte();
                let start_position = edit.start_position().inner;
                let old_end_position = edit.old_end_position().inner;
                let new_end_position = edit.new_end_position().inner;
                web_tree_sitter::Edit::new(
                    start_index,
                    old_end_index,
                    new_end_index,
                    &start_position,
                    &old_end_position,
                    &new_end_position,
                )
            };
            self.inner.edit(&edit);
        }

        // FIXME: implement bindings upstream first
        // pub fn changed_ranges(&self, other: &Tree) -> impl ExactSizeIterator<Item = Range> {
        //     unimplemented!()
        // }

        pub fn language(&self) -> Language {
            self.inner.get_language().into()
        }

        pub fn root_node(&self) -> Node<'_> {
            self.inner.root_node().into()
        }

        pub fn walk(&self) -> TreeCursor<'_> {
            self.inner.walk().into()
        }
    }

    impl Clone for Tree {
        fn clone(&self) -> Tree {
            self.inner.copy().into()
        }
    }

    impl Drop for Tree {
        fn drop(&mut self) {
            self.inner.delete();
        }
    }

    impl From<web_tree_sitter::Tree> for Tree {
        #[inline]
        fn from(inner: web_tree_sitter::Tree) -> Self {
            Self { inner }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
