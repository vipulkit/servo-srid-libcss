// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::libc::c_void;

// FIXME: These methods should be unsafe
pub trait VoidPtrLike {
    fn from_void_ptr(ptr: *c_void) -> Self;
    fn to_void_ptr(&self) -> *c_void;
}
