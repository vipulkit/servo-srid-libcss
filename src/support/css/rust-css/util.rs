/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use std::libc::c_void;

pub trait VoidPtrLike {
    fn from_void_ptr(ptr: *c_void) -> Self;
    fn to_void_ptr(&self) -> *c_void;
}


pub type DataStream = @fn() -> Option<~[u8]>;
