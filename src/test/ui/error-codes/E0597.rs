// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![feature(rustc_attrs)]
struct Foo<'a> {
    x: Option<&'a u32>,
}

fn main() { #![rustc_error] // rust-lang/rust#49855
    let mut x = Foo { x: None };
    let y = 0;
    x.x = Some(&y);
    //~^ `y` does not live long enough [E0597]
}
