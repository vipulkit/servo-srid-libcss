/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/**
Constructs a list of css style rules from a token stream
*/

// TODO: fail according to the css spec instead of failing when things
// are not as expected

extern mod srid_css;
use util::DataStream;
use extra::url::Url;
use srid_css::css::*;
use srid_css::libwapcaplet::wapcaplet::*;  
use srid_css::parse::propstrings::*;  
use srid_css::stylesheet::*; 
use srid_css::utils::errors::*;  
use srid_css::stylesheet::css_url_resolution_fn;  

fn default_params(url: Url) -> css_params {
    let resolve: css_url_resolution_fn = resolve_url;
    css_params {
        params_version: CSS_PARAMS_VERSION_1,
        level: CSS_LEVEL_21,
        charset: Some(~"UTF-8"),
        url: url.to_str(),
        title: ~"FIXME-css-title",
        allow_quirks: false,
        inline_style: false,
        resolve: resolve,
        import: None,
        color: None,
        font: None,
    }
}

// This takes a DataStreamFactory instead of a DataStream because
// servo's DataStream contains a comm::Port, which is not sendable,
// so DataStream is an @fn which can't be sent to the lexer task.
// So the DataStreamFactory gives the caller an opportunity to create
// the data stream from inside the lexer task.
pub fn parse_stylesheet(url: Url, input: DataStream) -> ~css {
    let params = default_params(url);
    let mut sheet = css::css_create(&params);
    lwc();
    let propstring = css_propstrings::css_propstrings(unsafe{lwc_ref.get_mut_ref()}); 

    loop {
        match input() {
            Some(data) => {
                sheet.css_stylesheet_append_data(&propstring, data);
            }
            None => break
        }
    }
    sheet.css_stylesheet_data_done(&propstring);
    sheet
}

pub fn parse_style_attribute(url: Url, data: &str) -> ~css {
    let mut params = default_params(url);
    params.inline_style = true;
    lwc();  
    let propstring = css_propstrings::css_propstrings(unsafe{lwc_ref.get_mut_ref()}); 
    let mut sheet = css::css_create(&params);
    sheet.css_stylesheet_append_data(&propstring, data.as_bytes().to_owned());
    sheet.css_stylesheet_data_done(&propstring);
    sheet
}

fn resolve_url(_base: &str, _rel: uint) -> (css_error,Option<uint>)  {
    //fail!(~"resolving url");
    (CSS_OK,None) 
}
