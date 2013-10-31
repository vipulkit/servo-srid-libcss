/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern mod srid_css;

use units::{Length, Px, Em};
//use netsurfcss::util::css_fixed_to_float;
use std::either::{Either, Left, Right};
//use n;
use values::*;
use srid_css::include::types::*;
use srid_css::libwapcaplet::wapcaplet::*;
use srid_css::stylesheet::*;
use srid_css::select::common::*;
use srid_css::include::properties::*;
use srid_css::select::computed::*;
use std::cast::*;

use color::{Color, rgba};

pub enum CssPseudoElement {
    CssPseudoElementNone         = 0,
    CssPseudoElementFirstLine   = 1,
    CssPseudoElementFirstLetter = 2,
    CssPseudoElementBefore       = 3,
    CssPseudoElementAfter        = 4,
    CssPseudoElementCount   = 5
}

pub struct ComputedStyle<'self> {
    //inner: css_computed_style<'self>
    result_backref: &'self SelectResults,
    computed_style: ~css_computed_style,
}

/**
Represents the 'style' of a single node, including it's pseudo-elements.
*/
pub struct SelectResults {
    inner: Option<~css_select_results>
}

impl<'self> SelectResults {
    /** Retrieve the computed style of a single pseudo-element */
    pub fn computed_style(&'self self) -> ComputedStyle<'self> {
        let element : uint = CssPseudoElementNone as uint;
        assert!(self.inner.is_some());
        let res = self.inner.get_ref() ;
        let llstyle = res.styles[element].deep_clone().expect("");
    
        ComputedStyle {
            result_backref: self,
            computed_style: llstyle
        }
    }
}

impl<'self> ComputedStyle<'self> {

    // CSS 2.1, Section 8 - Box model

