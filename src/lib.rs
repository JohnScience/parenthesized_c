//! This is a simple library providing [`ParenthesizedC`] [ZST] that implements [syn::parse::Parse] and
//! thus, for example, can be used with [syn::parse2]\(\) for the purpose of parsing `(C)` [attribute tokens](https://docs.rs/syn/1.0.84/syn/struct.Attribute.html)
//! as in [`#[repr(C)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc) [outer attribute](https://doc.rust-lang.org/reference/attributes.html).
//!
//! # Example:
//!
//! ```rust
//! extern crate syn;
//! extern crate outer_attribute;
//! extern crate parenthesized_c;
//!
//! use parenthesized_c::ParenthesizedC;
//! use outer_attribute::different_layout::OuterAttribute;
//!
//! fn main() -> syn::Result<()> {
//!     let repr_c = syn::parse_str::<OuterAttribute>("#[repr(C)]")?;
//!     let repr_transpartent = syn::parse_str::<OuterAttribute>("#[repr(transparent)]")?;
//!     assert!(matches!(syn::parse2::<ParenthesizedC>(repr_c.tokens), Ok(_)));
//!     assert!(matches!(syn::parse2::<ParenthesizedC>(repr_transpartent.tokens), Err(_)));
//!     Ok(())
//! }
//! ```
//!
//! [ZST]: https://runrust.miraheze.org/wiki/Zero-sized_type

use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    Ident, Result,
};

/// A [ZST] that implements [syn::parse::Parse] and thus, for example, can be used with [syn::parse2]\(\)
/// for the purpose of parsing `(C)` [attribute tokens](https://docs.rs/syn/1.0.84/syn/struct.Attribute.html)
/// as in [`#[repr(C)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc)
/// [outer attribute](https://doc.rust-lang.org/reference/attributes.html).
///
/// [ZST]: https://runrust.miraheze.org/wiki/Zero-sized_type
#[derive(Default)]
pub struct ParenthesizedC;

impl Parse for ParenthesizedC {
    fn parse(input: ParseStream) -> Result<Self> {
        let enclosed_content;
        parenthesized!(enclosed_content in input);
        if enclosed_content.parse::<Ident>()? != "C" {
            return Err(enclosed_content.error("Unexpected data layout. Expected: C."))
        };
        Ok(ParenthesizedC::default())
    }
}
