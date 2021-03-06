// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that a nominal type (like `Foo<'a>`) outlives `'b` if its
// arguments (like `'a`) outlive `'b`.
//
// Rule OutlivesNominalType from RFC 1214.

// compile-pass

#![feature(rustc_attrs)]
#![allow(dead_code)]

mod variant_struct_type {
    struct Foo<T> {
        x: T
    }
    struct Bar<'a,'b> {
        f: &'a Foo<&'b i32> //~ ERROR reference has a longer lifetime
    }
}

fn main() { }
