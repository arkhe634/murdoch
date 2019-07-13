[![crate-name at crates.io](https://img.shields.io/crates/v/murdoch-charsets.svg)](https://crates.io/crates/murdoch-charsets)
[![crate-name at docs.rs](https://docs.rs/murdoch-charsets/badge.svg)](https://docs.rs/murdoch-charsets)

# murdoch-charsets

## Murdoch-Charsets
`murdoch-charsets` provides API for name of character sets defined by Internet Assigned Numbers Authority (IANA).

In all settings, `murdoch-charset` provides `CharacterSet` traits and `InvalidCharacterSetNameError`.

### Default Setting

By default, `murdoch-charsets` provides types corresponding character sets defined by [IANA],
and `CharacterSetEnum` to hold any of them.

[IANA]: http://www.iana.org/assignments/character-sets/character-sets.xhtml

### Custom Character-Sets

You can customize the character sets by using `no_charset` feature

```cargo
[dependencies]
murdoch-charsets = { features = [ "no_charset" ] }
```

then `murdoch-charsets` exports `character_sets!` macro.

```fragment
character_sets!(
    UnicodeCharacterSet,
    UTF_8,                           // character-set type/variant name
    None,                            // preferred_mime_name
    "UTF-8",                         // name
    ["UTF8", "UTF_8",],              // aliases
    ["UTF-8", "UTF8", "UTF_8",],     // uppercase set of names (for FromStr)
    UTF_16,
    None,
    "UTF-16",
    ["UTF16", "UTF_16",],
    ["UTF-16", "UTF16", "UTF_16",],
    UTF_32,
    None,
    "UTF-32",
    ["UTF32", "UTF_32",],
    ["UTF-32", "UTF32", "UTF_32",],
);
```

By calling the macro as above, the following types and implementations of traits (`Debug`,
`Display`, `Copy`, `Clone`, `PartialEq`, `Eq`, `FromStr`, `AsRef<str>`, `AsRef<[u8]>`) are generated.

```fragment
pub struct UTF_8;
pub struct UTF_16;
pub struct UTF_32;
pub enum UnicodeCharacterSet
{
    UTF_8,
    UTF_16,
    UTF_32,
}
```

License: MIT
