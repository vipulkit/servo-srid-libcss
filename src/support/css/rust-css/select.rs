/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*!
Selector matching

Select matching is performed on generic node types. Client-specific details
about the DOM are encapsulated in the `SelectHandler` type which the `SelectCtx`
uses to query various DOM and UA properties.
*/
//extern mod std;
extern mod srid_css;
extern mod wapcaplet;

use std::cast::transmute;
use std::libc::*;
//use util::VoidPtrLike;
//use wapcaplet::LwcString;
//use lwcstr_from_rust_str = wapcaplet::from_rust_string;
//use n::u::{rust_str_to_net_qname, net_qname_to_rust_str};
use types::StylesheetOrigin;
//use n;
use srid_css::stylesheet::*;
use srid_css::select::select::*;
use srid_css::select::common::*;
use srid_css::utils::errors::*;
use srid_css::include::types::*;
use srid_css::include::properties::*;
use wapcaplet::*;
use values::*;
use stylesheet::Stylesheet;
use computed::ComputedStyle;

pub struct SelectCtx {
    inner: css_select_ctx
}

/**
The SelectCtx, used for performing selector matching.

The `SelectCtx` takes ownership of any number of `Stylesheet` objects,
encapsulates the cascade. Individual node styles can be requested with
the `select_style` method.
*/
// FIXME: These methods should be unsafe
pub trait VoidPtrLike {
    fn from_void_ptr(ptr: *c_void) -> Self;
    fn to_void_ptr(&self) -> *c_void;
}

pub struct UntypedHandler{
    node_name: @fn(node: *c_void, qname: *mut css_qname) -> css_error,
    node_classes: @fn(node: *c_void, classes: *mut **lwc_string, n_classes: *mut uint32_t) -> css_error,
    node_id: @fn(node: *c_void, id: *mut *lwc_string) -> css_error,
    named_parent_node: @fn(node: *c_void, qname: *css_qname, parent: *mut *c_void) -> css_error,
    parent_node: @fn(node: *c_void, parent: *mut *c_void) -> css_error,
    node_has_class: @fn(node: *c_void, name: *lwc_string, match_: *mut bool) -> css_error,
    node_has_id: @fn(node: *c_void, name: *lwc_string, match_: *mut bool) -> css_error,
    named_ancestor_node: @fn(node: *c_void,
                             qname: *css_qname,
                             parent: *mut *c_void) -> css_error,
    node_is_root: @fn(node: *c_void, match_: *mut bool) -> css_error,
    node_is_link: @fn(node: *c_void, match_: *mut bool) -> css_error,
    node_is_visited:@fn(node: *c_void, match_: *mut bool) -> css_error,
    ua_default_for_property: @fn(property: uint32_t, hint: *mut css_hint) -> css_error,
}

