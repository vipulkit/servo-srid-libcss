/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*!
Selector matching

Select matching is performed on generic node types. Client-specific details
about the DOM are encapsulated in the `SelectHandler` type which the `SelectCtx`
uses to query various DOM and UA properties.
*/
extern mod srid_css;
use std::cast::transmute;
use std::libc::*;
use stylesheet::Stylesheet;
use computed::ComputedStyle;
use helpers::VoidPtrLike;
use types::StylesheetOrigin;

use srid_css::include::types::*;
use srid_css::utils::errors::*;
use helpers::select::*;
use helpers::properties::*;
use helpers::hint::*;
use helpers::types::CssQName;
use srid_css::select::common::*;
use srid_css::libwapcaplet::wapcaplet::*;
//use srid_css::select::select::*;
use srid_css::stylesheet::css_qname;

//use srid_css::select::select::css_select_ctx;

pub struct SelectCtx {
    inner: CssSelectCtx
}

pub struct UntypedHandler<'self> {
    node_name: &'self fn(node: *c_void, qname: &mut css_qname) -> css_error,
    node_classes: &'self fn(node: *c_void, classes: &mut ~[uint]) -> css_error,
    node_id: &'self fn(node: *c_void, id: &mut uint) -> css_error,
    named_parent_node: &'self fn(node: *c_void, qname: &mut css_qname, parent: *mut *c_void) -> css_error,
    parent_node: &'self fn(node: *c_void, parent: *mut *c_void) -> css_error,
    node_has_class: &'self fn(node: *c_void, name: uint, match_: &mut bool) -> css_error,
    node_has_id: &'self fn(node: *c_void, name: uint, match_: &mut bool) -> css_error,
    named_ancestor_node: &'self fn(node: *c_void,
                             qname: &mut css_qname,
                             parent: *mut *c_void) -> css_error,
    node_is_root: &'self fn(node: *c_void, match_: &mut bool) -> css_error,
    node_is_link: &'self fn(node: *c_void, match_: &mut bool) -> css_error,
    node_is_visited: &'self fn(node: *c_void, match_: &mut bool) -> css_error,
    ua_default_for_property: &'self fn(property: u32, hint: &mut ~css_hint) -> css_error,
}


