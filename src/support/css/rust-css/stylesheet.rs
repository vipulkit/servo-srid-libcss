/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*!
CSS stylesheets, owned types, immutable after creation
*/
extern mod srid_css;

use extra::url::Url;
use util::DataStream;
//use netsurfcss::stylesheet::CssStylesheet;
use parser::{parse_stylesheet, parse_style_attribute};
use srid_css::css::*;
use srid_css::stylesheet::*;

pub struct Stylesheet {
    inner: @mut css_stylesheet,
}

impl Stylesheet {
    pub fn new(url: Url, input: DataStream) -> Stylesheet {
        let mut scss = parse_stylesheet(url, input) ;
        Stylesheet {
            inner:scss
        }
    }

    pub fn from_attribute(url: Url, data: &str) -> Stylesheet {
            let mut scss = parse_style_attribute(url, data) ;
        Stylesheet {
            inner: scss
        }
    }
}
