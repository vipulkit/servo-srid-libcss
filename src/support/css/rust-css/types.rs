/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//use n;

extern mod srid_css;

use srid_css::include::types::*;

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
     