fn unimpl(n: &str) -> ! {
    fail!(fmt!("unimplemented css callback handler: %s", n))
}
fn unimpl_warn(what: &str) {
    warn!("unimplemented css value: %?", what);
}
fn enter(n: &str) {
    debug!("entering raw handler: %s", n);
}
fn ph<'a>(pw: *c_void) -> &'a UntypedHandler<'a> {
    unsafe { transmute(pw) }
}
pub fn node_name(pw: *c_void, node: *c_void, qname: &mut css_qname) -> css_error {
    enter("node_name");
    (ph(pw).node_name)(node, qname)
}
pub fn node_classes(_lwc: &mut ~lwc, pw: *c_void, node: *c_void, classes: &mut ~[uint]) -> css_error {
    enter("node_classes");
    (ph(pw).node_classes)(node, classes)
}
pub fn node_id(_lwc: &mut ~lwc, pw: *c_void, node: *c_void, id: &mut uint) -> css_error {
    enter("node_id");
    (ph(pw).node_id)(node, id)
}
pub fn named_ancestor_node(
                                  pw: *c_void,
                                  _lwc: &mut ~lwc, 
                                  node: *c_void,
                                  qname: &mut css_qname,
                                  parent: *mut *c_void) -> css_error {
    enter("named_ancestor_node");
    (ph(pw).named_ancestor_node)(node, qname, parent)
}
pub fn named_parent_node(pw: *c_void, _lwc: &mut ~lwc, node: *c_void, qname: &mut css_qname, parent: *mut *c_void) -> css_error {
    enter("named_parent_node");
    (ph(pw).named_parent_node)(node, qname, parent)
}
pub fn named_sibling_node(_pw: *c_void, _lwc: &mut ~lwc,  node: *c_void, _qname: &mut css_qname, sibling: *mut *c_void) -> css_error {
    unimpl_warn("named_sibling_node");
    unsafe {
        *sibling = node;
        CSS_OK
    }
}
pub fn named_generic_sibling_node(_pw: *c_void, _lwc_ref: &mut ~lwc , _node: *c_void, _qname: &mut css_qname, _sibling: *mut *c_void) -> css_error {
    unimpl("named_generic_sibling_node")
}
pub fn parent_node(pw: *c_void, node: *c_void, parent: *mut *c_void) -> css_error {
    enter("parent_node");
    (ph(pw).parent_node)(node, parent)
}
pub fn sibling_node(_pw: *c_void, node: *c_void, sibling: *mut *c_void) -> css_error {
    unimpl_warn("sibling_node");
    unsafe {
        *sibling = node;
        CSS_OK
    }
}
pub fn node_has_name(_lwc_ref: &mut ~lwc ,_pw: *c_void, _node: *c_void, _qname: &css_qname, _match_: &mut bool) -> css_error {
    unimpl("node_has_name")
}
pub fn node_has_class(_lwc_ref: &mut ~lwc , pw: *c_void, node: *c_void, name: uint, match_: &mut bool) -> css_error {
    enter("node_has_class");
    (ph(pw).node_has_class)(node, name, match_)
}
pub fn node_has_id(_lwc_ref: &mut ~lwc , pw: *c_void, node: *c_void, name: uint, match_: &mut bool) -> css_error {
    enter("node_has_id");
    (ph(pw).node_has_id)(node, name, match_)
}
pub fn node_has_attribute(_pw: *c_void, _lwc_ref: &mut ~lwc , _node: *c_void, _qname: &css_qname, match_: &mut bool) -> css_error {
    unimpl_warn("node_has_attribute");
        *match_ = false;
        CSS_OK
}
pub fn node_has_attribute_equal(_pw: *c_void, _lwc_ref: &mut ~lwc ,  _node: *c_void, _qname: &css_qname, _value: uint, match_: &mut bool) -> css_error {
    unimpl_warn("node_has_attribute_equal");
        *match_ = false;
        CSS_OK
}
pub fn node_has_attribute_dashmatch(_lwc_ref: &mut ~lwc , _node: *c_void, _qname: &css_qname, _value: uint, _match_: &mut bool) -> css_error {
    unimpl("node_has_attribute_dashmatch")
}
pub fn node_has_attribute_includes(_lwc_ref: &mut ~lwc , _node: *c_void, _qname: &css_qname, _value: uint, match_: &mut bool) -> css_error {
    unimpl_warn("node_has_attribute_includes");
        *match_ = false;
        CSS_OK
}
pub fn node_has_attribute_prefix(_lwc_ref: &mut ~lwc , _node: *c_void, _qname: &css_qname, _value: uint, match_: &mut bool) -> css_error {
    unimpl_warn("node_has_attribute_prefix");
        *match_ = false;
        CSS_OK
}
pub fn node_has_attribute_suffix(_lwc_ref: &mut ~lwc , _node: *c_void, _qname: &css_qname, _value: uint, match_: &mut bool) -> css_error {
    unimpl_warn("node_has_attribute_suffix");
        *match_ = false;
        CSS_OK
}
pub fn node_has_attribute_substring(_lwc_ref: &mut ~lwc , _node: *c_void, _qname: &css_qname, _value: uint, match_: &mut bool) -> css_error {
    unimpl_warn("node_has_attribute_substring");
        *match_ = false;
        CSS_OK
}
pub fn node_is_root(pw: *c_void, node: *c_void, match_: &mut bool) -> css_error {
    enter("node_is_root");
    (ph(pw).node_is_root)(node, match_)
}
pub fn node_count_siblings(_lwc_ref: &mut ~lwc , _node: *c_void, _same_name: bool, _after: bool, count: &mut i32) -> css_error {
    unimpl_warn("node_count_siblings");
        *count = 0;
        CSS_OK
}
pub fn node_is_empty(_node: *c_void, match_: &mut bool) -> css_error {
    unimpl_warn("node_is_empty");
        *match_ = true;
        CSS_OK
}
pub fn node_is_link(pw: *c_void, node: *c_void, match_: &mut bool) -> css_error {
    enter("node_is_link");
    (ph(pw).node_is_link)(node, match_)
}
pub fn node_is_visited(pw: *c_void, node: *c_void, match_: &mut bool) -> css_error {
    enter("node_is_visited");
    (ph(pw).node_is_visited)(node, match_)
}
pub fn node_is_hover(_node: *c_void, _match_: &mut bool) -> css_error {
    unimpl_warn("node_is_hover");
    CSS_OK
}
pub fn node_is_active(_node: *c_void, match_: &mut bool) -> css_error {
    unimpl_warn("node_is_active");
        *match_ = false;
        CSS_OK
}
pub fn node_is_focus(_node: *c_void, match_: &mut bool) -> css_error {
    unimpl_warn("node_is_focus");
        *match_ = false;
        CSS_OK
}
pub fn node_is_enabled(_node: *c_void, _match_: &mut bool) -> css_error {
    unimpl("node_is_enabled")
}
pub fn node_is_disabled(_node: *c_void, _match_: &mut bool) -> css_error {
    unimpl("node_is_disabled")
}
pub fn node_is_checked(_node: *c_void, _match_: &mut bool) -> css_error {
    unimpl("node_is_checked")
}
pub fn node_is_target(_node: *c_void, match_: &mut bool) -> css_error {
    unimpl_warn("node_is_target");
        *match_ = false;
        CSS_OK
}
pub fn node_is_lang(_node: *c_void, _lang: uint, match_: &mut bool) -> css_error {
    unimpl_warn("node_is_lang");
        *match_ = false;
        CSS_OK
}
pub fn node_presentational_hint(_node: *c_void, _property: u32) -> (css_error , Option<~css_hint>) {
    enter("node_presentational_hint");
    (CSS_PROPERTY_NOT_SET,None)
}
pub fn ua_default_for_property(pw: *c_void, property: u32, hint: &mut ~css_hint) -> css_error {
    enter("ua_default_for_property");
    (ph(pw).ua_default_for_property)(property, hint)
}
pub fn compute_font_size(/*_pw: *c_void ,*/ parent: Option<&mut ~css_hint>, size: Option<&mut ~css_hint>) -> css_error {
    enter("compute_font_size");
    // FIXME: This should be merged with the one in rust-css, I think. --pcwalton
    let parent_hint;
    if parent.is_none() {
        parent_hint = CssHintLength(types::CssUnitPx(16 * 1024));
    } else {
        parent_hint = CssHint::new(CssPropFontSize, parent);
    }
    parent_hint.write_to_ll(CssPropFontSize, size.unwrap());
    CSS_OK
}


