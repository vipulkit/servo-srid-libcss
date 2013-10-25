/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern mod srid_css;
use std::either::{Either, Left, Right};
use srid_css::select::common::*;
use srid_css::include::types::*;
use srid_css::include::properties::*;
use srid_css::stylesheet::*;



//use netsurfcss::util::css_fixed_to_float;
//use n;
use color::{Color, rgba};
use units::{Length, Px, Em};
use values::*;



pub struct ComputedStyle {
    inner: css_computed_style
}

impl ComputedStyle {

    // CSS 2.1, Section 8 - Box model

    pub fn margin_top(&self) -> CSSValue<CSSMargin> {
        convert_net_margin(self.inner.margin_top())
    }

    pub fn margin_right(&self) -> CSSValue<CSSMargin> {
        convert_net_margin(self.inner.margin_right())
    }

    pub fn margin_bottom(&self) -> CSSValue<CSSMargin> {
        convert_net_margin(self.inner.margin_bottom())
    }

    pub fn margin_left(&self) -> CSSValue<CSSMargin> {
        convert_net_margin(self.inner.margin_left())
    }

    pub fn padding_top(&self) -> CSSValue<CSSPadding> {
        convert_net_padding(self.inner.padding_top())
    }

    pub fn padding_right(&self) -> CSSValue<CSSPadding> {
        convert_net_padding(self.inner.padding_right())
    }

    pub fn padding_bottom(&self) -> CSSValue<CSSPadding> {
        convert_net_padding(self.inner.padding_bottom())
    }

    pub fn padding_left(&self) -> CSSValue<CSSPadding> {
        convert_net_padding(self.inner.padding_left())
    }

    pub fn border_top_style(&self) -> CSSValue<CSSBorderStyle> {
        convert_net_border_style(self.inner.border_top_style())
    }

    pub fn border_right_style(&self) -> CSSValue<CSSBorderStyle> {
        convert_net_border_style(self.inner.border_right_style())
    }

    pub fn border_bottom_style(&self) -> CSSValue<CSSBorderStyle> {
        convert_net_border_style(self.inner.border_bottom_style())
    }

    pub fn border_left_style(&self) -> CSSValue<CSSBorderStyle> {
        convert_net_border_style(self.inner.border_left_style())
    }

    pub fn border_top_width(&self) -> CSSValue<CSSBorderWidth> {
        convert_net_border_width(self.inner.border_top_width())
    }

    pub fn border_right_width(&self) -> CSSValue<CSSBorderWidth> {
        convert_net_border_width(self.inner.border_right_width())
    }

    pub fn border_bottom_width(&self) -> CSSValue<CSSBorderWidth> {
        convert_net_border_width(self.inner.border_bottom_width())
    }

    pub fn border_left_width(&self) -> CSSValue<CSSBorderWidth> {
        convert_net_border_width(self.inner.border_left_width())
    }

    pub fn border_top_color(&self) -> CSSValue<Color> {
        convert_net_color_value(self.inner.border_top_color())
    }

    pub fn border_right_color(&self) -> CSSValue<Color> {
        convert_net_color_value(self.inner.border_right_color())
    }

    pub fn border_bottom_color(&self) -> CSSValue<Color> {
        convert_net_color_value(self.inner.border_bottom_color())
    }

    pub fn border_left_color(&self) -> CSSValue<Color> {
        convert_net_color_value(self.inner.border_left_color())
    }

    // CSS 2.1, Section 9 - Visual formatting model

    pub fn display(&self, root: bool) -> CSSValue<CSSDisplay> {
        convert_net_display_value(self.inner.display(root))
    }

    pub fn position(&self) -> CSSValue<CSSPosition> {
        convert_net_position_value(self.inner.position())
    }

    pub fn float(&self) -> CSSValue<CSSFloat> {
        convert_net_float_value(self.inner.float())
    }

    pub fn clear(&self) -> CSSValue<CSSClear> {
        convert_net_clear_value(self.inner.clear())
    }

    // CSS 2.1, Section 10 - Visual formatting model details

    pub fn width(&self) -> CSSValue<CSSWidth> {
        convert_net_width_value(self.inner.width())
    }

    pub fn height(&self) -> CSSValue<CSSHeight> {
        convert_net_height_value(self.inner.height())
    }