    pub fn margin_top(&'self mut self) -> CSSValue<CSSMargin> {
        //convert_net_margin(self.margin_top());
        let (value, olength, ounit) = css_computed_margin_top(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
      
        convert_net_margin(unsafe { transmute(value as uint)},  unit , length)
    }
    
    pub fn margin_right(&'self mut self) -> CSSValue<CSSMargin> {
        //convert_net_margin(self.inner.margin_right())
        let (value, olength, ounit) = css_computed_margin_right(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
      
        convert_net_margin(unsafe { transmute(value as uint)},  unit, length)
    }

    pub fn margin_bottom(&'self mut self) -> CSSValue<CSSMargin> {
        //convert_net_margin(self.inner.margin_bottom())
        let (value, olength, ounit) = css_computed_margin_bottom(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
      
        convert_net_margin(unsafe { transmute(value as uint)},  unit, length)
    }

    pub fn margin_left(&'self mut self) -> CSSValue<CSSMargin> {
        //convert_net_margin(self.inner.margin_left())
        let (value, olength, ounit) = css_computed_margin_left(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
        convert_net_margin(unsafe { transmute(value as uint)},  unit, length)
    }

    pub fn padding_top(&'self mut self) -> CSSValue<CSSPadding> {
        //convert_net_padding(self.inner.padding_top())

        let (value, olength, ounit) = css_computed_padding_top(&'self mut self.computed_style);

        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
      
        convert_net_padding(unsafe { transmute(value as uint)},  unit, length)

    }

    pub fn padding_right(&'self mut self) -> CSSValue<CSSPadding> {
        //convert_net_padding(self.inner.padding_right())
        let (value, olength, ounit) = css_computed_padding_right(&'self mut self.computed_style);

        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
      
        convert_net_padding(unsafe { transmute(value as uint)},  unit, length)
    }

    pub fn padding_bottom(&'self mut self) -> CSSValue<CSSPadding> {
        //convert_net_padding(self.inner.padding_bottom())
        let (value, olength, ounit) = css_computed_padding_bottom(&'self mut self.computed_style);

        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
        convert_net_padding(unsafe { transmute(value as uint)},  unit, length)
    }

    pub fn padding_left(&'self mut self) -> CSSValue<CSSPadding> {
        //convert_net_padding(self.inner.padding_left())
        let (value, olength, ounit) = css_computed_padding_left(&'self mut self.computed_style);

        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
        convert_net_padding(unsafe { transmute(value as uint)},  unit, length)
        
    }

    pub fn border_top_style(&'self mut self) -> CSSValue<CSSBorderStyle> {
        //convert_net_border_style(self.inner.border_top_style())
        let value = css_computed_border_top_style(&'self mut self.computed_style);
        convert_net_border_style(unsafe { transmute(value as uint)})
    }

    pub fn border_right_style(&'self mut self) -> CSSValue<CSSBorderStyle> {
        //convert_net_border_style(self.inner.border_right_style())
        let value = css_computed_border_right_style(&'self mut self.computed_style);
        convert_net_border_style(unsafe { transmute(value as uint)})
    }

    pub fn border_bottom_style(&'self mut self) -> CSSValue<CSSBorderStyle> {
        //convert_net_border_style(self.inner.border_bottom_style())
        let value = css_computed_border_bottom_style(&'self mut self.computed_style);
        convert_net_border_style(unsafe { transmute(value as uint)})
    }

    pub fn border_left_style(&'self mut self) -> CSSValue<CSSBorderStyle> {
        //convert_net_border_style(self.inner.border_left_style())
         let value = css_computed_border_left_style(&'self mut self.computed_style);
        convert_net_border_style(unsafe { transmute(value as uint)})
    }

    pub fn border_top_width(&'self mut self) -> CSSValue<CSSBorderWidth> {
        //convert_net_border_width(self.inner.border_top_width())
        let (value, olength, ounit) = css_computed_border_top_width(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
      
        convert_net_border_width(unsafe { transmute(value as uint)},  unit, length)
    }

    pub fn border_right_width(&'self mut self) -> CSSValue<CSSBorderWidth> {
        //convert_net_border_width(self.inner.border_right_width())
        let (value, olength, ounit) = css_computed_border_right_width(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
      
        convert_net_border_width(unsafe { transmute(value as uint)},  unit, length)
    }

    pub fn border_bottom_width(&'self mut self) -> CSSValue<CSSBorderWidth> {
        //convert_net_border_width(self.inner.border_bottom_width())
        let (value, olength, ounit) = css_computed_border_bottom_width(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
      
        convert_net_border_width(unsafe { transmute(value as uint)},  unit, length)
    }

    pub fn border_left_width(&'self mut self) -> CSSValue<CSSBorderWidth> {
        //convert_net_border_width(self.inner.border_left_width())
        let (value, olength, ounit) = css_computed_border_left_width(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
      
        convert_net_border_width(unsafe { transmute(value as uint)},  unit, length)
    }

    pub fn border_top_color(&'self mut self) -> CSSValue<Color> {
        //convert_net_color_value(self.inner.border_top_color())
        let (value, length) = css_computed_border_top_color(&'self mut self.computed_style);
        convert_net_color_value(unsafe { transmute(value as uint)}, length)
    }

    pub fn border_right_color(&'self mut self) -> CSSValue<Color> {
        //convert_net_color_value(self.inner.border_right_color())
        let (value, length) = css_computed_border_right_color(&'self mut self.computed_style);
        convert_net_color_value(unsafe { transmute(value as uint)}, length)
    }

    pub fn border_bottom_color(&'self mut self) -> CSSValue<Color> {
        //convert_net_color_value(self.inner.border_bottom_color())
         let (value, length) = css_computed_border_bottom_color(&'self mut self.computed_style);
        convert_net_color_value(unsafe { transmute(value as uint)}, length)
    }

    pub fn border_left_color(&'self mut self) -> CSSValue<Color> {
        //convert_net_color_value(self.inner.border_left_color())

        let (value, length) = css_computed_border_left_color(&'self mut self.computed_style);
        convert_net_color_value(unsafe { transmute(value as uint)}, length)
    }


    // CSS 2.1, Section 9 - Visual formatting model

    pub fn display(&'self mut self, root: bool) -> CSSValue<CSSDisplay> {
        //convert_net_display_value(self.inner.display(root))
        let value = css_computed_display(&'self mut self.computed_style, root);
        convert_net_display_value(unsafe { transmute(value as uint)})
    }

    pub fn position(&'self mut self) -> CSSValue<CSSPosition> {
        //convert_net_position_value(self.inner.position())
        let value = css_computed_position(&'self mut self.computed_style);
        convert_net_position_value(unsafe { transmute(value as uint)})

    }

    pub fn float(&'self mut self) -> CSSValue<CSSFloat> {
        //convert_net_float_value(self.inner.float())
        let value = css_computed_float(&'self mut self.computed_style);
        convert_net_float_value(unsafe { transmute(value as uint)})
    }


    pub fn clear(&'self mut self) -> CSSValue<CSSClear> {
        //convert_net_clear_value(self.inner.clear())
        let value = css_computed_clear(&'self mut self.computed_style);
        convert_net_clear_value(unsafe { transmute(value as uint)})
    }

    // CSS 2.1, Section 10 - Visual formatting model details

    pub fn width(&'self mut self) -> CSSValue<CSSWidth> {
        //convert_net_width_value(self.inner.width())
        let (value, olength, ounit) = css_computed_width(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
              
        convert_net_width_value( unsafe { transmute(value as uint)},  unit, length)

    }

    pub fn height(&'self mut self) -> CSSValue<CSSHeight> {
        //convert_net_height_value(self.inner.height())
        let (value, olength, ounit) = css_computed_height(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
              
        convert_net_height_value(unsafe {transmute(value as uint)},  unit, length)
    }

    pub fn line_height(&'self mut self) -> CSSValue<CSSLineHeight> {
        //convert_net_line_height_value(self.inner.line_height())

        let (value, olength, ounit) = css_computed_line_height(&'self mut self.computed_style);
        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
              
        convert_net_line_height_value(unsafe { transmute(value as uint)},  unit, length)
    }

    pub fn vertical_align(&'self mut self) -> CSSValue<CSSVerticalAlign> {
        //convert_net_vertical_align_value(self.inner.vertical_align())
        let (value, olength, ounit) = css_computed_vertical_align(&'self mut self.computed_style);

        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
              
        convert_net_vertical_align_value(unsafe { transmute(value as uint)},  unit, length)
    }

    // CSS 2.1, Section 11 - Visual effects

    // CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

    // CSS 2.1, Section 13 - Paged media

    // CSS 2.1, Section 14 - Colors and Backgrounds

    pub fn background_color(&'self mut self) -> CSSValue<Color> {
        //convert_net_color_value(self.inner.background_color())
        let (value, length) = css_computed_background_color(&'self mut self.computed_style);

        let mut len = 0;
        if length.is_some()
        {
            len = length.expect("")
        }
        convert_net_color_value(unsafe { transmute(value as uint)}, len)
    }

    pub fn color(&'self mut self) -> CSSValue<Color> {
        //convert_net_color_value(self.inner.color())
        let (value, length) = css_computed_color(&'self mut self.computed_style);
        let mut len = 0;
        if length.is_some()
        {
            len = length.expect("")
        }
        convert_net_color_value(unsafe { transmute(value as uint)}, len)
    }

    // CSS 2.1, Section 15 - Fonts

    pub fn font_family(&'self mut self) -> CSSValue<~[CSSFontFamily]> {
       // convert_net_font_family_value(self.inner.font_family())
       let (value, namesi) = css_computed_font_family(&'self mut self.computed_style);
       convert_net_font_family_value(unsafe { transmute(value as uint)}, namesi)
    }


    pub fn font_style(&'self mut self) -> CSSValue<CSSFontStyle> {
        //convert_net_font_style_value(self.inner.font_style())
        let value = css_computed_font_style(&'self mut self.computed_style);
        convert_net_font_style_value(unsafe { transmute(value as uint)})

    }

    pub fn font_weight(&'self mut self) -> CSSValue<CSSFontWeight> {
        //convert_net_font_weight_value(self.inner.font_weight())
        let value = css_computed_font_weight(&'self mut self.computed_style);
        convert_net_font_weight_value(unsafe { transmute(value as uint)})

    }

    pub fn font_size(&'self mut self) -> CSSValue<CSSFontSize> {
        //convert_net_font_size_value(self.inner.font_size())

        let (value, olength, ounit) = css_computed_font_size(&'self mut self.computed_style);

        let mut length = 0 ;
        let mut unit = CSS_UNIT_PX;
        if (olength.is_some())
        {
            length = olength.expect("")
        }
        if (ounit.is_some())
        {
            unit = ounit.expect("");
        }
              
        convert_net_font_size_value(unsafe { transmute(value as uint)},  unit, length)
    }

    // CSS 2.1, Section 16 - Text

    pub fn text_align(&'self mut self) -> CSSValue<CSSTextAlign> {
        //convert_net_text_align_value(self.inner.text_align())
        let value = css_computed_text_align(&'self mut self.computed_style);
        convert_net_text_align_value(unsafe { transmute(value as uint)})
    }

    pub fn text_decoration(&'self mut self) -> CSSValue<CSSTextDecoration> {
        //convert_net_text_decoration_value(self.inner.text_decoration())
        let value = css_computed_text_decoration(&'self mut self.computed_style);
        convert_net_text_decoration_value(unsafe { transmute(value as uint)})
    }
    // CSS 2.1, Section 17 - Tables

    // CSS 2.1, Section 18 - User interface

}


fn convert_net_color(color:  css_color) -> Color {
    rgba( (color&0x00FF0000 >> 16) as u8, 
          (color&0x0000FF00 >> 8) as u8, 
          (color&0x000000FF >> 0) as u8 , 
          (color&0xFF000000 >> 24) as float 
        )
}

fn convert_net_color_value(color: css_color_e, value:u32) -> CSSValue<Color> {
    
    match color {
        CSS_COLOR_INHERIT => Inherit,
        CSS_COLOR_COLOR => Specified(convert_net_color(value))
    }
}

fn convert_net_border_style(style: css_border_style_e) -> CSSValue<CSSBorderStyle> {
    match style {
        CSS_BORDER_STYLE_INHERIT  => Inherit,
        CSS_BORDER_STYLE_NONE   => Specified(CSSBorderStyleNone),
        CSS_BORDER_STYLE_HIDDEN  => Specified(CSSBorderStyleHidden),
        CSS_BORDER_STYLE_DOTTED  => Specified(CSSBorderStyleDotted),
        CSS_BORDER_STYLE_DASHED => Specified(CSSBorderStyleDashed),
        CSS_BORDER_STYLE_SOLID  => Specified(CSSBorderStyleSolid),
        CSS_BORDER_STYLE_DOUBLE  => Specified(CSSBorderStyleDouble),
        CSS_BORDER_STYLE_GROOVE  => Specified(CSSBorderStyleGroove),
        CSS_BORDER_STYLE_RIDGE  => Specified(CSSBorderStyleRidge),
        CSS_BORDER_STYLE_INSET   => Specified(CSSBorderStyleInset),
        CSS_BORDER_STYLE_OUTSET  => Specified(CSSBorderStyleOutset)
    }
}

fn convert_net_border_width(width: css_border_width_e, value:css_unit, l:css_fixed) -> CSSValue<CSSBorderWidth> {
    match width {

        CSS_BORDER_WIDTH_INHERIT    => Inherit,
        CSS_BORDER_WIDTH_THIN       => Specified(CSSBorderWidthThin),
        CSS_BORDER_WIDTH_MEDIUM    => Specified(CSSBorderWidthMedium),
        CSS_BORDER_WIDTH_THICK     => Specified(CSSBorderWidthThick),
        CSS_BORDER_WIDTH_WIDTH  => Specified(CSSBorderWidthLength(convert_net_unit_to_length(value, l)))

    }
}

fn convert_net_margin(margin: css_margin_e, value:css_unit, l:css_fixed) -> CSSValue<CSSMargin> {
    match margin {
        CSS_MARGIN_INHERIT   => Inherit,
        CSS_MARGIN_SET => {
            let length = convert_net_unit_to_length_or_percent(value, l);
                match length {
                    Left(abs) => Specified(CSSMarginLength(abs)),
                    Right(percent) => Specified(CSSMarginPercentage(percent))
                }
            },
        CSS_MARGIN_AUTO  => Specified(CSSMarginAuto)
        
    }
}

fn convert_net_padding(padding: css_padding_e, value:css_unit, l:css_fixed) -> CSSValue<CSSPadding> {
    match padding {
        CSS_PADDING_INHERIT => Inherit,
        CSS_PADDING_SET => {
            let length = convert_net_unit_to_length_or_percent(value, l);
            match length {
                Left(abs) => Specified(CSSPaddingLength(abs)),
                Right(percent) => Specified(CSSPaddingPercentage(percent))
            }
        }
    }
}

fn convert_net_width_value(value: css_width_e, wvalue: css_unit, l:css_fixed) -> CSSValue<CSSWidth> {
    match value {
        CSS_WIDTH_INHERIT => Inherit,
        CSS_WIDTH_SET => {
            let length = convert_net_unit_to_length_or_percent(wvalue, l);
            match length {
                Left(abs) => Specified(CSSWidthLength(abs)),
                Right(percent) => Specified(CSSWidthPercentage(percent))
            }
        }
        CSS_WIDTH_AUTO => Specified(CSSWidthAuto)
    }
}

fn convert_net_height_value(value: css_height_e , hvalue:css_unit, l:css_fixed) -> CSSValue<CSSHeight> {
    match value {
        CSS_HEIGHT_INHERIT => Inherit,
        CSS_HEIGHT_SET => {
            let length = convert_net_unit_to_length_or_percent(hvalue, l);
            match length {
                Left(abs) => Specified(CSSHeightLength(abs)),
                Right(percent) => Specified(CSSHeightPercentage(percent))
            }
        }
        CSS_HEIGHT_AUTO => Specified(CSSHeightAuto)
    }
}

fn convert_net_display_value(value: css_display_e) -> CSSValue<CSSDisplay> {
    match value {
        CSS_DISPLAY_INHERIT      => Inherit,
        CSS_DISPLAY_INLINE   => Specified(CSSDisplayInline),
        CSS_DISPLAY_BLOCK    => Specified(CSSDisplayInline),
        CSS_DISPLAY_LIST_ITEM => Specified(CSSDisplayListItem),
        CSS_DISPLAY_RUN_IN => unimpl("display: run-in"), // FIXME: Not in CSS 2.1         = 0x04,
        CSS_DISPLAY_INLINE_BLOCK => Specified(CSSDisplayInlineBlock),
        CSS_DISPLAY_TABLE  => Specified(CSSDisplayTable),
        CSS_DISPLAY_INLINE_TABLE    => Specified(CSSDisplayInlineTable),     
        CSS_DISPLAY_TABLE_ROW_GROUP   => Specified(CSSDisplayTableRowGroup),
        CSS_DISPLAY_TABLE_HEADER_GROUP   => Specified(CSSDisplayTableHeaderGroup),
        CSS_DISPLAY_TABLE_FOOTER_GROUP  => Specified(CSSDisplayTableFooterGroup),
        CSS_DISPLAY_TABLE_ROW => Specified(CSSDisplayTableRow),
        CSS_DISPLAY_TABLE_COLUMN_GROUP => Specified(CSSDisplayTableColumnGroup),
        CSS_DISPLAY_TABLE_COLUMN    => Specified(CSSDisplayTableColumn),
        CSS_DISPLAY_TABLE_CELL    => Specified(CSSDisplayTableCell),
        CSS_DISPLAY_TABLE_CAPTION   => Specified(CSSDisplayTableCaption),
        CSS_DISPLAY_NONE    => Specified(CSSDisplayNone)
       
    }
}

fn convert_net_float_value(value: css_float_e) -> CSSValue<CSSFloat> {
    match value {
        CSS_FLOAT_INHERIT => Inherit,
        CSS_FLOAT_LEFT => Specified(CSSFloatLeft),
        CSS_FLOAT_RIGHT  => Specified(CSSFloatRight),
        CSS_FLOAT_NONE => Specified(CSSFloatNone)
    }
}

fn convert_net_clear_value(value: css_clear_e) -> CSSValue<CSSClear> {
    match value {
        CSS_CLEAR_INHERIT => Inherit,
        CSS_CLEAR_NONE => Specified(CSSClearNone),
        CSS_CLEAR_LEFT => Specified(CSSClearLeft),
        CSS_CLEAR_RIGHT => Specified(CSSClearRight),
        CSS_CLEAR_BOTH => Specified(CSSClearBoth)
    }
}

fn convert_net_position_value(value: css_position_e) -> CSSValue<CSSPosition> {
    match value {
        CSS_POSITION_INHERIT => Inherit,
        CSS_POSITION_STATIC => Specified(CSSPositionStatic),
        CSS_POSITION_RELATIVE => Specified(CSSPositionRelative),
        CSS_POSITION_ABSOLUTE => Specified(CSSPositionAbsolute),
        CSS_POSITION_FIXED => Specified(CSSPositionFixed)
    }
}

fn convert_net_font_family_value(value: css_font_family_e, names:~[uint]) -> CSSValue<~[CSSFontFamily]> {
    use units::{Serif, SansSerif, Cursive, Fantasy, Monospace};
    if (names.len() != 0){
        return Specified(names.map(|&n| CSSFontFamilyFamilyName(unsafe{lwc_ref.get_mut_ref()}.lwc_string_data(n) )));
    }

    match value {
        CSS_FONT_FAMILY_INHERIT => Inherit,
        CSS_FONT_FAMILY_SERIF => Specified(~[CSSFontFamilyGenericFamily(Serif)]),
        CSS_FONT_FAMILY_SANS_SERIF => Specified(~[CSSFontFamilyGenericFamily(SansSerif)]),
        CSS_FONT_FAMILY_CURSIVE => Specified(~[CSSFontFamilyGenericFamily(Cursive)]),
        CSS_FONT_FAMILY_FANTASY => Specified(~[CSSFontFamilyGenericFamily(Fantasy)]),
        CSS_FONT_FAMILY_MONOSPACE => Specified(~[CSSFontFamilyGenericFamily(Monospace)]),
    }
}

fn convert_net_font_size_value(value: css_font_size_e,size:css_unit, l:css_fixed) -> CSSValue<CSSFontSize> {
    use units::*;
    match value {
        CSS_FONT_SIZE_INHERIT => Inherit,
        CSS_FONT_SIZE_XX_SMALL => Specified(CSSFontSizeAbsoluteSize(XXSmall)),
        CSS_FONT_SIZE_X_SMALL => Specified(CSSFontSizeAbsoluteSize(XSmall)),
        CSS_FONT_SIZE_SMALL => Specified(CSSFontSizeAbsoluteSize(Small)),
        CSS_FONT_SIZE_MEDIUM => Specified(CSSFontSizeAbsoluteSize(Medium)),
        CSS_FONT_SIZE_LARGE => Specified(CSSFontSizeAbsoluteSize(Large)),
        CSS_FONT_SIZE_X_LARGE => Specified(CSSFontSizeAbsoluteSize(XLarge)),
        CSS_FONT_SIZE_XX_LARGE => Specified(CSSFontSizeAbsoluteSize(XXLarge)),
        CSS_FONT_SIZE_LARGER => Specified(CSSFontSizeRelativeSize(Larger)),
        CSS_FONT_SIZE_SMALLER => Specified(CSSFontSizeRelativeSize(Smaller)),
        CSS_FONT_SIZE_DIMENSION => {
            match convert_net_unit_to_length_or_percent(size, l) {
                Left(val) => Specified(CSSFontSizeLength(val)),
                Right(val) => Specified(CSSFontSizePercentage(val))
            }
        }
    }
}

fn convert_net_font_style_value(value: css_font_style_e) -> CSSValue<CSSFontStyle> {
    match value {
        CSS_FONT_STYLE_INHERIT => Inherit,
        CSS_FONT_STYLE_NORMAL => Specified(CSSFontStyleNormal),
        CSS_FONT_STYLE_ITALIC => Specified(CSSFontStyleItalic),
        CSS_FONT_STYLE_OBLIQUE => Specified(CSSFontStyleOblique)
    }
}

fn convert_net_font_weight_value(value: css_font_weight_e) -> CSSValue<CSSFontWeight> {
    match value {

        CSS_FONT_WEIGHT_INHERIT => Inherit,
        CSS_FONT_WEIGHT_NORMAL => Specified(CSSFontWeightNormal),
        CSS_FONT_WEIGHT_BOLD => Specified(CSSFontWeightBold),
        CSS_FONT_WEIGHT_BOLDER => Specified(CSSFontWeightBolder),
        CSS_FONT_WEIGHT_LIGHTER => Specified(CSSFontWeightLighter),
        CSS_FONT_WEIGHT_100 => Specified(CSSFontWeight100),
        CSS_FONT_WEIGHT_200 => Specified(CSSFontWeight200),
        CSS_FONT_WEIGHT_300 => Specified(CSSFontWeight300),
        CSS_FONT_WEIGHT_400 => Specified(CSSFontWeight400),
        CSS_FONT_WEIGHT_500 => Specified(CSSFontWeight500),
        CSS_FONT_WEIGHT_600 => Specified(CSSFontWeight600),
        CSS_FONT_WEIGHT_700 => Specified(CSSFontWeight700),
        CSS_FONT_WEIGHT_800 => Specified(CSSFontWeight800),
        CSS_FONT_WEIGHT_900 => Specified(CSSFontWeight900),
    }
}

fn convert_net_text_align_value(value: css_text_align_e) -> CSSValue<CSSTextAlign> {
    match value {
        CSS_TEXT_ALIGN_INHERIT => Inherit,
        CSS_TEXT_ALIGN_INHERIT_IF_NON_MAGIC => unimpl("inherit if non-magic? wtf?"),
        CSS_TEXT_ALIGN_LEFT => Specified(CSSTextAlignLeft),
        CSS_TEXT_ALIGN_RIGHT => Specified(CSSTextAlignRight),
        CSS_TEXT_ALIGN_CENTER => Specified(CSSTextAlignCenter),
        CSS_TEXT_ALIGN_JUSTIFY => Specified(CSSTextAlignJustify),
        CSS_TEXT_ALIGN_DEFAULT => Specified(CSSTextAlignLeft),
        CSS_TEXT_ALIGN_LIBCSS_LEFT => unimpl("text-align libcss left"),
        CSS_TEXT_ALIGN_LIBCSS_CENTER => unimpl("text-align libcss center"),
        CSS_TEXT_ALIGN_LIBCSS_RIGHT => unimpl("text-align libcss right"),
    }
}

fn convert_net_text_decoration_value(value: css_text_decoration_e) -> CSSValue<CSSTextDecoration> {
    match value {
        CSS_TEXT_DECORATION_INHERIT => Inherit,
        CSS_TEXT_DECORATION_NONE => Specified(CSSTextDecorationNone),
        CSS_TEXT_DECORATION_BLINK => Specified(CSSTextDecorationBlink),
        CSS_TEXT_DECORATION_LINE_THROUGH => Specified(CSSTextDecorationLineThrough),
        CSS_TEXT_DECORATION_OVERLINE => Specified(CSSTextDecorationOverline),
        CSS_TEXT_DECORATION_UNDERLINE => Specified(CSSTextDecorationUnderline),
    }
}

fn convert_net_line_height_value(value: css_line_height_e, v:css_unit, l:css_fixed) -> CSSValue<CSSLineHeight> {
    match value {
        CSS_LINE_HEIGHT_INHERIT => Inherit,
        CSS_LINE_HEIGHT_NUMBER => Specified(CSSLineHeightNumber(css_fixed_to_float(v as i32))),
        CSS_LINE_HEIGHT_DIMENSION => {
            match convert_net_unit_to_length_or_percent(v, l) {
                Left(val) => Specified(CSSLineHeightLength(val)),
                Right(val) => Specified(CSSLineHeightPercentage(val))
            }
        },
        CSS_LINE_HEIGHT_NORMAL => Specified(CSSLineHeightNormal)
    }
}

fn convert_net_vertical_align_value(value: css_vertical_align_e, v:css_unit, l:css_fixed) -> CSSValue<CSSVerticalAlign> {
    match value {

        CSS_VERTICAL_ALIGN_INHERIT => Inherit,
        CSS_VERTICAL_ALIGN_BASELINE => Specified(CSSVerticalAlignBaseline),
        CSS_VERTICAL_ALIGN_SUB => Specified(CSSVerticalAlignSub),
        CSS_VERTICAL_ALIGN_SUPER => Specified(CSSVerticalAlignSuper),
        CSS_VERTICAL_ALIGN_TOP => Specified(CSSVerticalAlignTop),
        CSS_VERTICAL_ALIGN_TEXT_TOP => Specified(CSSVerticalAlignTextTop),
        CSS_VERTICAL_ALIGN_MIDDLE => Specified(CSSVerticalAlignMiddle),
        CSS_VERTICAL_ALIGN_BOTTOM => Specified(CSSVerticalAlignBottom),
        CSS_VERTICAL_ALIGN_TEXT_BOTTOM => Specified(CSSVerticalAlignTextBottom),
        CSS_VERTICAL_ALIGN_SET => {
            match convert_net_unit_to_length_or_percent(v, l) {
                Left(val) => Specified(CSSVerticalAlignLength(val)),
                Right(val) => Specified(CSSVerticalAlignPercentage(val))
            }
        }
    }
}

fn convert_net_unit_to_length(unit: css_unit, len:css_fixed) -> Length {
    match convert_net_unit_to_length_or_percent(unit,len) {
        Left(v) => v,
        Right(*) => fail!(~"unexpected percentage unit"),
    }
}

fn convert_net_unit_to_length_or_percent(unit: css_unit,l : css_fixed) -> Either<Length, float> {
    match unit {
        CSS_UNIT_PX => Left(Px(css_fixed_to_float(l))),
        CSS_UNIT_EM => Left(Em(css_fixed_to_float(l))),
        CSS_UNIT_PT => Left(Px(css_fixed_to_float(l) / 72f * 96f)),
        CSS_UNIT_CM => Left(Px(css_fixed_to_float(l) / 2.54f * 96f)),
        CSS_UNIT_MM => Left(Px(css_fixed_to_float(l) / 25.4f * 96f)),
        CSS_UNIT_IN => Left(Px(css_fixed_to_float(l) / 1f * 96f)),
        CSS_UNIT_PC => Left(Px(css_fixed_to_float(l) / 6f * 96f)),
        CSS_UNIT_PCT => Right(css_fixed_to_float(l)),
        _ => unimpl("unit")
    }
}

fn unimpl(what: &str) -> ! {
    fail!(fmt!("css unimplemented %?", what))
}

pub fn css_fixed_to_float(f: css_fixed) -> float {
    static BEFORE: i32 = 10;
    f as float * 1.0f / ((1i32 << BEFORE) as float)
}

pub fn float_to_css_fixed(f: float) -> css_fixed {
    static BEFORE: i32 = 10;
    (f * ((1 << BEFORE) as float)) as css_fixed
}
