/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
extern mod srid_css;

use std::cast;
use std::cast::transmute;
use color::Color;
use select::SelectResults;
use computed::*;
use values::*;
use srid_css::select::common::*;
use srid_css::select::dispatch::*;
use srid_css::utils::errors::*;
use srid_css::include::types::*;
use srid_css::include::fpmath::*;
use srid_css::include::properties::*;
use helpers::computed::CssComputedStyle;


pub type ComputeFontSizeCb = @fn(parent: Option<& ~css_hint>, child: &~css_hint) -> ~css_hint;

pub struct CompleteSelectResults {
    inner: SelectResults
}
    
// Merge parent and child styles into another style. The result
// pointer may point to the child style, in which case the child
// style is overwritten
pub fn compose(parent: &CssComputedStyle, child: &mut CssComputedStyle,
                    result: &mut CssComputedStyle) {
    // println(fmt!("complete.rs :: compose"));
    // let llparent = parent.computed_style;
    // let llchild = child.computed_style;
    // let llresult = result.computed_style;
    let err = css_computed_style_compose(unsafe{transmute(parent.computed_style)}, unsafe{transmute(child.computed_style)}, compute_font_size_cb, unsafe{transmute(result.computed_style)});
    // println(fmt!("compose :: err == %?" , err));
    if err as uint != CSS_OK as uint {
        fail!(~"stylesheet composition failed")
    }
}

