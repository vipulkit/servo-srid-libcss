/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern mod srid_css;

//use std::cast;
use color::Color;
//use select::SelectResults;
use computed::*;
//use n::h::CssHintLength;
//use n::u::float_to_css_fixed;
use values::*;
//use n;
use srid_css::select::dispatch::*;
use srid_css::utils::errors::*;
use srid_css::include::types::*;
use srid_css::include::properties::*;
use srid_css::select::common::*;
//use srid_css::libwapcaplet::wapcaplet::*;

#[deriving(DeepClone)]
pub struct CompleteSelectResults {
    inner: SelectResults
}

impl CompleteSelectResults {
    pub fn new_root(root: SelectResults) -> CompleteSelectResults {
        CompleteSelectResults {
            inner: root
        }
    }

    pub fn new_from_parent(parent: &CompleteSelectResults,
                            child: SelectResults) -> CompleteSelectResults {
        // New lifetime
        {
            let mut parent_computed = parent.computed_style();
            let mut child_computed = child.computed_style();
            //let net_parent_computed = &parent_computed.inner.inner;
            //let net_child_computed = &/*mut*/ child_computed.inner;
            // FIXME: Need to get real font sizes
            let cb: css_fnptr_compute_font_size =
                |mut parent: Option<&mut ~css_hint>, mut child: Option<&mut ~css_hint>| -> css_error {

                let mut temporary_result = ~css_hint_length{value:0 , unit:CSS_UNIT_PX} ;

                if child.is_some() {
                    let child_hint = child.get_mut_ref();   
                    match child_hint.hint_type {
                //         // Handle relative units
                        HINT_LENGTH=>{
                            if child_hint.length.is_some() {
                                let child_hint_length = child_hint.length.get_mut_ref(); 
                                match child_hint_length.unit {
                                    CSS_UNIT_EM => {
                                        if parent.is_some() {
                                            let parent_hint = parent.get_mut_ref(); 
                                            match parent_hint.hint_type {

                                                HINT_LENGTH => {
                                                        if parent_hint.length.is_some() {
                                                            let parent_hint_length = parent_hint.length.get_mut_ref(); 
                                                            // CSS3 Values 5.1.1: Multiply parent unit by child unit.
                                                            let mut new_value =
                                                                css_fixed_to_float(parent_hint_length.value);
                                                            new_value *= css_fixed_to_float(child_hint_length.value);
                                                            // let unit = parent_unit.modify(float_to_css_fixed(new_value));
                                                            // CssHintLength(unit)
                                                            temporary_result = ~css_hint_length{
                                                                    value:float_to_css_fixed(new_value) , 
                                                                    unit:parent_hint_length.unit
                                                            };
                                                        }
                                                }
                                                _ => {
                                                        // n::h::CssHintLength(n::t::CssUnitPx(float_to_css_fixed(16.0))),
                                                        temporary_result = ~css_hint_length{
                                                            value:float_to_css_fixed(16.0) , 
                                                            unit:CSS_UNIT_PX
                                                        }
                                                }
                                            }
                                        }
                                    },
                                    CSS_UNIT_PCT => {
                                        if parent.is_some() {
                                            let parent_hint = parent.get_mut_ref(); 
                                            match parent_hint.hint_type {
                                                //Some(CssHintLength(parent_unit)) => {
                                                HINT_LENGTH => {
                                                    if parent_hint.length.is_some() {
                                                        let parent_hint_length = parent_hint.length.get_mut_ref(); 
                                                        // CSS3 Values 5.1.1: Multiply parent unit by child unit.
                                                        let mut new_value =
                                                            css_fixed_to_float(parent_hint_length.value);
                                                        new_value *= css_fixed_to_float(child_hint_length.value)/100.0;
                                                        // let unit = parent_unit.modify(float_to_css_fixed(new_value));
                                                        // CssHintLength(unit)
                                                        temporary_result = ~css_hint_length{
                                                                value:float_to_css_fixed(new_value), 
                                                                unit:parent_hint_length.unit
                                                        };
                                                    }
                                                }
                                                _ => {
                                                    // n::h::CssHintLength(n::t::CssUnitPx(float_to_css_fixed(16.0))),
                                                    temporary_result = ~css_hint_length{
                                                        value:float_to_css_fixed(16.0) , 
                                                        unit:CSS_UNIT_PX
                                                    }; 
                                                }
                                            }
                                        }
                                    },
                                    // Pass through absolute units
                                   _ =>{
                                        // CssHintLength(unit)
                                        temporary_result = ~css_hint_length{
                                                value:child_hint_length.value , 
                                                unit:child_hint_length.unit
                                        }; 
                                    }
                                }
                            }
                        },
                        _ => {
                            // n::h::CssHintLength(n::t::CssUnitPx(float_to_css_fixed(16.0))) 
                            temporary_result = ~css_hint_length{
                                    value:float_to_css_fixed(16.0) , 
                                    unit:CSS_UNIT_PX
                            }; 
                        }
                    }
                }
                //
                child.get_mut_ref().status = CSS_FONT_SIZE_DIMENSION as u8 ;
                child.get_mut_ref().length = Some(temporary_result) ;
                child.get_mut_ref().hint_type = HINT_LENGTH ;
                CSS_OK
            };
            // XXX: Need an aliasable &mut here
            let net_result_computed  = &mut css_computed_style_create() ;
            let net_child_computed  = &mut child_computed.computed_style;         
            let net_parent_computed = &mut parent_computed.inner.computed_style;
            //n::c::compose(net_parent_computed, net_child_computed, cb, net_result_computed);
            css_computed_style_compose(net_parent_computed,net_child_computed,cb,net_result_computed);
        }

        CompleteSelectResults {
            inner: child
        }
    }

