/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
extern mod srid_css;

use color::{Color, rgba};
use units::{Length, Px, Em};
use std::either::{Either, Left, Right};
use helpers::computed::CssComputedStyle;
use helpers::values::*;
use helpers::types::*;
use values::*;
use srid_css::libwapcaplet::wapcaplet::*;

pub type css_fixed = i32;

#[inline]
pub fn css_fixed_to_float(f: css_fixed) -> float {
    static BEFORE: i32 = 10;
    f as float * 1.0f / ((1i32 << BEFORE) as float)
}

pub struct ComputedStyle<'self>{
    inner: CssComputedStyle<'self>
}

impl<'self> ComputedStyle<'self> {

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
        println("COMING IN FONT_FAMI");
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

fn convert_net_color(color: CssColor) -> Color {
    rgba(color.r, color.g, color.b, (color.a as float) / 255.0)
}

fn convert_net_color_value(color: CssColorValue) -> CSSValue<Color> {
    match color {
        CssColorInherit => Inherit,
        CssColorColor(v) => Specified(convert_net_color(v))
    }
}

fn convert_net_border_style(style: CssBorderStyleValue) -> CSSValue<CSSBorderStyle> {
    match style {
        CssBorderStyleInherit => Inherit,
        CssBorderStyleNone => Specified(CSSBorderStyleNone),
        CssBorderStyleHidden => Specified(CSSBorderStyleHidden),
        CssBorderStyleDotted => Specified(CSSBorderStyleDotted),
        CssBorderStyleDashed => Specified(CSSBorderStyleDashed),
        CssBorderStyleSolid => Specified(CSSBorderStyleSolid),
        CssBorderStyleDouble => Specified(CSSBorderStyleDouble),
        CssBorderStyleGroove => Specified(CSSBorderStyleGroove),
        CssBorderStyleRidge => Specified(CSSBorderStyleRidge),
        CssBorderStyleInset => Specified(CSSBorderStyleInset),
        CssBorderStyleOutset => Specified(CSSBorderStyleOutset),
    }
}

fn convert_net_border_width(width: CssBorderWidthValue) -> CSSValue<CSSBorderWidth> {
    match width {
        CssBorderWidthInherit => Inherit,
        CssBorderWidthThin => Specified(CSSBorderWidthThin),
        CssBorderWidthMedium => Specified(CSSBorderWidthMedium),
        CssBorderWidthThick => Specified(CSSBorderWidthThick),
        CssBorderWidthWidth(width) => Specified(CSSBorderWidthLength(convert_net_unit_to_length(width))),
    }
}

fn convert_net_margin(margin: CssMarginValue) -> CSSValue<CSSMargin> {
    match margin {
        CssMarginInherit => Inherit,
        CssMarginSet(value) => {
            let length = convert_net_unit_to_length_or_percent(value);
            match length {
                Left(abs) => Specified(CSSMarginLength(abs)),
                Right(percent) => Specified(CSSMarginPercentage(percent))
            }
        }
        CssMarginAuto => Specified(CSSMarginAuto)
    }
}

fn convert_net_padding(padding: CssPaddingValue) -> CSSValue<CSSPadding> {
    match padding {
        CssPaddingInherit => Inherit,
        CssPaddingSet(value) => {
            let length = convert_net_unit_to_length_or_percent(value);
            match length {
                Left(abs) => Specified(CSSPaddingLength(abs)),
                Right(percent) => Specified(CSSPaddingPercentage(percent))
            }
        }
    }
}

fn convert_net_width_value(value: CssWidthValue) -> CSSValue<CSSWidth> {
    match value {
        CssWidthInherit => Inherit,
        CssWidthSet(value) => {
            let length = convert_net_unit_to_length_or_percent(value);
            match length {
                Left(abs) => Specified(CSSWidthLength(abs)),
                Right(percent) => Specified(CSSWidthPercentage(percent))
            }
        }
        CssWidthAuto => Specified(CSSWidthAuto)
    }
}

fn convert_net_height_value(value: CssHeightValue) -> CSSValue<CSSHeight> {
    match value {
        CssHeightInherit => Inherit,
        CssHeightSet(value) => {
            let length = convert_net_unit_to_length_or_percent(value);
            match length {
                Left(abs) => Specified(CSSHeightLength(abs)),
                Right(percent) => Specified(CSSHeightPercentage(percent))
            }
        }
        CssHeightAuto => Specified(CSSHeightAuto)
    }
}

fn convert_net_display_value(value: CssDisplayValue) -> CSSValue<CSSDisplay> {
    match value {
        CssDisplayInherit => Inherit,
        CssDisplayInline => Specified(CSSDisplayInline),
        CssDisplayBlock => Specified(CSSDisplayBlock),
        CssDisplayListItem => Specified(CSSDisplayListItem),
        CssDisplayRunIn => unimpl("display: run-in"), // FIXME: Not in CSS 2.1
        CssDisplayInlineBlock => Specified(CSSDisplayInlineBlock),
        CssDisplayTable => Specified(CSSDisplayTable),
        CssDisplayInlineTable => Specified(CSSDisplayInlineTable),
        CssDisplayTableRowGroup => Specified(CSSDisplayTableRowGroup),
        CssDisplayTableHeaderGroup => Specified(CSSDisplayTableHeaderGroup),
        CssDisplayTableFooterGroup => Specified(CSSDisplayTableFooterGroup),
        CssDisplayTableRow => Specified(CSSDisplayTableRow),
        CssDisplayTableColumnGroup => Specified(CSSDisplayTableColumnGroup),
        CssDisplayTableColumn => Specified(CSSDisplayTableColumn),
        CssDisplayTableCell => Specified(CSSDisplayTableCell),
        CssDisplayTableCaption => Specified(CSSDisplayTableCaption),
        CssDisplayNone => Specified(CSSDisplayNone)
    }
}

fn convert_net_float_value(value: CssFloatValue) -> CSSValue<CSSFloat> {
    match value {
        CssFloatInherit => Inherit,
        CssFloatLeft => Specified(CSSFloatLeft),
        CssFloatRight => Specified(CSSFloatRight),
        CssFloatNone => Specified(CSSFloatNone)
    }
}

fn convert_net_clear_value(value: CssClearValue) -> CSSValue<CSSClear> {
    match value {
        CssClearInherit => Inherit,
        CssClearNone => Specified(CSSClearNone),
        CssClearLeft => Specified(CSSClearLeft),
        CssClearRight => Specified(CSSClearRight),
        CssClearBoth => Specified(CSSClearBoth)
    }
}

fn convert_net_position_value(value: CssPositionValue) -> CSSValue<CSSPosition> {
    match value {
        CssPositionInherit => Inherit,
        CssPositionStatic => Specified(CSSPositionStatic),
        CssPositionRelative => Specified(CSSPositionRelative),
        CssPositionAbsolute => Specified(CSSPositionAbsolute),
        CssPositionFixed => Specified(CSSPositionFixed)
    }
}

fn convert_net_font_family_value(value: CssFontFamilyValue) -> CSSValue<~[CSSFontFamily]> {
    use units::{Serif, SansSerif, Cursive, Fantasy, Monospace};
    println("convert_net_font_family_value");

    match value {
        CssFontFamilyInherit => {
            println("convert_net_font_family_value :: COMING IN INHERIT");
            Inherit},
        CssFontFamilySerif => Specified(~[CSSFontFamilyGenericFamily(Serif)]),
        CssFontFamilySansSerif => Specified(~[CSSFontFamilyGenericFamily(SansSerif)]),
        CssFontFamilyCursive => Specified(~[CSSFontFamilyGenericFamily(Cursive)]),
        CssFontFamilyFantasy => Specified(~[CSSFontFamilyGenericFamily(Fantasy)]),
        CssFontFamilyMonospace => Specified(~[CSSFontFamilyGenericFamily(Monospace)]),
        CssFontFamilyValue(names) => 
        {println("convert_net_font_family_value :: in value");
            Specified(names.map(|&n| CSSFontFamilyFamilyName(unsafe{lwc_ref.get_mut_ref()}.lwc_string_data(n)) ))
        }
    }
}

fn convert_net_font_size_value(value: CssFontSizeValue) -> CSSValue<CSSFontSize> {
    use units::*;

    match value {
        CssFontSizeInherit => Inherit,
        CssFontSizeXXSmall => Specified(CSSFontSizeAbsoluteSize(XXSmall)),
        CssFontSizeXSmall => Specified(CSSFontSizeAbsoluteSize(XSmall)),
        CssFontSizeSmall => Specified(CSSFontSizeAbsoluteSize(Small)),
        CssFontSizeMedium => Specified(CSSFontSizeAbsoluteSize(Medium)),
        CssFontSizeLarge => Specified(CSSFontSizeAbsoluteSize(Large)),
        CssFontSizeXLarge => Specified(CSSFontSizeAbsoluteSize(XLarge)),
        CssFontSizeXXLarge => Specified(CSSFontSizeAbsoluteSize(XXLarge)),
        CssFontSizeLarger => Specified(CSSFontSizeRelativeSize(Larger)),
        CssFontSizeSmaller => Specified(CSSFontSizeRelativeSize(Smaller)),
        CssFontSizeDimension(size) => {
            match convert_net_unit_to_length_or_percent(size) {
                Left(val) => Specified(CSSFontSizeLength(val)),
                Right(val) => Specified(CSSFontSizePercentage(val))
            }
        }
    }
}

fn convert_net_font_style_value(value: CssFontStyleValue) -> CSSValue<CSSFontStyle> {
    match value {
        CssFontStyleInherit => Inherit,
        CssFontStyleNormal => Specified(CSSFontStyleNormal),
        CssFontStyleItalic => Specified(CSSFontStyleItalic),
        CssFontStyleOblique => Specified(CSSFontStyleOblique)
    }
}

fn convert_net_font_weight_value(value: CssFontWeightValue) -> CSSValue<CSSFontWeight> {
    match value {
        CssFontWeightNormal => Specified(CSSFontWeightNormal),
        CssFontWeightInherit => Inherit,
        CssFontWeightBold => Specified(CSSFontWeightBold),
        CssFontWeightBolder => Specified(CSSFontWeightBolder),
        CssFontWeightLighter => Specified(CSSFontWeightLighter),
        CssFontWeight100 => Specified(CSSFontWeight100),
        CssFontWeight200 => Specified(CSSFontWeight200),
        CssFontWeight300 => Specified(CSSFontWeight300),
        CssFontWeight400 => Specified(CSSFontWeight400),
        CssFontWeight500 => Specified(CSSFontWeight500),
        CssFontWeight600 => Specified(CSSFontWeight600),
        CssFontWeight700 => Specified(CSSFontWeight700),
        CssFontWeight800 => Specified(CSSFontWeight800),
        CssFontWeight900 => Specified(CSSFontWeight900),
    }
}

fn convert_net_text_align_value(value: CssTextAlignValue) -> CSSValue<CSSTextAlign> {
    match value {
        CssTextAlignInherit => Inherit,
        CssTextAlignInheritIfNonMagic => unimpl("inherit if non-magic? wtf?"),
        CssTextAlignLeft => Specified(CSSTextAlignLeft),
        CssTextAlignRight => Specified(CSSTextAlignRight),
        CssTextAlignCenter => Specified(CSSTextAlignCenter),
        CssTextAlignJustify => Specified(CSSTextAlignJustify),
        CssTextAlignDefault => Specified(CSSTextAlignLeft),
        CssTextAlignLibcssLeft => unimpl("text-align libcss left"),
        CssTextAlignLibcssCenter => unimpl("text-align libcss center"),
        CssTextAlignLibcssRight => unimpl("text-align libcss right"),
    }
}

fn convert_net_text_decoration_value(value: CssTextDecorationValue) -> CSSValue<CSSTextDecoration> {
    match value {
        CssTextDecorationInherit => Inherit,
        CssTextDecorationNone => Specified(CSSTextDecorationNone),
        CssTextDecorationBlink => Specified(CSSTextDecorationBlink),
        CssTextDecorationLineThrough => Specified(CSSTextDecorationLineThrough),
        CssTextDecorationOverline => Specified(CSSTextDecorationOverline),
        CssTextDecorationUnderline => Specified(CSSTextDecorationUnderline),
    }
}

fn convert_net_line_height_value(value: CssLineHeightValue) -> CSSValue<CSSLineHeight> {
    match value {
        CssLineHeightInherit => Inherit,
        CssLineHeightNumber(n) => Specified(CSSLineHeightNumber(css_fixed_to_float(n))),
        CssLineHeightDimension(v) => {
            match convert_net_unit_to_length_or_percent(v) {
                Left(val) => Specified(CSSLineHeightLength(val)),
                Right(val) => Specified(CSSLineHeightPercentage(val))
            }
        },
        CssLineHeightNormal => Specified(CSSLineHeightNormal)
    }
}

fn convert_net_vertical_align_value(value: CssVerticalAlignValue) -> CSSValue<CSSVerticalAlign> {
    match value {
        CssVerticalAlignInherit => Inherit,
        CssVerticalAlignBaseline => Specified(CSSVerticalAlignBaseline),
        CssVerticalAlignSub => Specified(CSSVerticalAlignSub),
        CssVerticalAlignSuper => Specified(CSSVerticalAlignSuper),
        CssVerticalAlignTop => Specified(CSSVerticalAlignTop),
        CssVerticalAlignTextTop => Specified(CSSVerticalAlignTextTop),
        CssVerticalAlignMiddle => Specified(CSSVerticalAlignMiddle),
        CssVerticalAlignBottom => Specified(CSSVerticalAlignBottom),
        CssVerticalAlignTextBottom => Specified(CSSVerticalAlignTextBottom),
        CssVerticalAlignDimension(v) => {
            match convert_net_unit_to_length_or_percent(v) {
                Left(val) => Specified(CSSVerticalAlignLength(val)),
                Right(val) => Specified(CSSVerticalAlignPercentage(val))
            }
        }
    }
}

fn convert_net_unit_to_length(unit: CssUnit) -> Length {
    match convert_net_unit_to_length_or_percent(unit) {
        Left(v) => v,
        Right(*) => fail!(~"unexpected percentage unit"),
    }
}

fn convert_net_unit_to_length_or_percent(unit: CssUnit) -> Either<Length, float> {
    match unit {
        CssUnitPx(l) => Left(Px(css_fixed_to_float(l))),
        CssUnitEm(l) => Left(Em(css_fixed_to_float(l))),
        CssUnitPt(l) => Left(Px(css_fixed_to_float(l) / 72f * 96f)),
        CssUnitCm(l) => Left(Px(css_fixed_to_float(l) / 2.54f * 96f)),
        CssUnitMm(l) => Left(Px(css_fixed_to_float(l) / 25.4f * 96f)),
        CssUnitIn(l) => Left(Px(css_fixed_to_float(l) / 1f * 96f)),
        CssUnitPc(l) => Left(Px(css_fixed_to_float(l) / 6f * 96f)),
        CssUnitPct(p) => Right(css_fixed_to_float(p)),
        _ => unimpl("unit")
    }
}

fn unimpl(what: &str) -> ! {
    fail!(fmt!("css unimplemented %?", what))
}