fn compute_font_size_cb(parent: Option<&~css_hint>, size: &mut ~css_hint) -> css_error {
    // println(fmt!("complete.rs :: compute_font_size_cb"));
    //let hlcbptr: *ComputeFontSizeCb = unsafe { transmute(pw) };
    let cb: ComputeFontSizeCb =
        |parent: Option<&~css_hint>, child: &~css_hint| -> ~css_hint {
            // println(fmt!("complete.rs :: ComputeFontSizeCb in compute_font_size_cb"));
        if child.length.is_some() {
            // Handle relative units
            match child.length.get_ref().unit {
                CSS_UNIT_EM | CSS_UNIT_PCT=> {
                    if parent.is_some() {
                        let mut new_value: float = 16.0;
                        if parent.get_ref().length.is_some() {
                            new_value = css_fixed_to_float(parent.get_ref().length.get_ref().value);
                            if child.length.get_ref().unit as uint == CSS_UNIT_EM as uint {
                                new_value *= css_fixed_to_float(child.length.get_ref().value);    
                                new_value = float_to_css_fixed(new_value) as float;
                            }
                            else {
                                new_value *= css_fixed_to_float(child.length.get_ref().value)/100.0;
                            }
                            
                        }
                        ~css_hint {
                            hint_type: HINT_LENGTH,
                            status: CSS_FONT_SIZE_DIMENSION as u8,
                            clip: None,
                            content: None,
                            counters: None,
                            length:Some(~css_hint_length { value:new_value as i32, unit:parent.get_ref().length.get_ref().unit}),
                            position: None,
                            color: 0,
                            fixed: 0,
                            integer: 0,
                            string: None,
                            strings: None
                        }
                    }
                        // CSS3 Values 5.1.1: Multiply parent unit by child unit.
                    else {
                        ~css_hint {
                            hint_type: HINT_LENGTH,
                            status: CSS_FONT_SIZE_DIMENSION as u8,
                            clip: None,
                            content: None,
                            counters: None,
                            length:Some(~css_hint_length { value:FLTTOFIX(16.0), unit:CSS_UNIT_PX}),
                            position: None,
                            color: 0,
                            fixed: 0,
                            integer: 0,
                            string: None,
                            strings: None
                        }
                    }
                },
                // Pass through absolute units    
                unit => {
                    ~css_hint {
                        hint_type: HINT_LENGTH,
                        status: CSS_FONT_SIZE_DIMENSION as u8,
                        clip: None,
                        content: None,
                        counters: None,
                        length:Some(~css_hint_length { value:child.length.get_ref().value, unit:unit}),
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
            
        else { 
            ~css_hint {
                hint_type: HINT_LENGTH,
                status: CSS_FONT_SIZE_DIMENSION as u8,
                clip: None,
                content: None,
                counters: None,
                length:Some(~css_hint_length { value:FLTTOFIX(16.0), unit:CSS_UNIT_PX}),
                position: None,
                color: 0,
                fixed: 0,
                integer: 0,
                string: None,
                strings: None
            }    
        }
    };
    
     
    let new_hint = cb(parent,size);
    size.hint_type = HINT_LENGTH;
    size.length.get_mut_ref().unit =  new_hint.length.get_ref().unit ;
    size.length.get_mut_ref().value = new_hint.length.get_ref().value ;
    size.status = new_hint.status ;
    
    
    CSS_OK    
}

impl<'self> CompleteSelectResults {
    pub fn new_root(root: SelectResults) -> CompleteSelectResults {
        //println(fmt!("complete.rs :: new_root"));
        CompleteSelectResults {
            inner: root
        }
    }

    pub fn new_from_parent(parent: &CompleteSelectResults,
                           child: SelectResults) -> CompleteSelectResults {
        // New lifetime
        //println(fmt!("complete.rs :: new_from_parent"));
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
        //println(fmt!("complete.rs :: computed_style"));
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
        //println(fmt!("complete.rs :: margin_top"));
        strip(self.inner.margin_top())
    }

    pub fn margin_right(&self) -> CSSMargin {
        // println(fmt!("complete.rs :: margin_right"));
        strip(self.inner.margin_right())
    }

    pub fn margin_bottom(&self) -> CSSMargin {
        // println(fmt!("complete.rs :: margin_bottom"));
        strip(self.inner.margin_bottom())
    }

    pub fn margin_left(&self) -> CSSMargin {
        // println(fmt!("complete.rs :: margin_left"));
        strip(self.inner.margin_left())
    }

    pub fn padding_top(&self) -> CSSPadding {
        // println(fmt!("complete.rs :: padding_top"));
        strip(self.inner.padding_top())
    }

    pub fn padding_right(&self) -> CSSPadding {
        // println(fmt!("complete.rs :: padding_right"));
        strip(self.inner.padding_right())
    }

    pub fn padding_bottom(&self) -> CSSPadding {
        // println(fmt!("complete.rs :: margin_top"));
        strip(self.inner.padding_bottom())
    }

    pub fn padding_left(&self) -> CSSPadding {
        // println(fmt!("complete.rs :: padding_left"));
        strip(self.inner.padding_left())
    }

    pub fn border_top_style(&self) -> CSSBorderStyle {
        // println(fmt!("complete.rs :: border_top_style"));
        strip(self.inner.border_top_style())
    }

    pub fn border_right_style(&self) -> CSSBorderStyle {
        // println(fmt!("complete.rs :: border_right_style"));
        strip(self.inner.border_right_style())
    }
    pub fn border_bottom_style(&self) -> CSSBorderStyle {
        // println(fmt!("complete.rs :: border_bottom_style"));
        strip(self.inner.border_bottom_style())
    }

    pub fn border_left_style(&self) -> CSSBorderStyle {
        // println(fmt!("complete.rs :: border_left_style"));
        strip(self.inner.border_left_style())
    }

    pub fn border_top_width(&self) -> CSSBorderWidth {
        // println(fmt!("complete.rs :: border_top_width"));
        strip(self.inner.border_top_width())
    }

    pub fn border_right_width(&self) -> CSSBorderWidth {
        // println(fmt!("complete.rs :: border_right_width"));
        strip(self.inner.border_right_width())
    }

    pub fn border_bottom_width(&self) -> CSSBorderWidth {
        // println(fmt!("complete.rs :: border_bottom_width"));
        strip(self.inner.border_bottom_width())
    }

    pub fn border_left_width(&self) -> CSSBorderWidth {
        // println(fmt!("complete.rs :: border_left_width"));
        strip(self.inner.border_left_width())
    }

    pub fn border_top_color(&self) -> Color {
        // println(fmt!("complete.rs :: border_top_color"));
        strip(self.inner.border_top_color())
    }

    pub fn border_right_color(&self) -> Color {
        // println(fmt!("complete.rs :: border_right_color"));
        strip(self.inner.border_right_color())
    }

    pub fn border_bottom_color(&self) -> Color {
        // println(fmt!("complete.rs :: border_bottom_color"));
        strip(self.inner.border_bottom_color())
    }

    pub fn border_left_color(&self) -> Color {
        // println(fmt!("complete.rs :: border_left_color"));
        strip(self.inner.border_left_color())
    }

    // CSS 2.1, Section 9 - Visual formatting model

    pub fn display(&self, root: bool) -> CSSDisplay {
        // println(fmt!("complete.rs :: display"));
        strip(self.inner.display(root))
    }

    pub fn position(&self) -> CSSPosition {
        // println(fmt!("complete.rs :: position"));
        strip(self.inner.position())
    }

    pub fn float(&self) -> CSSFloat {
        // println(fmt!("complete.rs :: float"));
        strip(self.inner.float())
    }

    pub fn clear(&self) -> CSSClear {
        // println(fmt!("complete.rs :: clear"));
        strip(self.inner.clear())
    }

    // CSS 2.1, Section 10 - Visual formatting model details

    pub fn width(&self) -> CSSWidth {
        // println(fmt!("complete.rs :: width"));
        strip(self.inner.width())
    }

    pub fn height(&self) -> CSSHeight {
        // println(fmt!("complete.rs :: height"));
        strip(self.inner.height())
    }

    pub fn line_height(&self) -> CSSLineHeight {
        // println(fmt!("complete.rs :: line_height"));
        strip(self.inner.line_height())
    }

    pub fn vertical_align(&self) -> CSSVerticalAlign {
        // println(fmt!("complete.rs :: vertical_align"));
        strip(self.inner.vertical_align())
    }

    // CSS 2.1, Section 11 - Visual effects

    // CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

    // CSS 2.1, Section 13 - Paged media

    // CSS 2.1, Section 14 - Colors and Backgrounds

    pub fn background_color(&self) -> Color {
        // println(fmt!("complete.rs :: background_color"));
        strip(self.inner.background_color())
    }

    pub fn color(&self) -> Color {
        // println(fmt!("complete.rs :: color"));
        strip(self.inner.color())
    }

    // CSS 2.1, Section 15 - Fonts

    pub fn font_family(&self) -> ~[CSSFontFamily] {
        // println(fmt!("complete.rs :: font_family"));
        strip(self.inner.font_family())
    }

    pub fn font_style(&self) -> CSSFontStyle {
        // println(fmt!("complete.rs :: font_style"));
        strip(self.inner.font_style())
    }

    pub fn font_weight(&self) -> CSSFontWeight {
        // println(fmt!("complete.rs :: font_weight"));
        strip(self.inner.font_weight())
    }

    pub fn font_size(&self) -> CSSFontSize {
        // println(fmt!("complete.rs :: font_size"));
        strip(self.inner.font_size())
    }

    pub fn text_decoration(&self) -> CSSTextDecoration{
        // println(fmt!("complete.rs :: text_decoration"));
        strip(self.inner.text_decoration())
    }

    // CSS 2.1, Section 16 - Text

    pub fn text_align(&self) -> CSSTextAlign {
        // println(fmt!("complete.rs :: text_align"));
        strip(self.inner.text_align())
    }

    // CSS 2.1, Section 17 - Tables

    // CSS 2.1, Section 18 - User interface

}

fn strip<T>(value: CSSValue<T>) -> T {
    // println(fmt!("complete.rs :: strip"));
    match value {
        Inherit => fail!(~"unexpected 'inherit' value in complete style"),
        Specified(v) => v
    }
}

