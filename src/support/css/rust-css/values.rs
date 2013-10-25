/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/*!
The representation of CSS property values

Enums are named after the property. Variants have the same name + the
name of the value used in the spec. This leads to some verbose names,
e.g.:

The property 'background-color' and the specified value called '<color>'
in the spec lead to the variant CSSBackgroundColorColor(Color).

At least it's consistent though.
*/

use std::cmp::Eq;
use std::ptr;
use std::libc::types::common::c99::uint8_t;
use std::cast::transmute;
use srid_css::include::types::*;
use srid_css::select::common::*;
use srid_css::include::properties::*;
use srid_css::utils::errors::*;
use types::*;
use extra::url::Url;
use units::{Length, AbsoluteSize, RelativeSize};
use units::GenericFontFamily;
use color::Color;
/** A partial CSS value, before inheritance has been resolved */
#[deriving(Eq)]
pub enum CSSValue<T> {
    Inherit,
    Specified(T),
}


// CSS 2.1, Section 8 - Box model

#[deriving(Eq)]
pub enum CSSMargin {
    CSSMarginLength(Length),
    CSSMarginPercentage(float),
    CSSMarginAuto
}

#[deriving(Eq)]
pub enum CSSPadding {
    CSSPaddingLength(Length),
    CSSPaddingPercentage(float)
}

#[deriving(Eq)]
pub enum CSSBorderWidth {
    CSSBorderWidthThin,
    CSSBorderWidthMedium,
    CSSBorderWidthThick,
    CSSBorderWidthLength(Length)
}

#[deriving(Eq)]
pub enum CSSBorderColor {
    CSSBorderColorColor(Color),
    CSSBorderColorTransparent
}

#[deriving(Eq, Clone)]
pub enum CSSBorderStyle {
    CSSBorderStyleNone,
    CSSBorderStyleHidden,
    CSSBorderStyleDotted,
    CSSBorderStyleDashed,
    CSSBorderStyleSolid,
    CSSBorderStyleDouble,
    CSSBorderStyleGroove,
    CSSBorderStyleRidge,
    CSSBorderStyleInset,
    CSSBorderStyleOutset,
}

// CSS 2.1, Section 9 - Visual formatting model

#[deriving(Eq)]
pub enum CSSDisplay {
    CSSDisplayInline,
    CSSDisplayBlock,
    CSSDisplayListItem,
    CSSDisplayInlineBlock,
    CSSDisplayTable,
    CSSDisplayInlineTable,
    CSSDisplayTableRowGroup,
    CSSDisplayTableHeaderGroup,
    CSSDisplayTableFooterGroup,
    CSSDisplayTableRow,
    CSSDisplayTableColumnGroup,
    CSSDisplayTableColumn,
    CSSDisplayTableCell,
    CSSDisplayTableCaption,
    CSSDisplayNone
}

#[deriving(Eq)]
pub enum CSSPosition {
    CSSPositionStatic,
    CSSPositionRelative,
    CSSPositionAbsolute,
    CSSPositionFixed
}

#[deriving(Eq)]
pub enum CSSTop {
    CSSTopLength(Length),
    CSSTopPercentage,
    CSSTopAuto
}

#[deriving(Eq)]
pub enum CSSRight {
    CSSRightLength(Length),
    CSSRightPercentage(float),
    CSSRightAuto
}

#[deriving(Eq)]
pub enum CSSBottom {
    CSSBottomLength(Length),
    CSSBottomPercentage(float),
    CSSBottomAuto
}

#[deriving(Eq)]
pub enum CSSLeft {
    CSSLeftLength(Length),
    CSSLeftPercentage(float),
    CSSLeftAuto
}

#[deriving(Eq)]
pub enum CSSFloat {
    CSSFloatLeft,
    CSSFloatRight,
    CSSFloatNone
}

#[deriving(Eq)]
pub enum CSSClear {
    CSSClearLeft,
    CSSClearRight,
    CSSClearBoth,
    CSSClearNone
}

#[deriving(Eq)]
pub enum CSSDirection {
    CSSDirectionLtr,
    CSSDirectionRtl
}

