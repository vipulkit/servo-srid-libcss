/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
extern mod srid_css;

use std::libc::*;
use std::cast;
use std::cast::transmute;
use color::Color;
use select::SelectResults;
use computed::ComputedStyle;
//use n::h::CssHintLength;
//use n::u::float_to_css_fixed;
use values::*;
use types::*;
use srid_css::select::common::*;
use srid_css::select::dispatch::*;
use srid_css::utils::errors::*;
use srid_css::include::types::*;
use srid_css::include::fpmath::*;
use srid_css::include::properties::*;
use helpers::hint::*;
use helpers::properties::*;
use helpers::computed::CssComputedStyle;


pub type ComputeFontSizeCb = @fn(parent: &Option<css_hint>, child: &css_hint) -> css_hint;

pub struct CompleteSelectResults {
    inner: SelectResults
}

// pub mod properties {

//     use std::libc::types::common::c99::uint32_t;
//     use std::cast::transmute;

    
// }
// Merge parent and child styles into another style. The result
// pointer may point to the child style, in which case the child
// style is overwritten
#[fixed_stack_segment]
pub fn compose(parent: &CssComputedStyle, child: &mut CssComputedStyle,
                    result: &mut CssComputedStyle) {
    let llparent = &parent.computed_style;
    let llchild = &child.computed_style;
    // let pw = unsafe { transmute(&compute_font_size) };
    let llresult = &mut result.computed_style;
    let err = css_computed_style_compose(llparent, llchild, compute_font_size_cb, llresult);
    if err as uint != CSS_OK as uint {
        fail!(~"stylesheet composition failed")
    }
}

fn compute_font_size_cb(parent: Option<&mut ~css_hint>, size: Option<&mut ~css_hint>) -> css_error {
    //let hlcbptr: *ComputeFontSizeCb = unsafe { transmute(pw) };
    /*let cb: ComputeFontSizeCb =
                |parent: Option<css_hint>, child: &css_hint| -> css_hint {
                if child.length.is_some() {
                    // Handle relative units
                    match child.length.get_ref().unit {
                        CSS_UNIT_EM | CSS_UNIT_PCT=> {
                            if parent.is_some() {
                                if parent.get_ref().length.is_some() {
                                    let mut new_value = css_fixed_to_float(parent.length.get_ref().value.to_css_fixed());
                                    if child.length.get_ref().unit as uint == CSS_UNIT_EM as uint {
                                        new_value *= css_fixed_to_float(child.length.get_ref().value);    
                                    }
                                    else {
                                        new_value *= css_fixed_to_float(child.length.get_ref().value)/100.0;
                                    }
                                    
                                    parent.get_mut_ref().length.get_mut_ref().value = new_value;
                                    parent.get_ref().clone()    
                                }
                            }
                                // CSS3 Values 5.1.1: Multiply parent unit by child unit.
                            else {
                                    css_hint {
                                        hint_type: HINT_LENGTH,
                                        status: 0,
                                        clip: None,
                                        content: None,
                                        counters: None,
                                        length:Some(css_hint_length { value:FLTTOFIX(16.0), unit:CSS_UNIT_PX}),
                                        position: None,
                                        color: 0,
                                        fixed: 0,
                                        integer: 0,
                                        string: None,
                                        strings: None
                                    }
                            }
                        },    
                        unit => {
                            css_hint {
                                hint_type: HINT_LENGTH,
                                status: 0,
                                clip: None,
                                content: None,
                                counters: None,
                                length:Some(css_hint_length { value:child.length.get_ref().value, unit:unit}),
                                position: None,
                                color: 0,
                                fixed: 0,
                                integer: 0,
                                string: None,
                                strings: None
                            }
                        }
                    }
                }
                    // Pass through absolute units
                else { 
                    css_hint {
                        hint_type: HINT_LENGTH,
                        status: 0,
                        clip: None,
                        content: None,
                        counters: None,
                        length:Some(css_hint_length { value:FLTTOFIX(16.0), unit:CSS_UNIT_PX}),
                        position: None,
                        color: 0,
                        fixed: 0,
                        integer: 0,
                        string: None,
                        strings: None

                    }    
                }
            };
    
    let hlparent = if parent.is_null() {
        None
    } else {
        Some(CssHint::new(CssPropFontSize, parent))
    };
    
    let hlchild = CssHint::new(CssPropFontSize, unsafe { transmute(size) });
    let new_hint = cb(&hlparent, &hlchild);
    new_hint.write_to_ll(CssPropFontSize, size);
    */
    CSS_OK
}

impl<'self> CompleteSelectResults {
    pub fn new_root(root: SelectResults) -> CompleteSelectResults {
        CompleteSelectResults {
            inner: root
        }
    }

    pub fn new_from_parent(parent: &CompleteSelectResults,
                           child: SelectResults) -> CompleteSelectResults {
        // New lifetime
        {
            let parent_computed = parent.computed_style();
            let child_computed = child.computed_style();
            //let net_parent_computed = &parent_computed.inner.inner;
            let net_child_computed = &/*mut*/ child_computed.inner;
            
            // XXX: Need an aliasable &mut here
            let net_result_computed: &mut CssComputedStyle = unsafe { cast::transmute(net_child_computed) };
            let net_child_computed: &mut CssComputedStyle = unsafe { cast::transmute(&child_computed.inner) };
            let net_parent_computed = &parent_computed.inner.inner;
            compose(net_parent_computed, net_child_computed, net_result_computed);
        }

        CompleteSelectResults {
            inner: child
        }
    }

    pub fn computed_style(&'self self) -> CompleteStyle<'self> {
        CompleteStyle {
            inner: self.inner.computed_style()
        }
    }
}

