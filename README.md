This is a simple library providing [`ParenthesizedC`] [ZST] that implements [`syn::parse::Parse`] and
thus, for example, can be used with [`syn::parse2`]\(\) for the purpose of parsing `(C)` [attribute tokens][attribute token] as in [#[repr(C)]][`#\[repr(C)\]`] [outer attribute].

# Example:

```rust
extern crate syn;
extern crate outer_attribute;
extern crate parenthesized_c;

use parenthesized_c::ParenthesizedC;
use outer_attribute::different_layout::OuterAttribute;

fn main() -> syn::Result<()> {
    let repr_c = syn::parse_str::<OuterAttribute>("#[repr(C)]")?;
    let repr_transpartent = syn::parse_str::<OuterAttribute>("#[repr(transparent)]")?;
    assert!(matches!(syn::parse2::<ParenthesizedC>(repr_c.tokens), Ok(_)));
    assert!(matches!(syn::parse2::<ParenthesizedC>(repr_transpartent.tokens), Err(_)));
    Ok(())
}
```

[`ParenthesizedC`]: https://docs.rs/parenthesized_c/latest/parenthesized_c/struct.ParenthesizedC.html
[ZST]: https://runrust.miraheze.org/wiki/Zero-sized_type
[`syn::parse::Parse`]: https://docs.rs/syn/latest/syn/parse/trait.Parse.html
[`syn::parse2`]: https://docs.rs/syn/latest/syn/fn.parse2.html
[attribute token]: https://docs.rs/syn/1.0.84/syn/struct.Attribute.html
[`#\[repr(C)\]`]: https://doc.rust-lang.org/nomicon/other-reprs.html#reprc
[outer attribute]: https://doc.rust-lang.org/reference/attributes.html

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