    pub fn computed_style(&self) -> CompleteStyle {
        CompleteStyle {
            inner: self.inner.computed_style()
        }
    }
}

#[deriving(DeepClone)]
pub struct CompleteStyle {
    inner: ComputedStyle
}

impl CompleteStyle {

    // CSS 2.1, Section 8 - Box model

    pub fn margin_top(&mut self) -> CSSMargin {
        strip(self.inner.margin_top())
    }

    pub fn margin_right(&mut self) -> CSSMargin {
        strip(self.inner.margin_right())
    }

    pub fn margin_bottom(&mut self) -> CSSMargin {
        strip(self.inner.margin_bottom())
    }

    pub fn margin_left(&mut self) -> CSSMargin {
        strip(self.inner.margin_left())
    }

    pub fn padding_top(&mut self) -> CSSPadding {
        strip(self.inner.padding_top())
    }

    pub fn padding_right(&mut self) -> CSSPadding {
        strip(self.inner.padding_right())
    }

    pub fn padding_bottom(&mut self) -> CSSPadding {
        strip(self.inner.padding_bottom())
    }

    pub fn padding_left(&mut self) -> CSSPadding {
        strip(self.inner.padding_left())
    }

    pub fn border_top_style(&mut self) -> CSSBorderStyle {
        strip(self.inner.border_top_style())
    }

    pub fn border_right_style(&mut self) -> CSSBorderStyle {
        strip(self.inner.border_right_style())
    }
    pub fn border_bottom_style(&mut self) -> CSSBorderStyle {
        strip(self.inner.border_bottom_style())
    }

    pub fn border_left_style(&mut self) -> CSSBorderStyle {
        strip(self.inner.border_left_style())
    }

    pub fn border_top_width(&mut self) -> CSSBorderWidth {
        strip(self.inner.border_top_width())
    }

    pub fn border_right_width(&mut self) -> CSSBorderWidth {
        strip(self.inner.border_right_width())
    }

    pub fn border_bottom_width(&mut self) -> CSSBorderWidth {
        strip(self.inner.border_bottom_width())
    }

    pub fn border_left_width(&mut self) -> CSSBorderWidth {
        strip(self.inner.border_left_width())
    }

    pub fn border_top_color(&mut self) -> Color {
        strip(self.inner.border_top_color())
    }

    pub fn border_right_color(&mut self) -> Color {
        strip(self.inner.border_right_color())
    }

    pub fn border_bottom_color(&mut self) -> Color {
        strip(self.inner.border_bottom_color())
    }

    pub fn border_left_color(&mut self) -> Color {
        strip(self.inner.border_left_color())
    }

    // CSS 2.1, Section 9 - Visual formatting model

    pub fn display(&mut self, root: bool) -> CSSDisplay {
        strip(self.inner.display(root))
    }

    pub fn position(&mut self) -> CSSPosition {
        strip(self.inner.position())
    }

    pub fn float(&mut self) -> CSSFloat {
        strip(self.inner.float())
    }

    pub fn clear(&mut self) -> CSSClear {
        strip(self.inner.clear())
    }

    // CSS 2.1, Section 10 - Visual formatting model details

    pub fn width(&mut self) -> CSSWidth {
        strip(self.inner.width())
    }

    pub fn height(&mut self) -> CSSHeight {
        strip(self.inner.height())
    }

    pub fn line_height(&mut self) -> CSSLineHeight {
        strip(self.inner.line_height())
    }

    pub fn vertical_align(&mut self) -> CSSVerticalAlign {
        strip(self.inner.vertical_align())
    }

    // CSS 2.1, Section 11 - Visual effects

    // CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

    // CSS 2.1, Section 13 - Paged media

    // CSS 2.1, Section 14 - Colors and Backgrounds

    pub fn background_color(&mut self) -> Color {
        strip(self.inner.background_color())
    }

    pub fn color(&mut self) -> Color {
        strip(self.inner.color())
    }

    // CSS 2.1, Section 15 - Fonts

    pub fn font_family(&mut self) -> ~[CSSFontFamily] {
        strip(self.inner.font_family())
    }

    pub fn font_style(&mut self) -> CSSFontStyle {
        strip(self.inner.font_style())
    }

    pub fn font_weight(&mut self) -> CSSFontWeight {
        strip(self.inner.font_weight())
    }

    pub fn font_size(&mut self) -> CSSFontSize {
        strip(self.inner.font_size())
    }

    pub fn text_decoration(&mut self) -> CSSTextDecoration{
        strip(self.inner.text_decoration())
    }

    // CSS 2.1, Section 16 - Text

    pub fn text_align(&mut self) -> CSSTextAlign {
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