//#[deriving(DeepClone)]
pub struct CompleteStyle<'self> {
    inner: ComputedStyle<'self>
}

impl<'self> CompleteStyle<'self> {

    // CSS 2.1, Section 8 - Box model

    pub fn margin_top(&self) -> CSSMargin {
        strip(self.inner.margin_top())
    }

    pub fn margin_right(&self) -> CSSMargin {
        strip(self.inner.margin_right())
    }

    pub fn margin_bottom(&self) -> CSSMargin {
        strip(self.inner.margin_bottom())
    }

    pub fn margin_left(&self) -> CSSMargin {
        strip(self.inner.margin_left())
    }

    pub fn padding_top(&self) -> CSSPadding {
        strip(self.inner.padding_top())
    }

    pub fn padding_right(&self) -> CSSPadding {
        strip(self.inner.padding_right())
    }

    pub fn padding_bottom(&self) -> CSSPadding {
        strip(self.inner.padding_bottom())
    }

    pub fn padding_left(&self) -> CSSPadding {
        strip(self.inner.padding_left())
    }

    pub fn border_top_style(&self) -> CSSBorderStyle {
        strip(self.inner.border_top_style())
    }

    pub fn border_right_style(&self) -> CSSBorderStyle {
        strip(self.inner.border_right_style())
    }
    pub fn border_bottom_style(&self) -> CSSBorderStyle {
        strip(self.inner.border_bottom_style())
    }

    pub fn border_left_style(&self) -> CSSBorderStyle {
        strip(self.inner.border_left_style())
    }

    pub fn border_top_width(&self) -> CSSBorderWidth {
        strip(self.inner.border_top_width())
    }

    pub fn border_right_width(&self) -> CSSBorderWidth {
        strip(self.inner.border_right_width())
    }

    pub fn border_bottom_width(&self) -> CSSBorderWidth {
        strip(self.inner.border_bottom_width())
    }

    pub fn border_left_width(&self) -> CSSBorderWidth {
        strip(self.inner.border_left_width())
    }

    pub fn border_top_color(&self) -> Color {
        strip(self.inner.border_top_color())
    }

    pub fn border_right_color(&self) -> Color {
        strip(self.inner.border_right_color())
    }

    pub fn border_bottom_color(&self) -> Color {
        strip(self.inner.border_bottom_color())
    }

    pub fn border_left_color(&self) -> Color {
        strip(self.inner.border_left_color())
    }

    // CSS 2.1, Section 9 - Visual formatting model

    pub fn display(&self, root: bool) -> CSSDisplay {
        strip(self.inner.display(root))
    }

    pub fn position(&self) -> CSSPosition {
        strip(self.inner.position())
    }

    pub fn float(&self) -> CSSFloat {
        strip(self.inner.float())
    }

    pub fn clear(&self) -> CSSClear {
        strip(self.inner.clear())
    }

    // CSS 2.1, Section 10 - Visual formatting model details

    pub fn width(&self) -> CSSWidth {
        strip(self.inner.width())
    }

    pub fn height(&self) -> CSSHeight {
        strip(self.inner.height())
    }

    pub fn line_height(&self) -> CSSLineHeight {
        strip(self.inner.line_height())
    }

    pub fn vertical_align(&self) -> CSSVerticalAlign {
        strip(self.inner.vertical_align())
    }

    // CSS 2.1, Section 11 - Visual effects

    // CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

    // CSS 2.1, Section 13 - Paged media

    // CSS 2.1, Section 14 - Colors and Backgrounds

    pub fn background_color(&self) -> Color {
        strip(self.inner.background_color())
    }

    pub fn color(&self) -> Color {
        strip(self.inner.color())
    }

    // CSS 2.1, Section 15 - Fonts

    pub fn font_family(&self) -> ~[CSSFontFamily] {
        strip(self.inner.font_family())
    }

    pub fn font_style(&self) -> CSSFontStyle {
        strip(self.inner.font_style())
    }

    pub fn font_weight(&self) -> CSSFontWeight {
        strip(self.inner.font_weight())
    }

    pub fn font_size(&self) -> CSSFontSize {
        strip(self.inner.font_size())
    }

    pub fn text_decoration(&self) -> CSSTextDecoration{
        strip(self.inner.text_decoration())
    }

    // CSS 2.1, Section 16 - Text

    pub fn text_align(&self) -> CSSTextAlign {
        strip(self.inner.text_align())
    }

    // CSS 2.1, Section 17 - Tables

    // CSS 2.1, Section 18 - User interface

}

fn strip<T>(value: CSSValue<T>) -> T {
    match value {
        Inherit => fail!(~"unexpected 'inherit' value in complete style"),
        Specified(v) => v
    }
}

