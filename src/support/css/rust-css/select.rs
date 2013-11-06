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

use std::libc::*;
use stylesheet::Stylesheet;
use computed::ComputedStyle;
use helpers::VoidPtrLike;
use types::StylesheetOrigin;

use srid_css::include::types::CSS_MEDIA_SCREEN;
use helpers::select::*;
use helpers::properties::*;
use helpers::hint::*;
use helpers::types::CssQName;
use srid_css::select::common::*;
use srid_css::libwapcaplet::wapcaplet::*;
// use srid_css::stylesheet::css_qname;

//use srid_css::select::select::css_select_ctx;

pub struct SelectCtx {
    inner: CssSelectCtx
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
        
        self.inner.append_sheet(sheet, origin.to_net(), CSS_MEDIA_SCREEN as u64)
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
    inner: CssSelectResults
}

impl<'self> SelectResults {
    /** Retrieve the computed style of a single pseudo-element */
    pub fn computed_style(&'self self) -> ComputedStyle<'self> {
        ComputedStyle {
            inner: self.inner.computed_style(CssPseudoElementNone)
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
    fn named_parent_node(&self, node: &N, name:&str) -> Option<N>;
    fn parent_node(&self, node: &N) -> Option<N>;
    fn node_has_class(&self, node: &N, &str) -> bool;
    fn node_has_id(&self, node: &N, &str) -> bool;
    fn named_ancestor_node(&self, node: &N, name: &str) -> Option<N>;
    fn node_is_root(&self, node: &N) -> bool;
    fn node_is_link(&self, node: &N) -> bool;
}

/** Used to convert the netsurfcss CssSelectHandler callbacks to out SelectHandler callbacks */
struct SelectHandlerWrapper<N, H> {
    // FIXME: Can't encode region variables
    inner: *H
}

impl<'self, N, H: SelectHandler<N>> SelectHandlerWrapper<N, H> {
    fn inner_ref(&self) -> &'self H {
        unsafe { &*self.inner }
    }
}

impl<N, H: SelectHandler<N>> CssSelectHandler<N> for SelectHandlerWrapper<N, H> {
    fn node_name(&self, node: &N) -> CssQName {
        do self.inner_ref().with_node_name(node) |name| {
            // TODO use wapcaplet 
            CssQName{ns:None, name:~""}
        }
    }

    fn node_classes(&self, node: &N) -> Option<~[uint]> {
        do self.inner_ref().with_node_classes(node) |node_classes_opt| {
           do node_classes_opt.map |s| {
               debug!("SelectHandlerWrapper::node_classes - classes: %?", *s);
               let mut v = ~[];
               lwc();
               for t in s.split_iter(' ') {
                   debug!("SelectHandlerWrapper::node_classes - class: %?", t);
                   if t != "" { v.push(unsafe{ lwc_ref.get_mut_ref().lwc_intern_string(t)}) }
               }
               debug!("SelectHandlerWrapper::classes: %?", v);
               v
           }
        }
    }
    // TODO if expecting ~str then convert uint to corresponding lwc_string
    fn node_id(&self, node: &N) -> Option<uint> {
        lwc();
        do self.inner_ref().with_node_id(node) |node_id_opt| {
            node_id_opt.map(|s| unsafe{ lwc_ref.get_mut_ref()}.lwc_intern_string(*s))
        }
    }

    fn named_parent_node(&self, node: &N, qname: &mut CssQName) -> Option<N> {
        self.inner_ref().named_parent_node(node, qname.name)
    }

    fn parent_node(&self, node: &N) -> Option<N> {
        self.inner_ref().parent_node(node)
    }

    fn node_has_class(&self, node: &N, name: &str) -> bool {
        self.inner_ref().node_has_class(node, name)
    }

    fn node_has_id(&self, node: &N, name: &str) -> bool {
        self.inner_ref().node_has_id(node, name)
    }

    fn named_ancestor_node(&self, node: &N, qname: &mut CssQName) -> Option<N> {
        self.inner_ref().named_ancestor_node(node, qname.name)
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

    fn ua_default_for_property(&self, property: CssProperty) -> CssHint {
        warn!("not specifiying ua default for property %?", property);
        CssHintDefault
    }
}

fn warn_unimpl(what: &str) {
    warn!("unimplemented select handler: %?", what);
}
