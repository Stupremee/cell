//! Various macros that help developing the compiler.

/// Macro to generate a struct that is used as an id for
/// interned data by [`salsa`].
///
/// The macro will create a struct with the given name
/// and will automatically implement [`salsa::InternKey`] and other useful
/// traits like `Clone`, `Copy`, `Hash`, etc.
///
/// # Example
///
/// ```
/// # use cell_common::intern_id_struct;
///
/// intern_id_struct! {
///     /// An id used to intern files.
///     pub struct FileId;
/// }
/// ```
///
/// [`salsa`]: https://docs.rs/salsa
/// [`salsa::InternKey`]: https://docs.rs/salsa/0.15.2/salsa/trait.InternKey.html
#[macro_export]
macro_rules! intern_id_struct {
    ($(#[$meta:meta])*
     pub struct $name:ident;
    ) => {
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        $(#[$meta])*
        pub struct $name(salsa::InternId);

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl salsa::InternKey for $name {
            fn from_intern_id(id: salsa::InternId) -> Self {
                Self(id)
            }

            fn as_intern_id(&self) -> salsa::InternId {
                self.0
            }
        }
    };
}
