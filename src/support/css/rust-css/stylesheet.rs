/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*!
CSS stylesheets, owned types, immutable after creation
*/
extern mod srid_css;
use extra::url::Url;
use util::DataStream;
use srid_css::css::*;  

//use netsurfcss::stylesheet::CssStylesheet;
use parser::{parse_stylesheet, parse_style_attribute};

pub struct Stylesheet {
    inner: uint
}

impl Stylesheet {
    pub fn new(url: Url, input: DataStream) -> Stylesheet {
        let stylesheet = parse_stylesheet(url, input);
        Stylesheet {
            inner:stylesheet.stylesheet
        }
    }

    pub fn from_attribute(url: Url, data: &str) -> Stylesheet {
        
        Stylesheet {
            inner: parse_style_attribute(url, data).stylesheet
        }
    }
}