    pub fn line_height(&self) -> CSSValue<CSSLineHeight> {
        convert_net_line_height_value(self.inner.line_height())
    }

    pub fn vertical_align(&self) -> CSSValue<CSSVerticalAlign> {
        convert_net_vertical_align_value(self.inner.vertical_align())
    }

    // CSS 2.1, Section 11 - Visual effects

    // CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

    // CSS 2.1, Section 13 - Paged media

    // CSS 2.1, Section 14 - Colors and Backgrounds

    pub fn background_color(&self) -> CSSValue<Color> {
        convert_net_color_value(self.inner.background_color())
    }

    pub fn color(&self) -> CSSValue<Color> {
        convert_net_color_value(self.inner.color())
    }

    // CSS 2.1, Section 15 - Fonts

    pub fn font_family(&self) -> CSSValue<~[CSSFontFamily]> {
        convert_net_font_family_value(self.inner.font_family())
    }

    pub fn font_style(&self) -> CSSValue<CSSFontStyle> {
        convert_net_font_style_value(self.inner.font_style())
    }

    pub fn font_weight(&self) -> CSSValue<CSSFontWeight> {
        convert_net_font_weight_value(self.inner.font_weight())
    }

    pub fn font_size(&self) -> CSSValue<CSSFontSize> {
        convert_net_font_size_value(self.inner.font_size())
    }

    // CSS 2.1, Section 16 - Text

    pub fn text_align(&self) -> CSSValue<CSSTextAlign> {
        convert_net_text_align_value(self.inner.text_align())
    }

    pub fn text_decoration(&self) -> CSSValue<CSSTextDecoration> {
        convert_net_text_decoration_value(self.inner.text_decoration())
    }

    // CSS 2.1, Section 17 - Tables

    // CSS 2.1, Section 18 - User interface

}

fn convert_net_color(color:  css_color) -> Color {
    rgba(color.r, color.g, color.b, (color.a as float) / 255.0)
}

