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

use stylesheet::Stylesheet;
use computed::*;
//use util::VoidPtrLike;
//use wapcaplet::lwc_string;
//use lwcstr_from_rust_str = wapcaplet::from_rust_string;
//use n::u::{rust_str_to_net_qname, net_qname_to_rust_str};
use types::StylesheetOrigin;
use types::*;
//use n;
use srid_css::select::select::*;
use srid_css::select::common::*;
use srid_css::libwapcaplet::wapcaplet::*;
use srid_css::stylesheet::*;
use srid_css::include::types::*;
use srid_css::utils::errors::*;

use std::cast::transmute;
use std::ptr::{null, /*to_mut_unsafe_ptr,*/ to_unsafe_ptr};

use std::libc::c_void;

pub trait VoidPtrLike {
    fn from_void_ptr(ptr: *c_void) -> Self;
    fn to_void_ptr(&self) -> *c_void;
}


pub struct SelectCtx {
    priv select_ctx:~css_select_ctx,
        // Whenever a sheet is added to the select ctx we will take ownership of it
        // to ensure that it stays alive
    priv sheets: ~[css_stylesheet],
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
            select_ctx: css_select_ctx::css_select_ctx_create(),
            sheets: ~[]
        }
    }

    /**
    Add `Stylesheet`s to the selection context, where they will participate in the cascade
    during future selector matching
    */
    pub fn append_sheet(&mut self, sheet: Stylesheet, origin: StylesheetOrigin) {
        self.select_ctx.css_select_ctx_append_sheet(sheet.inner, origin.to_net(), srid_css::include::types::CSS_MEDIA_SCREEN as u64);

    }

    /**
    Select the style for a single node. `handler` is used to query the client for
    a wide range of client-specific details like node relationships, names, and UA
    defaults.
    */
    pub fn select_style<N: VoidPtrLike, H: SelectHandler<N>>(&mut self,
                                                             node: &N,
                                                             inline_style: Option<&Stylesheet>,
                                                             handler: &H) -> SelectResults {
        // let inner_handler = SelectHandlerWrapper {
        //     inner: handler
        // };
        // let inner_inline_style = match inline_style {
        //     None => None,
        //     Some(ref sheet) => Some(&sheet.inner),
        // };
        // SelectResults {
        //     inner: self.inner.select_style::<N, SelectHandlerWrapper<N, H>>(
        //         node,
        //         n::ll::t::CSS_MEDIA_SCREEN,
        //         inner_inline_style,
        //         &inner_handler)
        // }
                let raw_handler = build_raw_handler();
                
                // let inline_sheet = match inline_style {
                //     None => null(),
                //     Some(sheet) => stylesheet_vector_ref.get_ref()[sheet],
                // };
                let (error, results) = self.select_ctx.css_select_style(node.to_void_ptr(),
                                CSS_MEDIA_SCREEN as u64,
                                Some(inline_style.get_ref().inner),
                                raw_handler,
                                unsafe {transmute(to_unsafe_ptr(handler))} );
                                
                if error as uint != CSS_OK as uint {
                    fail!("Error in Select Style")
                }

                SelectResults {
                    inner: results
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

fn build_raw_handler() -> ~css_select_handler {
    ~css_select_handler {
        handler_version: CSS_SELECT_HANDLER_VERSION_1 as uint,
        node_name: raw_handler::node_name,
        node_classes: raw_handler::node_classes,
        node_id: raw_handler::node_id,
        named_ancestor_node: raw_handler::named_ancestor_node,
        named_parent_node: raw_handler::named_parent_node,
        named_sibling_node: raw_handler::named_sibling_node,
        named_generic_sibling_node: raw_handler::named_generic_sibling_node,
        parent_node: raw_handler::parent_node,
        sibling_node: raw_handler::sibling_node,
        node_has_name: raw_handler::node_has_name,
        node_has_class: raw_handler::node_has_class,
        node_has_id: raw_handler::node_has_id,
        node_has_attribute: raw_handler::node_has_attribute,
        node_has_attribute_equal: raw_handler::node_has_attribute_equal,
        node_has_attribute_dashmatch: raw_handler::node_has_attribute_dashmatch,
        node_has_attribute_includes: raw_handler::node_has_attribute_includes,
        node_has_attribute_prefix: raw_handler::node_has_attribute_prefix,
        node_has_attribute_suffix: raw_handler::node_has_attribute_suffix,
        node_has_attribute_substring: raw_handler::node_has_attribute_substring,
        node_is_root: raw_handler::node_is_root,
        node_count_siblings: raw_handler::node_count_siblings,
        node_is_empty: raw_handler::node_is_empty,
        node_is_link: raw_handler::node_is_link,
        node_is_visited: raw_handler::node_is_visited,
        node_is_hover: raw_handler::node_is_hover,
        node_is_active: raw_handler::node_is_active,
        node_is_focus: raw_handler::node_is_focus,
        node_is_enabled: raw_handler::node_is_enabled,
        node_is_disabled: raw_handler::node_is_disabled,
        node_is_checked: raw_handler::node_is_checked,
        node_is_target: raw_handler::node_is_target,
        node_is_lang: raw_handler::node_is_lang,
        node_presentational_hint: raw_handler::node_presentational_hint,
        ua_default_for_property: raw_handler::ua_default_for_property,
        compute_font_size: raw_handler::compute_font_size
    }
}

pub mod types {
    //use wapcaplet::uint;
    use srid_css::stylesheet::css_fixed;
    use srid_css::select::common::css_hint_length;
    //use helpers::ToLl;
    
    pub enum CssLanguageLevel {
        CssLevel1,
        CssLevel2,
        CssLevel21,
        CssLevel3,
        CssLevelDefault, // NB: This is not the same as the ll value
        // NB: Sentinal variant to prevent the naive transmute conversion from working
        CssLevelNotACLikeEnum(uint)
    }

    // NB: This must have the same binary structure as css_color
    pub struct CssColor { b: u8, g: u8, r: u8, a: u8 }

    pub struct CssQName {
        ns: Option<uint>,
        name: uint
    }

    pub enum CssUnit {
        CssUnitPx(css_fixed),
        CssUnitEx(css_fixed),
        CssUnitEm(css_fixed),
        CssUnitIn(css_fixed),
        CssUnitCm(css_fixed),
        CssUnitMm(css_fixed),
        CssUnitPt(css_fixed),
        CssUnitPc(css_fixed),
        CssUnitPct(css_fixed),
        CssUnitDeg(css_fixed),
        CssUnitGrad(css_fixed),
        CssUnitRad(css_fixed),
        CssUnitMs(css_fixed),
        CssUnitS(css_fixed),
        CssUnitHz(css_fixed),
        CssUnitKHz(css_fixed)
    }

    impl CssUnit {
        pub fn to_ll_css_hint_length(&self) -> ~css_hint_length {
            let (unit, value) = self.to_ll();
            ~css_hint_length {
                value: value,
                unit: unit
            }
        }

        pub fn to_css_fixed(&self) -> css_fixed {
            match *self {
                CssUnitPx(css_fixed) |
                CssUnitEx(css_fixed) |
                CssUnitEm(css_fixed) |
                CssUnitIn(css_fixed) |
                CssUnitCm(css_fixed) |
                CssUnitMm(css_fixed) |
                CssUnitPt(css_fixed) |
                CssUnitPc(css_fixed) |
                CssUnitPct(css_fixed) |
                CssUnitDeg(css_fixed) |
                CssUnitGrad(css_fixed) |
                CssUnitRad(css_fixed) |
                CssUnitMs(css_fixed) |
                CssUnitS(css_fixed) |
                CssUnitHz(css_fixed) |
                CssUnitKHz(css_fixed) => css_fixed
            }
        }

        pub fn modify(&self, new_value: css_fixed) -> CssUnit {
            match *self {
                CssUnitPx(_) => CssUnitPx(new_value),
                CssUnitEx(_) => CssUnitEx(new_value),
                CssUnitEm(_) => CssUnitEm(new_value),
                CssUnitIn(_) => CssUnitIn(new_value),
                CssUnitCm(_) => CssUnitCm(new_value),
                CssUnitMm(_) => CssUnitMm(new_value),
                CssUnitPt(_) => CssUnitPt(new_value),
                CssUnitPc(_) => CssUnitPc(new_value),
                CssUnitPct(_) => CssUnitPct(new_value),
                CssUnitDeg(_) => CssUnitDeg(new_value),
                CssUnitGrad(_) => CssUnitGrad(new_value),
                CssUnitRad(_) => CssUnitRad(new_value),
                CssUnitMs(_) => CssUnitMs(new_value),
                CssUnitS(_) => CssUnitS(new_value),
                CssUnitHz(_) => CssUnitHz(new_value),
                CssUnitKHz(_) => CssUnitKHz(new_value),
            }
        }
    }
}

pub trait ToLl<T> {
    fn to_ll(&self) -> T;
}

impl ToLl<(css_unit, css_fixed)> for types::CssUnit {
    fn to_ll(&self) -> (css_unit, css_fixed) {
        use super::types::*;

        match *self {
            types::CssUnitPx(value) => (CSS_UNIT_PX, value),
            types::CssUnitEx(value) => (CSS_UNIT_EX, value),
            types::CssUnitEm(value) => (CSS_UNIT_EM, value),
            types::CssUnitIn(value) => (CSS_UNIT_IN, value),
            types::CssUnitCm(value) => (CSS_UNIT_CM, value),
            types::CssUnitMm(value) => (CSS_UNIT_MM, value),
            types::CssUnitPt(value) => (CSS_UNIT_PT, value),
            types::CssUnitPc(value) => (CSS_UNIT_PC, value),
            types::CssUnitPct(value) => (CSS_UNIT_PCT, value),
            types::CssUnitDeg(value) => (CSS_UNIT_DEG, value),
            types::CssUnitGrad(value) => (CSS_UNIT_GRAD, value),
            types::CssUnitRad(value) => (CSS_UNIT_RAD, value),
            types::CssUnitMs(value) => (CSS_UNIT_MS, value),
            types::CssUnitS(value) => (CSS_UNIT_S, value),
            types::CssUnitHz(value) => (CSS_UNIT_HZ, value),
            types::CssUnitKHz(value) => (CSS_UNIT_KHZ, value)
        }
    }
}

pub fn ll_unit_to_hl_unit(unit: css_unit, value: css_fixed) -> types::CssUnit {
    use super::types::*;
    use srid_css::include::types::*;

    if unit  as uint == CSS_UNIT_PX as uint {
        types::CssUnitPx(value)
    } else if unit as uint == CSS_UNIT_EX as uint {
        types::CssUnitEx(value)
    } else if unit as uint == CSS_UNIT_EM as uint {
        types::CssUnitEm(value)
    } else if unit as uint == CSS_UNIT_IN as uint {
        types::CssUnitIn(value)
    } else if unit as uint == CSS_UNIT_CM as uint {
        types::CssUnitCm(value)
    } else if unit as uint == CSS_UNIT_MM as uint {
        types::CssUnitMm(value)
    } else if  unit as uint == CSS_UNIT_PT  as uint{
        types::CssUnitPt(value)
    } else if unit as uint == CSS_UNIT_PC as uint {
        types::CssUnitPc(value)
    } else if unit as uint == CSS_UNIT_PCT as uint {
        types::CssUnitPct(value)
    } else if unit as uint == CSS_UNIT_DEG as uint {
        types::CssUnitDeg(value)
    } else if unit as uint == CSS_UNIT_GRAD as uint {
        types::CssUnitGrad(value)
    } else if unit as uint == CSS_UNIT_RAD as uint {
        types::CssUnitRad(value)
    } else if unit as uint == CSS_UNIT_MS  as uint{
        types::CssUnitMs(value)
    } else if unit as uint == CSS_UNIT_S as uint {
        types::CssUnitS(value)
    } else if unit  as uint== CSS_UNIT_HZ as uint {
        types::CssUnitHz(value)
    } else if unit as uint == CSS_UNIT_KHZ as uint {
        types::CssUnitKHz(value)
    } else {
        fail!()
    }
}

mod raw_handler {
    //use std::libc::types::common::c99::{uint32_t, int32_t};
    use srid_css::stylesheet::css_qname;
    use std::libc::c_void;
    use std::cast::transmute;
    use srid_css::utils::errors::{css_error, CSS_OK, CSS_PROPERTY_NOT_SET};
    use srid_css::select::common::css_hint;
    use super::types;
    use super::hint;
    // //use helpers::VoidPtrLike;
    use super::properties::*;
    use super::hint::CssHint;
    use super::UntypedHandler;
    use srid_css::libwapcaplet::wapcaplet::lwc;


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
            parent_hint = hint::CssHintLength(types::CssUnitPx(16 * 1024));
        } else {
            parent_hint = CssHint::new(CssPropFontSize, parent);
        }
        parent_hint.write_to_ll(CssPropFontSize, size.unwrap());
        CSS_OK
    }
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

pub mod properties {
    use std::cast::transmute;
    pub enum CssProperty {
        CssPropAzimuth          = 0x000,
        CssPropBackgroundAttachment     = 0x001,
        CssPropBackgroundColor      = 0x002,
        CssPropBackgroundImage      = 0x003,
        CssPropBackgroundPosition       = 0x004,
        CssPropBackgroundRepeat     = 0x005,
        CssPropBorderCollapse       = 0x006,
        CssPropBorderSpacing            = 0x007,
        CssPropBorderTopColor       = 0x008,
        CssPropBorderRightColor     = 0x009,
        CssPropBorderBottomColor        = 0x00a,
        CssPropBorderLeftColor      = 0x00b,
        CssPropBorderTopStyle       = 0x00c,
        CssPropBorderRightStyle     = 0x00d,
        CssPropBorderBottomStyle        = 0x00e,
        CssPropBorderLeftStyle      = 0x00f,
        CssPropBorderTopWidth       = 0x010,
        CssPropBorderRightWidth     = 0x011,
        CssPropBorderBottomWidth        = 0x012,
        CssPropBorderLeftWidth      = 0x013,
        CssPropBottom               = 0x014,
        CssPropCaptionSide          = 0x015,
        CssPropClear                = 0x016,
        CssPropClip             = 0x017,
        CssPropColor                = 0x018,
        CssPropContent          = 0x019,
        CssPropCounterIncrement     = 0x01a,
        CssPropCounterReset         = 0x01b,
        CssPropCueAfter         = 0x01c,
        CssPropCueBefore            = 0x01d,
        CssPropCursor               = 0x01e,
        CssPropDirection            = 0x01f,
        CssPropDisplay          = 0x020,
        CssPropElevation            = 0x021,
        CssPropEmptyCells           = 0x022,
        CssPropFloat                = 0x023,
        CssPropFontFamily           = 0x024,
        CssPropFontSize         = 0x025,
        CssPropFontStyle            = 0x026,
        CssPropFontVariant          = 0x027,
        CssPropFontWeight           = 0x028,
        CssPropHeight               = 0x029,
        CssPropLeft             = 0x02a,
        CssPropLetterSpacing            = 0x02b,
        CssPropLineHeight           = 0x02c,
        CssPropListStyleImage       = 0x02d,
        CssPropListStylePosition        = 0x02e,
        CssPropListStyleType        = 0x02f,
        CssPropMarginTop            = 0x030,
        CssPropMarginRight          = 0x031,
        CssPropMarginBottom         = 0x032,
        CssPropMarginLeft           = 0x033,
        CssPropMaxHeight            = 0x034,
        CssPropMaxWidth         = 0x035,
        CssPropMinHeight            = 0x036,
        CssPropMinWidth         = 0x037,
        CssPropOrphans,
        CssPropOutlineColor         = 0x039,
        CssPropOutlineStyle         = 0x03a,
        CssPropOutlineWidth         = 0x03b,
        CssPropOverflow         = 0x03c,
        CssPropPaddingTop           = 0x03d,
        CssPropPaddingRight         = 0x03e,
        CssPropPaddingBottom            = 0x03f,
        CssPropPaddingLeft          = 0x040,
        CssPropPageBreakAfter       = 0x041,
        CssPropPageBreakBefore      = 0x042,
        CssPropPageBreakInside      = 0x043,
        CssPropPauseAfter           = 0x044,
        CssPropPauseBefore          = 0x045,
        CssPropPitchRange           = 0x046,
        CssPropPitch                = 0x047,
        CssPropPlayDuring           = 0x048,
        CssPropPosition         = 0x049,
        CssPropQuotes               = 0x04a,
        CssPropRichness         = 0x04b,
        CssPropRight                = 0x04c,
        CssPropSpeakHeader          = 0x04d,
        CssPropSpeakNumeral         = 0x04e,
        CssPropSpeakPunctuation     = 0x04f,
        CssPropSpeak                = 0x050,
        CssPropSpeechRate           = 0x051,
        CssPropStress               = 0x052,
        CssPropTableLayout          = 0x053,
        CssPropTextAlign            = 0x054,
        CssPropTextDecoration       = 0x055,
        CssPropTextIndent           = 0x056,
        CssPropTextTransform            = 0x057,
        CssPropTop              = 0x058,
        CssPropUnicodeBidi          = 0x059,
        CssPropVerticalAlign            = 0x05a,
        CssPropVisibility           = 0x05b,
        CssPropVoiceFamily          = 0x05c,
        CssPropVolume               = 0x05d,
        CssPropWhiteSpace           = 0x05e,
        CssPropWidows               = 0x05f,
        CssPropWidth                = 0x060,
        CssPropWordSpacing          = 0x061,
        CssPropZIndex           = 0x062,
        CssPropOpacity          = 0x063,
        CssPropBreakAfter           = 0x064,
        CssPropBreakBefore          = 0x065,
        CssPropBreakInside          = 0x066,
        CssPropColumnCount          = 0x067,
        CssPropColumnFill           = 0x068,
        CssPropColumnGap            = 0x069,
        CssPropColumnRuleColor      = 0x06a,
        CssPropColumnRuleStyle      = 0x06b,
        CssPropColumnRuleWidth      = 0x06c,
        CssPropColumnSpan           = 0x06d,
        CssPropClomumnWidth         = 0x06e,
    }

     pub fn property_from_uint(property: u32) -> CssProperty {
        unsafe { transmute(property as uint) }
    }

    pub enum CssFontStyle {
        CssFontStyleInherit         = 0x0,
        CssFontStyleNormal          = 0x1,
        CssFontStyleItalic          = 0x2,
        CssFontStyleOblique         = 0x3
    }

    pub enum CssFontFamily {
        CssFontFamilyInherit            = 0x0,
        /* Named fonts exist if pointer is non-NULL */
        CssFontFamilySerif          = 0x1,
        CssFontFamilySansSerif      = 0x2,
        CssFontFamilyCursive            = 0x3,
        CssFontFamilyFantasy            = 0x4,
        CssFontFamilyMonospace      = 0x5
    }

    pub enum CssFontVariant {
        CssFontVariantInherit = 0,
        CssFontVariantNormal = 1,
        CssFontVariantSmallCaps = 2,
    }

    pub enum CssFontWeight {
        CssFontWeightInherit            = 0x0,
        CssFontWeightNormal         = 0x1,
        CssFontWeightBold           = 0x2,
        CssFontWeightBolder         = 0x3,
        CssFontWeightLighter            = 0x4,
        CssFontWeight100            = 0x5,
        CssFontWeight200            = 0x6,
        CssFontWeight300            = 0x7,
        CssFontWeight400            = 0x8,
        CssFontWeight500            = 0x9,
        CssFontWeight600            = 0xa,
        CssFontWeight700            = 0xb,
        CssFontWeight800            = 0xc,
        CssFontWeight900            = 0xd
    }

    // NB: This is not identical to css_quotes_e
    pub enum CssQuotes {
    CssQuotesInherit,
        CssQuotesString,
        CssQuotesNone,
        // Sentinal value to give this enum a non-word size, so the
        // naive unsafe conversion to ll fails
        CssQuotesNotACLikeEnum(uint)
    }

}

pub mod hint {


    use super::properties::{CssProperty,
                     CssPropFontSize,
                     CssPropFontFamily,
                     CssPropQuotes,
                     CssPropColor,
                     CssFontFamily};
    use super::types::*;                 
    // use ll::properties::*;
    // use ll::errors::*;
    // use ll::types::css_color;
    //use std::ptr::null;
    //use std::libc::types::common::c99::uint8_t;
    use std::cast::transmute;
    //use std::ptr;
    use srid_css::select::common::*;
    use srid_css::include::properties::*;
    use super::*;
    use srid_css::utils::errors::*;

    // An interpretation of the delightful css_hint union
    pub enum CssHint {
        CssHintFontFamily(~[uint], CssFontFamily),
        CssHintLength(CssUnit),
        CssHintDefault,
        CssHintUnknown
    }

    impl CssHint {

        pub fn new(property: CssProperty, mut hint: Option<&mut ~css_hint>) -> CssHint {
            let status = hint.get_ref().status as u32;
            match property {
                CssPropFontSize => {
                    if status == CSS_FONT_SIZE_DIMENSION as u32 {
                        let length: &mut ~css_hint_length = hint.get_mut_ref().length.get_mut_ref();
                        CssHintLength(ll_unit_to_hl_unit( length.unit,  length.value))
                    } else {
                        CssHintUnknown
                    }
                }
                _ => fail!(fmt!("unknown css hint: %?", property))
            }
        }
        
        pub fn write_to_ll(&self, property: CssProperty, llhint: &mut ~css_hint) -> css_error {
            match (property, self) {
                (CssPropFontFamily, &CssHintDefault) => {
                    llhint.strings = None;
                    // *strings = null();
                    llhint.status = CSS_FONT_FAMILY_SANS_SERIF as u8;
                }
                (CssPropQuotes, &CssHintDefault) => {
                    llhint.strings = None;
                    llhint.status = CSS_QUOTES_STRING_OR_NONE as u8;
                }
                (CssPropColor, &CssHintDefault) => {
                    // let color: &mut css_color = hint_data_field(llhint);
                    llhint.color = unsafe {transmute(CssColor { a: 255, r: 0, g: 0, b: 0 })};
                    llhint.status = CSS_COLOR_COLOR as u8;
                }
                (CssPropFontSize, &CssHintLength(val)) => {
                    llhint.length = Some(val.to_ll_css_hint_length());
                    llhint.status = CSS_FONT_SIZE_DIMENSION as u8;
                }
                (_, &CssHintUnknown) => {
                    fail!(fmt!("unknown css hint %?", property));
                }
                (_, _) => {
                    fail!(fmt!("incorrectly handled property hint: %?, %?", property, self));
                }
            }

            return CSS_OK;
        }
    }

}