fn build_css_select_handler() -> css_select_handler {
    css_select_handler {
        handler_version: 1,
        node_name: @node_name,
        node_classes: @node_classes,
        node_id: @node_id,
        named_ancestor_node: @named_ancestor_node,
        named_parent_node: @named_parent_node,
        named_sibling_node: @named_sibling_node,
        named_generic_sibling_node: @named_generic_sibling_node,
        parent_node: @parent_node,
        sibling_node: @sibling_node,
        node_has_name: @node_has_name,
        node_has_class: @node_has_class,
        node_has_id: @node_has_id,
        node_has_attribute: @node_has_attribute,
        node_has_attribute_equal: @node_has_attribute_equal,
        node_has_attribute_dashmatch: @node_has_attribute_dashmatch,
        node_has_attribute_includes: @node_has_attribute_includes,
        node_has_attribute_prefix: @node_has_attribute_prefix,
        node_has_attribute_suffix: @node_has_attribute_suffix,
        node_has_attribute_substring: @node_has_attribute_substring,
        node_is_root: @node_is_root,
        node_count_siblings: @node_count_siblings,
        node_is_empty: @node_is_empty,
        node_is_link: @node_is_link,
        node_is_visited: @node_is_visited,
        node_is_hover: @node_is_hover,
        node_is_active: @node_is_active,
        node_is_focus: @node_is_focus,
        node_is_enabled: @node_is_enabled,
        node_is_disabled: @node_is_disabled,
        node_is_checked: @node_is_checked,
        node_is_target: @node_is_target,
        node_is_lang: @node_is_lang,
        node_presentational_hint: @node_presentational_hint,
        ua_default_for_property: @ua_default_for_property,
        compute_font_size: @compute_font_size
    }
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
fn ph(pw: *c_void) -> @UntypedHandler {
    unsafe { transmute(pw) }
}

fn node_name(pw: *c_void, node: *c_void, qname: *mut css_qname) -> css_error {
    enter("node_name");
    (ph(pw).node_name)(node, qname)
}
fn node_classes(pw: *c_void, node: *c_void, classes: *mut **lwc_string, n_classes: *mut uint32_t) -> css_error {
    enter("node_classes");
    (ph(pw).node_classes)(node, classes, n_classes)
}
fn node_id(pw: *c_void, node: *c_void, id: *mut *lwc_string) -> css_error {
    enter("node_id");
    (ph(pw).node_id)(node, id)
}
fn named_ancestor_node(pw: *c_void,
                                  node: *c_void,
                                  qname: *css_qname,
                                  parent: *mut *c_void) -> css_error {
    enter("named_ancestor_node");
    (ph(pw).named_ancestor_node)(node, qname, parent)
}
fn named_parent_node(pw: *c_void, node: *c_void, qname: *css_qname, parent: *mut *c_void) -> css_error {
    enter("named_parent_node");
    (ph(pw).named_parent_node)(node, qname, parent)
}
fn named_sibling_node(_pw: *c_void, node: *c_void, _qname: *css_qname, sibling: *mut *c_void) -> css_error {
    unimpl_warn("named_sibling_node");
    unsafe {
        *sibling = node;
        CSS_OK
    }
}
fn named_generic_sibling_node(_pw: *c_void, _node: *c_void, _qname: *css_qname, _sibling: **c_void) -> css_error {
    unimpl("named_generic_sibling_node")
}
fn parent_node(pw: *c_void, node: *c_void, parent: *mut *c_void) -> css_error {
    enter("parent_node");
    (ph(pw).parent_node)(node, parent)
}
fn sibling_node(_pw: *c_void, node: *c_void, sibling: *mut *c_void) -> css_error {
    unimpl_warn("sibling_node");
    unsafe {
        *sibling = node;
        CSS_OK
    }
}
fn node_has_name(_pw: *c_void, _node: *c_void, _qname: *css_qname, _match_: *bool) -> css_error {
    unimpl("node_has_name")
}
fn node_has_class(pw: *c_void, node: *c_void, name: *lwc_string, match_: *mut bool) -> css_error {
    enter("node_has_class");
    (ph(pw).node_has_class)(node, name, match_)
}
fn node_has_id(pw: *c_void, node: *c_void, name: *lwc_string, match_: *mut bool) -> css_error {
    enter("node_has_id");
    (ph(pw).node_has_id)(node, name, match_)
}
fn node_has_attribute(_pw: *c_void, _node: *c_void, _qname: *css_qname, match_: *mut bool) -> css_error {
    unimpl_warn("node_has_attribute");
    unsafe {
        *match_ = false;
        CSS_OK
    }
}
fn node_has_attribute_equal(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, match_: *mut bool) -> css_error {
    unimpl_warn("node_has_attribute_equal");
    unsafe {
        *match_ = false;
        CSS_OK
    }
}
fn node_has_attribute_dashmatch(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, _match_: *bool) -> css_error {
    unimpl("node_has_attribute_dashmatch")
}
fn node_has_attribute_includes(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, match_: *mut bool) -> css_error {
    unimpl_warn("node_has_attribute_includes");
    unsafe {
        *match_ = false;
        CSS_OK
    }
}
fn node_has_attribute_prefix(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, match_: *mut bool) -> css_error {
    unimpl_warn("node_has_attribute_prefix");
    unsafe {
        *match_ = false;
        CSS_OK
    }
}
fn node_has_attribute_suffix(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, match_: *mut bool) -> css_error {
    unimpl_warn("node_has_attribute_suffix");
    unsafe {
        *match_ = false;
        CSS_OK
    }
}
fn node_has_attribute_substring(_pw: *c_void, _node: *c_void, _qname: *css_qname, _value: *lwc_string, match_: *mut bool) -> css_error {
    unimpl_warn("node_has_attribute_substring");
    unsafe {
        *match_ = false;
        CSS_OK
    }
}
fn node_is_root(pw: *c_void, node: *c_void, match_: *mut bool) -> css_error {
    enter("node_is_root");
    (ph(pw).node_is_root)(node, match_)
}
fn node_count_siblings(_pw: *c_void, _node: *c_void, _same_name: bool, _after: bool, count: *mut int32_t) -> css_error {
    unimpl_warn("node_count_siblings");
    unsafe {
        *count = 0;
        CSS_OK
    }
}
fn node_is_empty(_pw: *c_void, _node: *c_void, match_: *mut bool) -> css_error {
    unimpl_warn("node_is_empty");
    unsafe {
        *match_ = true;
        CSS_OK
    }
}
fn node_is_link(pw: *c_void, node: *c_void, match_: *mut bool) -> css_error {
    enter("node_is_link");
    (ph(pw).node_is_link)(node, match_)
}
fn node_is_visited(pw: *c_void, node: *c_void, match_: *mut bool) -> css_error {
    enter("node_is_visited");
    (ph(pw).node_is_visited)(node, match_)
}
fn node_is_hover(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
    unimpl_warn("node_is_hover");
    CSS_OK
}
fn node_is_active(_pw: *c_void, _node: *c_void, match_: *mut bool) -> css_error {
    unimpl_warn("node_is_active");
    unsafe {
        *match_ = false;
        CSS_OK
    }
}
fn node_is_focus(_pw: *c_void, _node: *c_void, match_: *mut bool) -> css_error {
    unimpl_warn("node_is_focus");
    unsafe {
        *match_ = false;
        CSS_OK
    }
}
fn node_is_enabled(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
    unimpl("node_is_enabled")
}
fn node_is_disabled(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
    unimpl("node_is_disabled")
}
fn node_is_checked(_pw: *c_void, _node: *c_void, _match_: *bool) -> css_error {
    unimpl("node_is_checked")
}
fn node_is_target(_pw: *c_void, _node: *c_void, match_: *mut bool) -> css_error {
    unimpl_warn("node_is_target");
    unsafe {
        *match_ = false;
        CSS_OK
    }
}
fn node_is_lang(_pw: *c_void, _node: *c_void, _lang: *lwc_string, match_: *mut bool) -> css_error {
    unimpl_warn("node_is_lang");
    unsafe {
        *match_ = false;
        CSS_OK
    }
}
fn node_presentational_hint(_pw: *c_void, _node: *c_void, _property: uint32_t, _hint: *css_hint) -> css_error {
    enter("node_presentational_hint");
    CSS_PROPERTY_NOT_SET
}
fn ua_default_for_property(pw: *c_void, property: uint32_t, hint: *mut css_hint) -> css_error {
    enter("ua_default_for_property");
    (ph(pw).ua_default_for_property)(property, hint)
}
fn compute_font_size(_pw: *c_void, parent: *css_hint, size: *mut css_hint) -> css_error {
    enter("compute_font_size");
    // FIXME: This should be merged with the one in rust-css, I think. --pcwalton
    let parent_hint;
    if parent.is_null() {
        parent_hint = CssHintLength(CSS_UNIT_PX(16 * 1024));
    } else {
        parent_hint = CssHint::new(CSS_PROP_FONT_SIZE, parent);
    }
    parent_hint.write_to_ll(CSS_PROP_FONT_SIZE, size);
    CSS_OK
}




impl SelectCtx {
    pub fn new() -> SelectCtx {
        SelectCtx {
            inner: css_select_ctx::css_select_ctx_create()
        }
    }

    /**
    Add `Stylesheet`s to the selection context, where they will participate in the cascade
    during future selector matching
    */
    pub fn append_sheet(&mut self, sheet: Stylesheet, origin: StylesheetOrigin) {
        let sheet = match sheet {
            Stylesheet { inner: inner } => inner
        };

        self.inner.css_select_ctx_append_sheet(sheet, origin.to_net(), CSS_MEDIA_SCREEN)
    }

    /**
    Select the style for a single node. `handler` is used to query the client for
    a wide range of client-specific details like node relationships, names, and UA
    defaults.
    */
    pub fn select_style<N: VoidPtrLike, H: SelectHandler<N>>(&self,
                                                             node: &N,
                                                             inline_style: Option<&Stylesheet>,
                                                             handler: &H) -> SelectResults {
        let inner_handler = SelectHandlerWrapper {
            inner: handler
        };
        let inner_inline_style = match inline_style {
            None => None,
            Some(ref sheet) => Some(&sheet.inner),
        };
        SelectResults {
            inner: self.inner.select_style::<N, SelectHandlerWrapper<N, H>>(
                node,
                CSS_MEDIA_SCREEN,
                inner_inline_style,
                &inner_handler)
        }
    }
}

/**
Represents the 'style' of a single node, including it's pseudo-elements.
*/
pub struct SelectResults {
    inner: @mut css_select_results
}

impl SelectResults {
    /** Retrieve the computed style of a single pseudo-element */
    pub fn computed_style(&mut self) -> ComputedStyle {
        ComputedStyle {
            inner: self.inner.computed_style(CSS_PSEUDO_ELEMENT_NONE)
        }
    }
}

/**
Callbacks used to query the implementation-specific DOM
*/
pub trait SelectHandler<N> {
    fn with_node_name<R>(&self, node: &N, f: &fn(&str) -> R) -> R;
    fn with_node_classes<R>(&self, node: &N, f: &fn(Option<&str>) -> R) -> R;
    fn with_node_id<R>(&self, node: &N, f: &fn(Option<&str>) -> R) -> R;
    fn named_parent_node(&self, node: &N, name: &str) -> Option<N>;
    fn parent_node(&self, node: &N) -> Option<N>;
    fn node_has_class(&self, node: &N, &str) -> bool;
    fn node_has_id(&self, node: &N, &str) -> bool;
    fn named_ancestor_node(&self, node: &N, name: &str) -> Option<N>;
    fn node_is_root(&self, node: &N) -> bool;
    fn node_is_link(&self, node: &N) -> bool;
}

pub trait CssSelectHandler<N> {
    fn node_name(&self, node: &N) -> css_qname;
    fn node_classes(&self, node: &N) -> Option<~[~str]>;
    fn node_id(&self, node: &N) -> Option<~str>;
    fn named_parent_node(&self, node: &N, qname: &css_qname) -> Option<N>;
    fn parent_node(&self, node: &N) -> Option<N>;
    fn node_has_class(&self, node: &N, name: ~str) -> bool;
    fn node_has_id(&self, node: &N, name: ~str) -> bool;
    fn named_ancestor_node(&self, node: &N, qname: &css_qname) -> Option<N>;
    fn node_is_root(&self, node: &N) -> bool;
    fn node_is_link(&self, node: &N) -> bool;
    fn node_is_visited(&self, node: &N) -> bool;
    fn ua_default_for_property(&self, property: css_properties_e) -> CssHint;
}

/** Used to convert the netsurfcss CssSelectHandler callbacks to out SelectHandler callbacks */
struct SelectHandlerWrapper<N, H> {
    // FIXME: Can't encode region variables
    inner: *H
}

impl< N, H: SelectHandler<N>> SelectHandlerWrapper<N, H> {
    fn inner_ref(&self) -> &H {
        unsafe { &*self.inner }
    }
}

impl<N, H: SelectHandler<N>> CssSelectHandler<N> for SelectHandlerWrapper<N, H> {
    fn node_name(&self, node: &N) -> css_qname {
        do self.inner_ref().with_node_name(node) |name| {
            // TODO use wapcaplet 
            css_qname{name:0, ns:0}
        }
    }

    fn node_classes(&self, node: &N) -> Option<~[~str]> {
        do self.inner_ref().with_node_classes(node) |node_classes_opt| {
           do node_classes_opt.map |s| {
               debug!("SelectHandlerWrapper::node_classes - classes: %?", *s);
               let mut v = ~[];
               for t in s.split_iter(' ') {
                   debug!("SelectHandlerWrapper::node_classes - class: %?", t);
                   if t != "" { v.push(t) }
               }
               debug!("SelectHandlerWrapper::classes: %?", v);
               v
           }
        }
    }

    fn node_id(&self, node: &N) -> Option<~str> {
        do self.inner_ref().with_node_id(node) |node_id_opt| {
            //node_id_opt.map(|s| lwcstr_from_rust_str(*s))
            node_id_opt
        }
    }

    fn named_parent_node(&self, node: &N, qname: &css_qname) -> Option<N> {
        self.inner_ref().named_parent_node(node, qname.name)
    }

    fn parent_node(&self, node: &N) -> Option<N> {
        self.inner_ref().parent_node(node)
    }

    fn node_has_class(&self, node: &N, name: ~str) -> bool {
        self.inner_ref().node_has_class(node, name)
    }

    fn node_has_id(&self, node: &N, name: ~str) -> bool {
        self.inner_ref().node_has_id(node, name)
    }

    fn named_ancestor_node(&self, node: &N, qname: &css_qname) -> Option<N> {
        self.inner_ref().named_ancestor_node(node,qname.name)
    }

    fn node_is_root(&self, node: &N) -> bool {
        self.inner_ref().node_is_root(node)
    }

    fn node_is_link(&self, node: &N) -> bool {
        self.inner_ref().node_is_link(node)
    }

    fn node_is_visited(&self, _node: &N) -> bool {
        // FIXME
        warn_unimpl("node_is_visited");
        false
    }

    fn ua_default_for_property(&self, property: css_properties_e ) -> CssHint {
        warn!("not specifiying ua default for property %?", property);
        CssHintDefault
    }
}

fn warn_unimpl(what: &str) {
    warn!("unimplemented select handler: %?", what);
}