// CSS 2.1, Section 10 - Visual formatting model details

#[deriving(Eq)]
pub enum CSSWidth {
    CSSWidthLength(Length),
    CSSWidthPercentage(float),
    CSSWidthAuto
}

#[deriving(Eq)]
pub enum CSSHeight {
    CSSHeightLength(Length),
    CSSHeightPercentage(float),
    CSSHeightAuto
}

#[deriving(Eq)]
pub enum CSSLineHeight {
    CSSLineHeightNormal,
    CSSLineHeightNumber(float),
    CSSLineHeightLength(Length),
    CSSLineHeightPercentage(float),
}

#[deriving(Eq)]
pub enum CSSVerticalAlign {
    CSSVerticalAlignBaseline,
    CSSVerticalAlignSub,
    CSSVerticalAlignSuper,
    CSSVerticalAlignTop,
    CSSVerticalAlignTextTop,
    CSSVerticalAlignMiddle,
    CSSVerticalAlignBottom,
    CSSVerticalAlignTextBottom,
    CSSVerticalAlignPercentage(float),
    CSSVerticalAlignLength(Length),
}

// CSS 2.1, Section 11 - Visual effects

#[deriving(Eq)]
pub enum CSSOverflow {
    CSSOverflowVisible,
    CSSOverflowHidden,
    CSSOverflowScroll,
    CSSOverflowAuto
}

#[deriving(Eq)]
pub enum CSSVisibility {
    CSSVisibilityVisible,
    CSSVisibilityHidden,
    CSSVisibilityCollapse
}

// CSS 2.1, Section 12 - Generated content, automatic numbering, and lists

// CSS 2.1, Section 13 - Paged media

// CSS 2.1, Section 14 - Colors and Backgrounds

#[deriving(Eq)]
pub enum CSSColor {
    CSSColorColor(Color)
}

#[deriving(Eq)]
pub enum CSSBackgroundColor {
    CSSBackgroundColorColor(Color),
    CSSBackgroundColorTransparent
}

#[deriving(Eq)]
pub enum CSSBackgroundImage {
    CSSBackgroundUri(Url),
    CSSBackgroundImageNone
}

#[deriving(Eq)]
pub enum CSSBackgroundRepeat {
    CSSBackgroundRepeatRepeat,
    CSSBackgroundRepeatRepeatX,
    CSSBackgroundRepeatRepeatY,
    CSSBackgroundRepeatNoRepeat
}

#[deriving(Eq)]
pub enum CSSBackgroundAttachment {
    CSSBackgroundAttachmentScroll,
    CSSBackgroundAttachmentFixed
}

#[deriving(Eq)]
pub enum CSSBackgroundPosition {
    CSSBackgroundPositionPercentage(float),
    CSSBackgroundPositionLength(Length),
    CSSBackgroundPositionLeft,
    CSSBackgroundPositionCenter,
    CSSBackgroundPositionRight,
    CSSBackgroundPositionTop,
    CSSBackgroundPositionBottom
}

// CSS 2.1, Section 15 - Fonts

#[deriving(Eq)]
pub enum CSSFontFamily {
    CSSFontFamilyFamilyName(~str),
    CSSFontFamilyGenericFamily(GenericFontFamily)
}

#[deriving(Eq)]
pub enum CSSFontStyle {
    CSSFontStyleNormal,
    CSSFontStyleItalic,
    CSSFontStyleOblique
}

#[deriving(Eq)]
pub enum CSSFontWeight {
    CSSFontWeightNormal,
    CSSFontWeightBold,
    CSSFontWeightBolder,
    CSSFontWeightLighter,
    CSSFontWeight100,
    CSSFontWeight200,
    CSSFontWeight300,
    CSSFontWeight400,
    CSSFontWeight500,
    CSSFontWeight600,
    CSSFontWeight700,
    CSSFontWeight800,
    CSSFontWeight900
}

#[deriving(Eq)]
pub enum CSSFontSize {
    CSSFontSizeAbsoluteSize(AbsoluteSize),
    CSSFontSizeRelativeSize(RelativeSize),
    CSSFontSizeLength(Length),
    CSSFontSizePercentage(float)
}

