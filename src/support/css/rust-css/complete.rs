/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern mod srid_css;

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
//use n;

pub struct CompleteSelectResults {
    inner: SelectResults
}

pub fn css_fixed_to_float(f: css_fixed) -> float {
    static BEFORE: i32 = 10;
    f as float * 1.0f / ((1i32 << BEFORE) as float)
}

pub fn float_to_css_fixed(f: float) -> css_fixed {
    static BEFORE: i32 = 10;
    (f * ((1 << BEFORE) as float)) as css_fixed
}

// pub type ComputeFontSizeCb = @fn(parent: &Option<CssHint>, child: &CssHint) -> CssHint;

pub fn compose(parent: &CssComputedStyle, child: &mut CssComputedStyle,
               compute_font_size: css_fnptr_compute_font_size,
               result: &mut CssComputedStyle) {
    let llparent = parent.computed_style;
    let llchild = child.computed_style;
    let pw = unsafe { transmute(&compute_font_size) };
    let llresult = result.computed_style as *mut css_computed_style;
    // let err = unsafe { css_computed_style_compose(llparent, llchild, compute_font_size_cb, pw, llresult) };
    // if err != CSS_OK {
    //     fail!(~"stylesheet composition failed")
    // }
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
            // FIXME: Need to get real font sizes
            let cb: css_fnptr_compute_font_size =
                |parent: &Option<CssHint>, child: &CssHint| -> CssHint {
                match *child {
                    // Handle relative units
                    CssHintLength(CSS_UNIT_EM(child_em)) => {
                        match *parent {
                            Some(CssHintLength(parent_unit)) => {
                                // CSS3 Values 5.1.1: Multiply parent unit by child unit.
                                let mut new_value =
                                    css_fixed_to_float(parent_unit.to_css_fixed());
                                new_value *= css_fixed_to_float(child_em);
                                let unit = parent_unit.modify(float_to_css_fixed(
                                    new_value));
                                CssHintLength(unit)
                            }
                            _ => CssHintLength(CSS_UNIT_PX(float_to_css_fixed(16.0))),
                        }
                    }
                    CssHintLength(CSS_UNIT_PT(child_pct)) => {
                        match *parent {
                            Some(CssHintLength(parent_unit)) => {
                                // CSS3 Values 5.1.1: Multiply parent unit by child unit.
                                let mut new_value =
                                    css_fixed_to_float(parent_unit.to_css_fixed());
                                new_value *= css_fixed_to_float(child_pct) / 100.0;
                                let unit = parent_unit.modify(float_to_css_fixed(
                                    new_value));
                                CssHintLength(unit)
                            }
                            _ => CssHintLength(CSS_UNIT_PX(float_to_css_fixed(16.0))),
                        }
                    }
                    // Pass through absolute units
                    CssHintLength(unit) => CssHintLength(unit),
                    _ => {
                        CssHintLength(CSS_UNIT_PX(float_to_css_fixed(16.0)))
                    }
                }
            };
            // XXX: Need an aliasable &mut here
            let net_result_computed: &mut CssComputedStyle = unsafe { cast::transmute(net_child_computed) };
            let net_child_computed: &mut CssComputedStyle = unsafe { cast::transmute(&child_computed.inner) };
            let net_parent_computed = &parent_computed.inner.inner;
            compose(net_parent_computed, net_child_computed, cb, net_result_computed);
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

