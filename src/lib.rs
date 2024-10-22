/*!
A collection of excellent utilities for nom, including:

- [`ParserExt`][parser_ext::ParserExt], a trait which makes available
  many common nom parser combinators as postfix methods, to complement
  those already available on [`nom::Parser`].
- [`ErrorTree`][error::ErrorTree], a nom error which retains as much
  information and context as possible about the details of the failed
  parse, with an excellent indenting formatter for printing these failures.
  Integrates with the extra error features of `nom-supreme`.
- Improved [`tag`] parsers, which attach the mismatched the error in the
  event of a parse failure, similar to
  [`char`][nom::character::complete::char].
- [`parse_separated_terminated`][multi::parse_separated_terminated], the
  perfected folding parser for building parse loops.
- [`final_parser`][final_parser::final_parser], which serves as a bridge
  between nom-style [`IResult`][nom::IResult] parsers and more typical rust
  results. It decorates a nom parser, requiring it to parse all of its
  input, not return `Incomplete`. It also uses an
  [`ExtractContext`][final_parser::ExtractContext] trait to convert the
  error locations in nom errors, which are usually just suffixes of the
  input, into more useful locations, such as a line and column number.
*/
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg(feature = "alloc")]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]
#![allow(clippy::doc_markdown)]
#![deny(missing_docs)]
#[macro_use]
extern crate alloc;


/**
Call a method or methods on an object, and then return the original object.

The input to this macro is the methods calls written in a chaining style:
`express!(vec.push(1).push(2).push(3))`, which is transformed into a series
of calls on the original object, then returning that object.
*/
macro_rules! express {
    ($thing:ident $( . $method:ident ( $($args:tt)* ) )*) => {{
        let mut thing = $thing;
        $( thing.$method($($args)*); )*
        thing
    }};
}

/// Lib module to re-export everything needed from `std` or `core`/`alloc`. This is how `serde` does
/// it, albeit there it is not public.
pub mod lib {
    /// `std` facade allowing `std`/`core` to be interchangeable. Reexports `alloc` crate optionally,
    /// as well as `core` or `std`
    #[cfg(not(feature = "std"))]
    /// internal std exports for no_std compatibility
    pub mod std {
        #[doc(hidden)]
        #[cfg(not(feature = "alloc"))]
        pub use core::borrow;

        #[cfg(feature = "alloc")]
        pub use alloc::{borrow, boxed, string, vec};

        #[doc(hidden)]
        pub use core::{cmp, convert, fmt, iter, mem, num, ops, option, result, slice, str};

        pub use core::error;

        /// internal reproduction of std prelude
        #[doc(hidden)]
        pub mod prelude {
            pub use core::prelude as v1;
        }
    }

    #[cfg(feature = "std")]
    /// internal std exports for no_std compatibility
    pub mod std {
        #[doc(hidden)]
        pub use std::{
            alloc, borrow, boxed, cmp, collections, convert, fmt, hash, iter, mem, num, ops, option,
            result, slice, str, string, vec,
        };

        /// internal reproduction of std prelude
        #[doc(hidden)]
        pub mod prelude {
            pub use std::prelude as v1;
        }
    }
}


pub mod context;
pub mod error;
pub mod final_parser;
pub mod multi;
pub mod parser_ext;
pub mod tag;

pub use parser_ext::ParserExt;



/// test documentation
#[test]
pub fn test() {

}