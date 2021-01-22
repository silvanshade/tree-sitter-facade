#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::{borrow::Cow, convert::TryFrom};

    #[derive(Clone, Debug, PartialEq)]
    pub struct Language {
        pub(crate) inner: tree_sitter::Language,
    }

    impl Language {
        #[inline]
        pub fn field_count(&self) -> u32 {
            u32::try_from(self.inner.field_count()).unwrap()
        }

        #[inline]
        pub fn field_id_for_name(&self, field_name: impl AsRef<[u8]>) -> Option<u16> {
            let field_name = field_name.as_ref();
            self.inner.field_id_for_name(field_name)
        }

        #[inline]
        pub fn field_name_for_id(&self, field_id: u16) -> Option<Cow<str>> {
            self.inner.field_name_for_id(field_id).map(Into::into)
        }

        #[inline]
        pub fn id_for_node_kind(&self, kind: &str, named: bool) -> u16 {
            self.inner.id_for_node_kind(kind, named)
        }

        #[inline]
        pub fn node_kind_count(&self) -> u32 {
            u32::try_from(self.inner.node_kind_count()).unwrap()
        }

        #[inline]
        pub fn node_kind_for_id(&self, id: u16) -> Option<Cow<str>> {
            self.inner.node_kind_for_id(id).map(Into::into)
        }

        #[inline]
        pub fn node_kind_is_named(&self, id: u16) -> bool {
            self.inner.node_kind_is_named(id)
        }

        #[inline]
        pub fn node_kind_is_visible(&self, id: u16) -> bool {
            self.inner.node_kind_is_visible(id)
        }

        #[inline]
        pub fn version(&self) -> u32 {
            u32::try_from(self.inner.version()).unwrap()
        }
    }

    impl From<tree_sitter::Language> for Language {
        #[inline]
        fn from(inner: tree_sitter::Language) -> Self {
            Self { inner }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(target_arch = "wasm32")]
mod wasm {
    use std::borrow::Cow;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Language {
        pub(crate) inner: web_tree_sitter::Language,
    }

    impl Language {
        #[inline]
        pub fn field_count(&self) -> u16 {
            self.inner.field_count()
        }

        #[inline]
        pub fn field_id_for_name(&self, field_name: impl AsRef<[u8]>) -> Option<u16> {
            let field_name = field_name.as_ref();
            let field_name = unsafe { std::str::from_utf8_unchecked(field_name) };
            self.inner.field_id_for_name(field_name)
        }

        #[inline]
        pub fn field_name_for_id(&self, field_id: u16) -> Option<Cow<str>> {
            self.inner.field_name_for_id(field_id).map(Into::into)
        }

        // #[inline]
        // pub fn id_for_node_kind(&self, kind: &str, named: bool) -> u16 {
        //     unimplemented!()
        // }

        // #[inline]
        // pub fn node_kind_count(&self) -> u32 {
        //     unimplemented!()
        // }

        // #[inline]
        // pub fn node_kind_for_id(&self, id: u16) -> Option<Cow<str>> {
        //     unimplemented!()
        // }

        // #[inline]
        // pub fn node_kind_is_named(&self, id: u16) -> bool {
        //     unimplemented!()
        // }

        // #[inline]
        // pub fn node_kind_is_visible(&self, id: u16) -> bool {
        //     unimplemented!()
        // }

        #[inline]
        pub fn version(&self) -> u32 {
            self.inner.version()
        }
    }

    impl From<web_tree_sitter::Language> for Language {
        #[inline]
        fn from(inner: web_tree_sitter::Language) -> Self {
            Self { inner }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