/**
The SelectCtx, used for performing selector matching.

The `SelectCtx` takes ownership of any number of `Stylesheet` objects,
encapsulates the cascade. Individual node styles can be requested with
the `select_style` method.
*/
impl SelectCtx {
    pub fn new() -> SelectCtx {
        SelectCtx {
            inner: css_select_ctx_create()
        }
    }

    /**
    Add `Stylesheet`s to the selection context, where they will participate in the cascade
    during future selector matching
    */
    pub fn append_sheet(&mut self, sheet: uint, origin: StylesheetOrigin) {
        
        self.inner.append_sheet(sheet, origin.to_net(), CSS_MEDIA_SCREEN as u64);
    }

    /**
    Select the style for a single node. `handler` is used to query the client for
    a wide range of client-specific details like node relationships, names, and UA
    defaults.
    */
    pub fn select_style<N: VoidPtrLike, H: SelectHandler<N>>(&mut self,
                                                             node: &N,
                                                             inline_style: Option<uint>,
                                                             handler: &H) -> SelectResults {
        let inner_handler = SelectHandlerWrapper {
            inner: handler
        };
        /*let inner_inline_style = match inline_style {
            None => None,
            Some(ref sheet) => Some(&sheet.inner),
        };*/

        SelectResults {
        inner:self.inner.select_style::<N, SelectHandlerWrapper<N, H>>(
                node,
                CSS_MEDIA_SCREEN as u64,
                inline_style,
                &inner_handler)
        }
    }
}

/**
Represents the 'style' of a single node, including it's pseudo-elements.
*/
pub struct SelectResults {
    inner: ~CssSelectResults
}

impl<'self> SelectResults {
    /** Retrieve the computed style of a single pseudo-element */
    pub fn computed_style(&'self self) -> ComputedStyle<'self> {
        ComputedStyle {
            inner: self.inner.computed_style(CssPseudoElementNone)
        }
    }
}


