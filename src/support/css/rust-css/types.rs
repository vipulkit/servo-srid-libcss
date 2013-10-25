/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//use n;

extern mod srid_css;

use std::libc::types::common::c99::int32_t;

use srid_css::include::types::*;
use srid_css::select::common::*;

pub enum StylesheetOrigin {
    OriginUA,
    OriginUser,
    OriginAuthor
}

impl StylesheetOrigin {
    pub fn to_net(&self) -> css_origin {
        match *self {
            OriginUA => CSS_ORIGIN_UA,
            OriginUser => CSS_ORIGIN_USER,
            OriginAuthor => CSS_ORIGIN_AUTHOR
        }
    }
}

pub struct CssSelectResults {
    priv results: *css_select_results,
}

pub struct CssComputedStyle<'self> {
    // A borrowed back reference to ensure this outlives the results
    result_backref: &'self CssSelectResults,
    computed_style: *css_computed_style,
}

pub struct CssColor { b: u8, g: u8, r: u8, a: u8 }
pub type css_fixed = int32_t;


     