// CSS 2.1, Section 16 - Text

#[deriving(Eq)]
pub enum CSSTextAlign {
    CSSTextAlignLeft,
    CSSTextAlignRight,
    CSSTextAlignCenter,
    CSSTextAlignJustify
}

#[deriving(Eq, Clone)]
pub enum CSSTextDecoration {
    CSSTextDecorationNone,
    CSSTextDecorationUnderline,
    CSSTextDecorationOverline,
    CSSTextDecorationLineThrough,
    CSSTextDecorationBlink
}

#[deriving(Eq)]
pub enum CSSTextTransform {
    CSSTextTransformCapitalize,
    CSSTextTransformUppercase,
    CSSTextTransformLowercase,
    CSSTextTransformNone
}

// CSS 2.1, Section 17 - Tables

// CSS 2.1, Section 18 - User interface

pub enum CssHint {
    // CssHintFontFamily(~[LwcString], CssFontFamily),
    CssHintLength(css_unit),
    CssHintDefault,
    CssHintUnknown
}

impl CssHint {

    pub fn new(property: css_properties_e, hint: *css_hint) -> CssHint {
        let status = get_css_hint_status(hint) as u32;
        match property {
            CssPropFontSize => {
                let length: &css_hint_length = hint_imm_data_field(hint);
                if status == CSS_FONT_SIZE_DIMENSION {
                    CssHintLength(length.unit, length.value)
                } else {
                    CssHintUnknown
                }
            }
            _ => fail!(fmt!("unknown css hint: %?", property))
        }
    }
    
    pub fn write_to_ll(&self, property: css_properties_e, llhint: *mut css_hint) -> css_error {
        match (property, self) {
            (CssPropFontFamily, &CssHintDefault) => {
                let strings: &mut ~str = hint_data_field(llhint);
                *strings = ~"";
                set_css_hint_status(llhint, CSS_FONT_FAMILY_SANS_SERIF as uint8_t);
            }
            (CssPropQuotes, &CssHintDefault) => {
                let strings: &mut ~str = hint_data_field(llhint);
                *strings = ~"";
                set_css_hint_status(llhint, CSS_QUOTES_STRING_OR_NONE as uint8_t);
            }
            (CssPropColor, &CssHintDefault) => {
                let color: &mut css_color = hint_data_field(llhint);
                *color = CssColor { a: 255, r: 0, g: 0, b: 0 }.to_ll();
                set_css_hint_status(llhint, CSS_COLOR_COLOR as uint8_t);
            }
            (CssPropFontSize, &CssHintLength(val)) => {
                let length: &mut css_hint_length = hint_data_field(llhint);
                *length = val.to_ll_css_hint_length();
                set_css_hint_status(llhint, CSS_FONT_SIZE_DIMENSION as uint8_t);
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

fn get_css_hint_status(llhint: *css_hint) -> uint8_t {
    unsafe {
        let llhint_bytes: *mut uint8_t = transmute(llhint);
        let status_field: *mut uint8_t = ptr::mut_offset(llhint_bytes, status_field_offset());
        *status_field
    }
}

fn set_css_hint_status(llhint: *mut css_hint, status: uint8_t) {
    // So gnarly. The status field is a uint8_t that comes after a union type.
    // We're just going to calculate it's address and write it
    unsafe {
        let llhint_bytes: *mut uint8_t = transmute(llhint);
        let status_field: *mut uint8_t = ptr::mut_offset(llhint_bytes, status_field_offset());
        *status_field = status;
    }
}

#[cfg(target_arch = "x86_64")]
#[cfg(target_arch = "x86")]
#[cfg(target_arch = "arm")]
fn status_field_offset() -> int { 16 }

fn hint_data_field<T>(llhint: *mut css_hint) -> &mut T {
    unsafe { transmute(llhint) }
}

fn hint_imm_data_field<T>(llhint: *css_hint) -> &T {
    unsafe { transmute(llhint) }
}


