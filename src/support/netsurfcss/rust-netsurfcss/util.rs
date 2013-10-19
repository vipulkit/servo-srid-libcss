// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ll::stylesheet::css_fixed;


pub fn css_fixed_to_float(f: css_fixed) -> float {
    static BEFORE: i32 = 10;
    f as float * 1.0f / ((1i32 << BEFORE) as float)
}