fn convert_net_color_value(color: css_color_e) -> CSSValue<Color> {
    match color {
        CSS_COLOR_INHERIT => Inherit,
        CSS_COLOR_COLOR(v) => Specified(convert_net_color(v))
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

fn convert_net_border_width(width: css_border_width_e) -> CSSValue<CSSBorderWidth> {
    match width {

        CSS_BORDER_WIDTH_INHERIT    => Inherit,
        CSS_BORDER_WIDTH_THIN       => Specified(CSSBorderWidthThin),
        CSS_BORDER_WIDTH_MEDIUM    => Specified(CSSBorderWidthMedium),
        CSS_BORDER_WIDTH_THICK     => Specified(CSSBorderWidthThick),
        CSS_BORDER_WIDTH_WIDTH(width)  => Specified(CSSBorderWidthLength(convert_net_unit_to_length(width)))
        
    }
}

fn convert_net_margin(margin: css_margin_e) -> CSSValue<CSSMargin> {
    match margin {
        CSS_MARGIN_INHERIT   => Inherit,
        CSS_MARGIN_SET (_e) => {
            let length = convert_net_unit_to_length_or_percent(_e);
                match length {
                    Left(abs) => Specified(CSSMarginLength(abs)),
                    Right(percent) => Specified(CSSMarginPercentage(percent))
                }
            },
        CSS_MARGIN_AUTO  => Specified(CSSMarginAuto)
        
    }
}

fn convert_net_padding(padding: css_padding_e) -> CSSValue<CSSPadding> {
    match padding {
        CSS_PADDING_INHERIT => Inherit,
        CSS_PADDING_SET(_e) => {
            let length = convert_net_unit_to_length_or_percent(_e);
            match length {
                Left(abs) => Specified(CSSPaddingLength(abs)),
                Right(percent) => Specified(CSSPaddingPercentage(percent))
            }
        }
    }
}

fn convert_net_width_value(value: css_width_e) -> CSSValue<CSSWidth> {
    match value {
        CSS_WIDTH_INHERIT => Inherit,
        CSS_WIDTH_SET(_e) => {
            let length = convert_net_unit_to_length_or_percent(_e);
            match length {
                Left(abs) => Specified(CSSWidthLength(abs)),
                Right(percent) => Specified(CSSWidthPercentage(percent))
            }
        }
        CSS_WIDTH_AUTO => Specified(CSSWidthAuto)
    }
}

fn convert_net_height_value(value: css_height_e) -> CSSValue<CSSHeight> {
    match value {
        CSS_HEIGHT_INHERIT => Inherit,
        CSS_HEIGHT_SET(_e) => {
            let length = convert_net_unit_to_length_or_percent(_e);
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
        CssFloatInherit => Inherit,
        CssFloatLeft => Specified(CSSFloatLeft),
        CssFloatRight => Specified(CSSFloatRight),
        CssFloatNone => Specified(CSSFloatNone)
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

fn convert_net_font_family_value(value: css_font_family_e) -> CSSValue<~[CSSFontFamily]> {
    use units::{Serif, SansSerif, Cursive, Fantasy, Monospace};

    match value {
        CSS_FONT_FAMILY_INHERIT => Inherit,
        CSS_FONT_FAMILY_SERIF => Specified(~[CSSFontFamilyGenericFamily(Serif)]),
        CSS_FONT_FAMILY_SANS_SERIF => Specified(~[CSSFontFamilyGenericFamily(SansSerif)]),
        CSS_FONT_FAMILY_CURSIVE => Specified(~[CSSFontFamilyGenericFamily(Cursive)]),
        CSS_FONT_FAMILY_FANTASY => Specified(~[CSSFontFamilyGenericFamily(Fantasy)]),
        CSS_FONT_FAMILY_MONOSPACE => Specified(~[CSSFontFamilyGenericFamily(Monospace)]),
        //css_font_family_e(names) => Specified(names.map(|n| CSSFontFamilyFamilyName(n.to_str()) ))
    }
}

fn convert_net_font_size_value(value: css_font_size_e) -> CSSValue<CSSFontSize> {
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
        CSS_FONT_SIZE_DIMENSION(size) => {
            match convert_net_unit_to_length_or_percent(size) {
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

fn convert_net_line_height_value(value: css_line_height_e) -> CSSValue<CSSLineHeight> {
    match value {
        CSS_LINE_HEIGHT_INHERIT => Inherit,
        CSS_LINE_HEIGHT_NUMBER(n) => Specified(CSSLineHeightNumber(css_fixed_to_float(n))),
        CSS_LINE_HEIGHT_DIMENSION(v) => {
            match convert_net_unit_to_length_or_percent(v) {
                Left(val) => Specified(CSSLineHeightLength(val)),
                Right(val) => Specified(CSSLineHeightPercentage(val))
            }
        },
        CSS_LINE_HEIGHT_NORMAL => Specified(CSSLineHeightNormal)
    }
}

fn convert_net_vertical_align_value(value: css_vertical_align_e) -> CSSValue<CSSVerticalAlign> {
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
        CSS_VERTICAL_ALIGN_SET(v) => {
            match convert_net_unit_to_length_or_percent(v) {
                Left(val) => Specified(CSSVerticalAlignLength(val)),
                Right(val) => Specified(CSSVerticalAlignPercentage(val))
            }
        }
    }
}

fn convert_net_unit_to_length(unit: css_unit) -> Length {
    match convert_net_unit_to_length_or_percent(unit) {
        Left(v) => v,
        Right(*) => fail!(~"unexpected percentage unit"),
    }
}

fn convert_net_unit_to_length_or_percent(unit: css_unit) -> Either<Length, float> {
    match unit {
        CSS_UNIT_PX(l) => Left(Px(css_fixed_to_float(l))),
        CSS_UNIT_EM(l) => Left(Em(css_fixed_to_float(l))),
        CSS_UNIT_PT(l) => Left(Px(css_fixed_to_float(l) / 72f * 96f)),
        CSS_UNIT_CM(l) => Left(Px(css_fixed_to_float(l) / 2.54f * 96f)),
        CSS_UNIT_MM(l) => Left(Px(css_fixed_to_float(l) / 25.4f * 96f)),
        CSS_UNIT_IN(l) => Left(Px(css_fixed_to_float(l) / 1f * 96f)),
        CSS_UNIT_PC(l) => Left(Px(css_fixed_to_float(l) / 6f * 96f)),
        CSS_UNIT_PCT(p) => Right(css_fixed_to_float(p)),
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
