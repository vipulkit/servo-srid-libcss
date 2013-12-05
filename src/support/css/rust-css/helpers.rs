use srid_css::include::types::*;
use srid_css::utils::errors::*;
use srid_css::stylesheet::*;
use std::libc::*;
use srid_css::libwapcaplet::wapcaplet::*;

pub trait VoidPtrLike {
    fn from_void_ptr(ptr: *c_void) -> Self;
    fn to_void_ptr(&self) -> *c_void;
}

pub fn require_ok(code: css_error, what: &str) {
    match code {
        e if e as uint == CSS_OK as uint => (),
        e => fail!(fmt!("CSS parsing failed while %s. code: %?", what, e))
    }
}

pub fn ll_qname_to_hl_qname(qname: &css_qname) -> types::CssQName {
    types::CssQName {
        ns: Some(unsafe{lwc_ref.get_mut_ref()}.lwc_string_data(qname.ns)),
        name: unsafe{lwc_ref.get_mut_ref()}.lwc_string_data(qname.name)
    }
}

pub fn write_ll_qname(hlqname: &mut types::CssQName, llqname: &mut css_qname) {
    match &hlqname.ns {
        &Some(ref ns) => {
            llqname.ns = unsafe{lwc_ref.get_mut_ref()}.lwc_intern_string(ns.clone());
        }
        _ => ()
    }
    llqname.name = unsafe{lwc_ref.get_mut_ref()}.lwc_intern_string(hlqname.name);
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

    use helpers::properties::{CssProperty,
                     CssPropFontSize,
                     CssPropFontFamily,
                     CssPropQuotes,
                     CssPropColor,
                     CssFontFamily};
    use helpers::types::*;                 
    use srid_css::utils::errors::*;
    // use srid_css::include::types::css_color;
    //use std::ptr::null;
    //use std::libc::types::common::c99::uint8_t;
    use std::cast::transmute;
    //use std::ptr;
    use srid_css::select::common::*;
    use srid_css::include::properties::*;
    use helpers::*;

    // An interpretation of the delightful css_hint union
    pub enum CssHint {
        CssHintFontFamily(~[uint], CssFontFamily),
        CssHintLength(CssUnit),
        CssHintDefault,
        CssHintUnknown
    }

    impl CssHint {

        pub fn new(property: CssProperty, hint: Option<&~css_hint>) -> CssHint {
            // println(fmt!("new hint == %?" , hint));
            let status = hint.get_ref().status as u32;
            match property {
                CssPropFontSize => {
                    if status == CSS_FONT_SIZE_DIMENSION as u32 {
                        let length: &~css_hint_length = hint.get_ref().length.get_ref();
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
                    llhint.hint_type = COLOR;
                    llhint.color = unsafe {transmute(CssColor { a: 255, r: 0, g: 0, b: 0 })};
                    llhint.status = CSS_COLOR_COLOR as u8;
                }
                (CssPropFontSize, &CssHintLength(val)) => {
                    llhint.hint_type = HINT_LENGTH;
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

pub mod types {
    //use wapcaplet::uint;
    use std::cast::transmute;
    use srid_css::include::types::*;
    use srid_css::stylesheet::css_fixed;
    use srid_css::select::common::css_hint_length;
    use helpers::ToLl;
    pub type c_enum = u8;
    pub type rust_enum = uint;
    
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
        ns: Option<~str>,
        name: ~str
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

    pub fn ll_color_to_hl_color(color: css_color) -> CssColor {
        unsafe { transmute(color) }
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
        use helpers::types::*;

        match *self {
            CssUnitPx(value) => (CSS_UNIT_PX, value),
            CssUnitEx(value) => (CSS_UNIT_EX, value),
            CssUnitEm(value) => (CSS_UNIT_EM, value),
            CssUnitIn(value) => (CSS_UNIT_IN, value),
            CssUnitCm(value) => (CSS_UNIT_CM, value),
            CssUnitMm(value) => (CSS_UNIT_MM, value),
            CssUnitPt(value) => (CSS_UNIT_PT, value),
            CssUnitPc(value) => (CSS_UNIT_PC, value),
            CssUnitPct(value) => (CSS_UNIT_PCT, value),
            CssUnitDeg(value) => (CSS_UNIT_DEG, value),
            CssUnitGrad(value) => (CSS_UNIT_GRAD, value),
            CssUnitRad(value) => (CSS_UNIT_RAD, value),
            CssUnitMs(value) => (CSS_UNIT_MS, value),
            CssUnitS(value) => (CSS_UNIT_S, value),
            CssUnitHz(value) => (CSS_UNIT_HZ, value),
            CssUnitKHz(value) => (CSS_UNIT_KHZ, value)
        }
    }
}

pub fn ll_unit_to_hl_unit(unit: css_unit, value: css_fixed) -> types::CssUnit {
    use helpers::types::*;
    use srid_css::include::types::*;
    if unit  as uint == CSS_UNIT_PX as uint {
        CssUnitPx(value)
    } else if unit as uint == CSS_UNIT_EX as uint {
        CssUnitEx(value)
    } else if unit as uint == CSS_UNIT_EM as uint {
        CssUnitEm(value)
    } else if unit as uint == CSS_UNIT_IN as uint {
        CssUnitIn(value)
    } else if unit as uint == CSS_UNIT_CM as uint {
        CssUnitCm(value)
    } else if unit as uint == CSS_UNIT_MM as uint {
        CssUnitMm(value)
    } else if  unit as uint == CSS_UNIT_PT  as uint{
        CssUnitPt(value)
    } else if unit as uint == CSS_UNIT_PC as uint {
        CssUnitPc(value)
    } else if unit as uint == CSS_UNIT_PCT as uint {
        CssUnitPct(value)
    } else if unit as uint == CSS_UNIT_DEG as uint {
        CssUnitDeg(value)
    } else if unit as uint == CSS_UNIT_GRAD as uint {
        CssUnitGrad(value)
    } else if unit as uint == CSS_UNIT_RAD as uint {
        CssUnitRad(value)
    } else if unit as uint == CSS_UNIT_MS  as uint{
        CssUnitMs(value)
    } else if unit as uint == CSS_UNIT_S as uint {
        CssUnitS(value)
    } else if unit  as uint== CSS_UNIT_HZ as uint {
        CssUnitHz(value)
    } else if unit as uint == CSS_UNIT_KHZ as uint {
        CssUnitKHz(value)
    } else {
        fail!()
    }
}

pub mod select {

    // use std::libc;
    use std::libc::c_void;
    use std::libc::types::common::c99::{uint32_t};
    // use std::vec;
    // use std::sys;
    // use std::ptr;
    use srid_css::include::types::css_origin;
    use helpers::types::CssQName;
    use srid_css::stylesheet::css_qname;
    use std::ptr::{null, /*to_mut_unsafe_ptr,*/ to_unsafe_ptr};
    use std::cast::transmute;
    use srid_css::utils::errors::{css_error, CSS_OK};
    use srid_css::select::select::css_select_ctx;
    use helpers::{/*ToLl,*/ write_ll_qname, ll_qname_to_hl_qname, /*require_ok,*/ VoidPtrLike};
    use helpers::properties::CssProperty;
    use helpers::hint::CssHint;
    use srid_css::libwapcaplet::wapcaplet::*;
    use srid_css::select::common::*;
    use srid_css::select::dispatch::*;
    use extra::time;
    // use dump_computed::*;

    pub enum CssPseudoElement {
    CssPseudoElementNone         = 0,
    CssPseudoElementFirstLine   = 1,
    CssPseudoElementFirstLetter = 2,
    CssPseudoElementBefore       = 3,
    CssPseudoElementAfter        = 4,
    CssPseudoElementCount   = 5
    }

    pub struct CssSelectCtx {
        priv select_ctx:~css_select_ctx,
        // Whenever a sheet is added to the select ctx we will take ownership of it
        // to ensure that it stays alive
        priv sheets: ~[uint],
    }



    #[fixed_stack_segment]
    pub fn css_select_ctx_create() -> CssSelectCtx {
        CssSelectCtx {
            select_ctx: css_select_ctx::css_select_ctx_create(),
            sheets: ~[]
        }
    }

    impl CssSelectCtx {
        #[fixed_stack_segment]
        pub fn append_sheet(&mut self, sheet: uint, origin: css_origin, media: u64) {
            self.select_ctx.css_select_ctx_append_sheet(sheet, origin, media);   
            self.sheets.push(sheet);         
        }

        #[fixed_stack_segment]
        pub fn count_sheets(&mut self) -> uint {
            self.select_ctx.css_select_ctx_count_sheets()            
        }

        #[fixed_stack_segment]
        pub fn select_style<N: VoidPtrLike, H: CssSelectHandler<N>>(&mut self, node: &N, media: u64,
                                                            inline_style: Option<uint>,
                                                            handler: &H , total_time: &mut u64) -> CssSelectResults {

            let lwc_ref = unsafe {lwc_ref.get_mut_ref()};
            
            do with_untyped_handler(handler) |untyped_handler| {
                let raw_handler = build_raw_handler();
                
                let pw:*c_void = unsafe {transmute(to_unsafe_ptr(untyped_handler))};
                
                let mut node_name = css_qname{ 
                    name:lwc_ref.lwc_intern_string("") , 
                    ns:lwc_ref.lwc_intern_string("")
                };
                ((raw_handler.node_name))(pw , node.to_void_ptr(), &mut node_name);

                let mut node_id : uint = 0;
                ((raw_handler.node_id))(lwc_ref , pw , node.to_void_ptr(), &mut node_id);

                let mut classes: ~[uint] = ~[];
                ((raw_handler.node_classes))(lwc_ref, pw, node.to_void_ptr(), 
                        &mut classes);

                let start_time = time::precise_time_ns();
                let (error, results) = self.select_ctx.css_select_style(node.to_void_ptr(),
                                media,
                                inline_style,
                                raw_handler,
                                pw  , node_name , node_id , classes);
                if error as uint != CSS_OK as uint {
                     println(fmt!("%?", error));
                     //fail!("Error in Select Style")
                }
                let end_time = time::precise_time_ns();
                
                *total_time += (end_time - start_time);  
                
                let result_unwrap = if results.is_none() {
                        ~css_select_results{ 
                                        styles:~[Some(css_computed_style_inline_create()),Some(css_computed_style_inline_create()),Some(css_computed_style_inline_create()),Some(css_computed_style_inline_create()),Some(css_computed_style_inline_create())] 
                        }
                    } else {
                        results.unwrap()
                };    
                //unsafe {
                //dump_computed_style((result_unwrap.styles[CSS_PSEUDO_ELEMENT_NONE as uint].get_mut_ref()), lwc_ref.get_mut_ref(), &mut result_string);        
                //}
                //println(fmt!("\n=================================================================="));
                //println(fmt!("\n== Result is ::====%s====",result_string));
                //println(fmt!("\n=================================================================="));


                CssSelectResults {
                    results: result_unwrap
                }

            }
        }
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

    mod raw_handler {
        //use std::libc::types::common::c99::{uint32_t, int32_t};
        use srid_css::stylesheet::css_qname;
        use std::libc::c_void;
        use std::cast::transmute;
        use srid_css::utils::errors::{css_error, CSS_OK, CSS_PROPERTY_NOT_SET};
        use srid_css::select::common::css_hint;
        use helpers::types;
        use helpers::hint;
        use helpers::properties::*;
        use helpers::hint::CssHint;
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
        pub fn compute_font_size(/*_pw: *c_void ,*/ parent: Option<&~css_hint>, size: &mut ~css_hint) -> css_error {
            enter("compute_font_size");
            // FIXME: This should be merged with the one in rust-css, I think. --pcwalton
            let parent_hint;
            if parent.is_none() {
                parent_hint = hint::CssHintLength(types::CssUnitPx(16 * 1024));
            } else {
                parent_hint = CssHint::new(CssPropFontSize, parent);
            }
            parent_hint.write_to_ll(CssPropFontSize, size);
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
        ua_default_for_property: &'self fn(property: uint32_t, hint: &mut ~css_hint) -> css_error,
    }

    fn with_untyped_handler<N: VoidPtrLike, H: CssSelectHandler<N>, R>(handler: &H, f: &fn(&UntypedHandler) -> R) -> R {
        unsafe {
            let untyped_handler = UntypedHandler {
                node_name: |node: *c_void, qname: &mut css_qname| -> css_error {
                    let hlnode: N = VoidPtrLike::from_void_ptr(node);
                    let mut hlqname = handler.node_name(&hlnode);
                    write_ll_qname(&mut hlqname, qname);
                    CSS_OK
                },
                node_classes: |node: *c_void, classes: &mut ~[uint]| -> css_error {
                    let hlnode: N = VoidPtrLike::from_void_ptr(node);
                    
                    *classes = match handler.node_classes(&hlnode) {
                        Some(classes) => {
                            classes
                        },
                        None => ~[]
                    };
                    
                    CSS_OK
                },
                node_id: | node: *c_void, id: &mut uint| -> css_error {
                    let hlnode: N = VoidPtrLike::from_void_ptr(node);
                    *id = match handler.node_id(&hlnode) {
                        Some(id) => id,
                        None => 0
                    };
                    CSS_OK
                },
                named_parent_node: |node: *c_void,  qname: &mut css_qname, parent: *mut *c_void| -> css_error {
                    let hlnode: N = VoidPtrLike::from_void_ptr(node);
                    let mut hlqname = ll_qname_to_hl_qname(qname);
                    *parent = match handler.named_parent_node(&hlnode, &mut hlqname) {
                        Some(p) => p.to_void_ptr(),
                        None => null()
                    };
                    CSS_OK
                },
                parent_node: |node: *c_void, parent: *mut *c_void| -> css_error {
                    let hlnode: N = VoidPtrLike::from_void_ptr(node);
                    let hlparent: Option<N> = handler.parent_node(&hlnode);
                    *parent = match hlparent {
                        Some(ref p) => p.to_void_ptr(),
                        None => null()
                    };
                    CSS_OK
                },
                node_has_class: | node: *c_void, name: uint, match_: &mut bool| -> css_error {
                    let hlnode: N = VoidPtrLike::from_void_ptr(node);
                    let hlname = name; //from_lwc_string
                    *match_ = handler.node_has_class(&hlnode, lwc_ref.get_mut_ref().lwc_string_data(hlname));
                    CSS_OK
                },
                node_has_id: | node: *c_void, name: uint, match_: &mut bool| -> css_error {
                    let hlnode: N = VoidPtrLike::from_void_ptr(node);
                    let hlname = name; //from_lwc_string
                    *match_ = handler.node_has_id(&hlnode, lwc_ref.get_mut_ref().lwc_string_data(hlname));
                    CSS_OK
                },
                named_ancestor_node: |node: *c_void,
                                      qname: &mut css_qname,
                                      parent: *mut *c_void| -> css_error {
                    let hlnode: N = VoidPtrLike::from_void_ptr(node);
                    let mut hlqname = ll_qname_to_hl_qname(qname);
                    *parent = match handler.named_ancestor_node(&hlnode, &mut hlqname) {
                        Some(p) => p.to_void_ptr(),
                        None => null()
                    };
                    CSS_OK
                },
                node_is_root: |node: *c_void, match_: &mut bool| -> css_error {
                    let hlnode = VoidPtrLike::from_void_ptr(node);
                    *match_ = handler.node_is_root(&hlnode);
                    CSS_OK
                },
                node_is_link: |node: *c_void, match_: &mut bool| -> css_error {
                    let hlnode = VoidPtrLike::from_void_ptr(node);
                    *match_ = handler.node_is_link(&hlnode);
                    CSS_OK
                },
                node_is_visited: |node: *c_void, match_: &mut bool| -> css_error {
                    let hlnode = VoidPtrLike::from_void_ptr(node);
                    *match_ = handler.node_is_visited(&hlnode);
                    CSS_OK
                },

                ua_default_for_property: |property: u32, hint: &mut ~css_hint| -> css_error {
                    use helpers::properties::property_from_uint;
                    let hlproperty = property_from_uint(property);
                    let hlhint = handler.ua_default_for_property(hlproperty);
                    hlhint.write_to_ll(hlproperty, hint)
                },
            };

            f(&untyped_handler)
        }
    }

    pub trait CssSelectHandler<N> {
        fn node_name(&self, node: &N) -> CssQName;
        fn node_classes(&self, node: &N) -> Option<~[uint]>;
        fn node_id(&self, node: &N) -> Option<uint>;
        fn named_parent_node(&self, node: &N, qname: &mut CssQName) -> Option<N>;
        fn parent_node(&self, node: &N) -> Option<N>;
        fn node_has_class(&self, node: &N, name: &str) -> bool;
        fn node_has_id(&self, node: &N, name: &str) -> bool;
        fn named_ancestor_node(&self, node: &N, qname: &mut CssQName) -> Option<N>;
        fn node_is_root(&self, node: &N) -> bool;
        fn node_is_link(&self, node: &N) -> bool;
        fn node_is_visited(&self, node: &N) -> bool;
        fn ua_default_for_property(&self, property: CssProperty) -> CssHint;
    }

    //#[deriving(DeepClone)]
    pub struct CssSelectResults {
        results:~css_select_results,
    }

    // impl Drop for CssSelectResults {
    //     #[fixed_stack_segment]
    //     fn drop(&self) {
    //         assert!(self.results.is_not_null());
    //         let code = unsafe { css_select_results_destroy(self.results) };
    //         require_ok(code, "destroying select results");
    //     }
    // }

    impl<'self> CssSelectResults {
        pub fn computed_style(&'self self, element: CssPseudoElement) -> super::computed::CssComputedStyle<'self> {
            //let element = element.to_ll();
            //let llstyle = unsafe { *self.results }.styles[element];
            
            // FIXME: Rust #3926
            //assert!((llstyle as *c_void).is_not_null());
            // println(fmt!("helpers.rs :: computed_style :: unsafe{transmute(self.computed_style)} == %?" , self.results.styles[element as uint]));
            super::computed::CssComputedStyle {
                result_backref: self,
                computed_style: unsafe{transmute(self.results.styles[element as uint].get_ref())}
            }
        }
    }

}

pub mod values {
    use srid_css::include::properties::*;
    use srid_css::include::types::{css_color, css_unit};
    use srid_css::include::fpmath::css_fixed;
    // use wapcaplet::LwcString;
    use helpers::types::*;
    use helpers::{ll_unit_to_hl_unit};
    use std::cast::*;

    // Like css_color_e
    pub enum CssColorValue {
        CssColorInherit,
        CssColorColor(CssColor)
    }

    impl CssColorValue {
        pub fn new(type_: u8, color: css_color) -> CssColorValue {
            if type_ == CSS_COLOR_INHERIT as u8 {
                CssColorInherit
            } else if type_ == CSS_COLOR_COLOR as u8 {
                CssColorColor(unsafe{transmute(color)})
            } else {
                unimpl("color")
            }
        }
    }

    pub enum CssMarginValue {
        CssMarginInherit,
        CssMarginSet(CssUnit),
        CssMarginAuto
    }

    impl CssMarginValue {
        pub fn new(type_: u8, length: css_fixed, unit: css_unit) -> CssMarginValue {
            if type_  == CSS_MARGIN_INHERIT as u8 {
                CssMarginInherit
            } else if type_ == CSS_MARGIN_SET as u8 {
                CssMarginSet(ll_unit_to_hl_unit(unit, length))
            } else if type_ == CSS_MARGIN_AUTO as u8 {
                CssMarginAuto
            } else {
                unimpl("margin")
            }
        }
    }


    pub enum CssPaddingValue {
        CssPaddingInherit,
        CssPaddingSet(CssUnit)
    }

    impl CssPaddingValue {
        pub fn new(type_: u8, length: css_fixed, unit: css_unit) -> CssPaddingValue {
            if type_ == CSS_PADDING_INHERIT as u8 {
                CssPaddingInherit
            } else if type_ == CSS_PADDING_SET as u8 {
                CssPaddingSet(ll_unit_to_hl_unit(unit, length))
            } else {
                unimpl("padding")
            }
        }
    }

    pub enum CssBorderStyleValue {
        CssBorderStyleInherit,
        CssBorderStyleNone,
        CssBorderStyleHidden,
        CssBorderStyleDotted,
        CssBorderStyleDashed,
        CssBorderStyleSolid,
        CssBorderStyleDouble,
        CssBorderStyleGroove,
        CssBorderStyleRidge,
        CssBorderStyleInset,
        CssBorderStyleOutset
    }

    impl CssBorderStyleValue {
        pub fn new(type_: u8) -> CssBorderStyleValue {
            let type_:css_border_style_e = unsafe{transmute(type_ as u64)};

            match type_ {
                CSS_BORDER_STYLE_INHERIT => CssBorderStyleInherit,
                CSS_BORDER_STYLE_NONE => CssBorderStyleNone,
                CSS_BORDER_STYLE_HIDDEN => CssBorderStyleHidden,
                CSS_BORDER_STYLE_DOTTED => CssBorderStyleDotted,
                CSS_BORDER_STYLE_DASHED => CssBorderStyleDashed,
                CSS_BORDER_STYLE_SOLID => CssBorderStyleSolid,
                CSS_BORDER_STYLE_DOUBLE => CssBorderStyleDouble,
                CSS_BORDER_STYLE_GROOVE => CssBorderStyleGroove,
                CSS_BORDER_STYLE_RIDGE => CssBorderStyleRidge,
                CSS_BORDER_STYLE_INSET => CssBorderStyleInset,
                CSS_BORDER_STYLE_OUTSET => CssBorderStyleOutset,
            }
        }
    }

    pub enum CssBorderWidthValue {
        CssBorderWidthInherit,
        CssBorderWidthThin,
        CssBorderWidthMedium,
        CssBorderWidthThick,
        CssBorderWidthWidth(CssUnit)
    }

    impl CssBorderWidthValue {
        pub fn new(type_: u8, length: css_fixed, unit: css_unit) -> CssBorderWidthValue {
            if type_ == CSS_BORDER_WIDTH_INHERIT  as u8 {
                CssBorderWidthInherit
            } else if type_ == CSS_BORDER_WIDTH_THIN as u8 {
                CssBorderWidthThin
            } else if type_ == CSS_BORDER_WIDTH_MEDIUM  as u8 {
                CssBorderWidthMedium
            } else if type_ == CSS_BORDER_WIDTH_THICK as u8 {
                CssBorderWidthThick
            } else if type_ == CSS_BORDER_WIDTH_WIDTH  as u8 {
                CssBorderWidthWidth(ll_unit_to_hl_unit(unit, length))
            } else {
                unimpl("border_width")
            }
        }
    }

    pub enum CssDisplayValue {
        CssDisplayInherit = 0x00,
        CssDisplayInline = 0x01,
        CssDisplayBlock = 0x02,
        CssDisplayListItem = 0x03,
        CssDisplayRunIn = 0x04,
        CssDisplayInlineBlock = 0x05,
        CssDisplayTable = 0x06,
        CssDisplayInlineTable = 0x07,
        CssDisplayTableRowGroup = 0x08,
        CssDisplayTableHeaderGroup = 0x09,
        CssDisplayTableFooterGroup = 0x0a,
        CssDisplayTableRow = 0x0b,
        CssDisplayTableColumnGroup = 0x0c,
        CssDisplayTableColumn = 0x0d,
        CssDisplayTableCell = 0x0e,
        CssDisplayTableCaption = 0x0f,
        CssDisplayNone = 0x10,
    }

    impl CssDisplayValue {
        pub fn new(type_: u8) -> CssDisplayValue {
            unsafe{transmute(type_ as uint)}
        }
    }

    pub enum CssPositionValue {
        CssPositionInherit = 0x0,
        CssPositionStatic = 0x1,
        CssPositionRelative = 0x2,
        CssPositionAbsolute = 0x3,
        CssPositionFixed = 0x4
    }

    impl CssPositionValue {
        pub fn new(type_: u8) -> CssPositionValue {
            unsafe {transmute(type_ as uint)}
        }
    }

    pub enum CssWidthValue {
        CssWidthInherit,
        CssWidthSet(CssUnit),
        CssWidthAuto
    }

    impl CssWidthValue {
        pub fn new(type_: u8, length: css_fixed, unit: css_unit) -> CssWidthValue {
            if type_ == CSS_WIDTH_INHERIT as u8 {
                CssWidthInherit
            } else if type_ == CSS_WIDTH_SET as u8 {
                CssWidthSet(ll_unit_to_hl_unit(unit, length))
            } else if type_ == CSS_WIDTH_AUTO as u8 {
                CssWidthAuto
            } else {
                unimpl("width")
            }
        }
    }

    pub enum CssHeightValue {
        CssHeightInherit,
        CssHeightSet(CssUnit),
        CssHeightAuto
    }

    impl CssHeightValue {
        pub fn new(type_: u8, length: css_fixed, unit: css_unit) -> CssHeightValue {
            if type_ == CSS_HEIGHT_INHERIT as u8 {
                CssHeightInherit
            } else if type_ == CSS_HEIGHT_SET as u8{
                CssHeightSet(ll_unit_to_hl_unit(unit, length))
            } else if type_ == CSS_HEIGHT_AUTO as u8{
                CssHeightAuto
            } else {
                unimpl("width")
            }
        }
    }

    pub enum CssFloatValue {
        CssFloatInherit = 0x0,
        CssFloatLeft = 0x1,
        CssFloatRight = 0x2,
        CssFloatNone = 0x3
    }

    impl CssFloatValue {
        pub fn new(type_: u8) -> CssFloatValue {
            unsafe {transmute(type_ as uint)}
        }
    }

    pub enum CssClearValue {
        CssClearInherit = 0x0,
        CssClearNone = 0x1,
        CssClearLeft = 0x2,
        CssClearRight = 0x3,
        CssClearBoth = 0x4
    }

    impl CssClearValue {
        pub fn new(type_: u8) -> CssClearValue {
            unsafe {transmute(type_ as uint)}
        }
    }

    pub enum CssFontFamilyValue {
        CssFontFamilyInherit,
        CssFontFamilySerif,
        CssFontFamilySansSerif,
        CssFontFamilyCursive,
        CssFontFamilyFantasy,
        CssFontFamilyMonospace,
        CssFontFamilyValue(~[uint])
    }

    impl CssFontFamilyValue {
        pub fn new(type_: u8, names: ~[uint]) -> CssFontFamilyValue {
            if names.len() != 0{
                CssFontFamilyValue(names)
            } else if type_ == CSS_FONT_FAMILY_INHERIT as u8 {
                CssFontFamilyInherit
            } else if type_ == CSS_FONT_FAMILY_SERIF as u8 {
                CssFontFamilySerif
            } else if type_ == CSS_FONT_FAMILY_SANS_SERIF as u8 {
                CssFontFamilySansSerif
            } else if type_ == CSS_FONT_FAMILY_CURSIVE as u8 {
                CssFontFamilyCursive
            } else if type_ == CSS_FONT_FAMILY_FANTASY as u8 {
                CssFontFamilyFantasy
            } else if type_ == CSS_FONT_FAMILY_MONOSPACE as u8 {
                CssFontFamilyMonospace
            } else {
                fail!()
            }
        }
    }

    pub enum CssFontSizeValue {
        CssFontSizeInherit,
        CssFontSizeXXSmall,
        CssFontSizeXSmall,
        CssFontSizeSmall,
        CssFontSizeMedium,
        CssFontSizeLarge,
        CssFontSizeXLarge,
        CssFontSizeXXLarge,
        CssFontSizeLarger,
        CssFontSizeSmaller,
        CssFontSizeDimension(CssUnit)
    }

    impl CssFontSizeValue {
        pub fn new(type_: u8, length: css_fixed, unit: css_unit) -> CssFontSizeValue {
            //println(fmt!("CssFontSizeValue :: new :: type_ == %? " , type_));
            match type_ {
                x if x == CSS_FONT_SIZE_INHERIT  as u8=> CssFontSizeInherit,
                x if x == CSS_FONT_SIZE_XX_SMALL  as u8=> CssFontSizeXXSmall,
                x if x == CSS_FONT_SIZE_X_SMALL as u8 => CssFontSizeXSmall,
                x if x == CSS_FONT_SIZE_SMALL as u8 => CssFontSizeSmall,
                x if x == CSS_FONT_SIZE_MEDIUM as u8 => CssFontSizeMedium,
                x if x == CSS_FONT_SIZE_LARGE as u8 => CssFontSizeLarge,
                x if x == CSS_FONT_SIZE_X_LARGE as u8 => CssFontSizeXLarge,
                x if x == CSS_FONT_SIZE_XX_LARGE as u8  => CssFontSizeXXLarge,
                x if x == CSS_FONT_SIZE_LARGER as u8 => CssFontSizeLarger,
                x if x == CSS_FONT_SIZE_SMALLER as u8 => CssFontSizeSmaller,
                x if x == CSS_FONT_SIZE_DIMENSION as u8 => CssFontSizeDimension(ll_unit_to_hl_unit(unit, length)),
                _ => fail!()
            }
        }
    }

    pub enum CssFontStyleValue {
        CssFontStyleInherit = 0x0,
        CssFontStyleNormal = 0x1,
        CssFontStyleItalic = 0x2,
        CssFontStyleOblique = 0x3
    }

    impl CssFontStyleValue {
        pub fn new(type_: u8) -> CssFontStyleValue {
            unsafe {transmute(type_ as uint)}
        }
    }

    pub enum CssFontWeightValue {
        CssFontWeightInherit = 0x0,
        CssFontWeightNormal = 0x1,
        CssFontWeightBold = 0x2,
        CssFontWeightBolder = 0x3,
        CssFontWeightLighter = 0x4,
        CssFontWeight100 = 0x5,
        CssFontWeight200 = 0x6,
        CssFontWeight300 = 0x7,
        CssFontWeight400 = 0x8,
        CssFontWeight500 = 0x9,
        CssFontWeight600 = 0xa,
        CssFontWeight700 = 0xb,
        CssFontWeight800 = 0xc,
        CssFontWeight900 = 0xd
    }

    impl CssFontWeightValue {
        pub fn new(type_: u8) -> CssFontWeightValue {
            unsafe {transmute(type_ as uint)}
        }
    }

    pub enum CssTextAlignValue {
    CssTextAlignInherit = 0x0,
    CssTextAlignInheritIfNonMagic = 0x1,
    CssTextAlignLeft = 0x2,
    CssTextAlignRight = 0x3,
    CssTextAlignCenter = 0x4,
    CssTextAlignJustify = 0x5,
    CssTextAlignDefault = 0x6,
    CssTextAlignLibcssLeft = 0x7,
    CssTextAlignLibcssCenter = 0x8,
    CssTextAlignLibcssRight = 0x9
    }

    impl CssTextAlignValue {
        pub fn new(type_: u8) -> CssTextAlignValue {
            unsafe {transmute(type_ as uint)}
        }
    }

    pub enum CssTextDecorationValue{
    CssTextDecorationInherit = 0x00,
    CssTextDecorationNone = 0x10,
    CssTextDecorationBlink = (1<<3),
    CssTextDecorationLineThrough = (1<<2),
    CssTextDecorationOverline = (1<<1),
    CssTextDecorationUnderline = (1<<0),
    }

    impl CssTextDecorationValue {
        pub fn new(type_: u8) -> CssTextDecorationValue {
            unsafe {transmute(type_ as uint)}
        }
    }

    pub enum CssLineHeightValue {
        CssLineHeightInherit,
        CssLineHeightNumber(css_fixed),
        CssLineHeightDimension(CssUnit),
        CssLineHeightNormal
    }

    impl CssLineHeightValue {
        pub fn new(type_: u8, length: css_fixed, unit: css_unit) -> CssLineHeightValue {
            if type_ == CSS_LINE_HEIGHT_INHERIT as u8 {
                CssLineHeightInherit
            } else if type_ == CSS_LINE_HEIGHT_NUMBER as u8 {
                CssLineHeightNumber(length)
            } else if type_ == CSS_LINE_HEIGHT_DIMENSION as u8 {
                CssLineHeightDimension(ll_unit_to_hl_unit(unit, length))
            } else if type_ == CSS_LINE_HEIGHT_NORMAL as u8 {
                CssLineHeightNormal
            } else {
                unimpl("line-height")
            }
        }
    }

    pub enum CssVerticalAlignValue {
        CssVerticalAlignInherit,
        CssVerticalAlignBaseline,
        CssVerticalAlignSub,
        CssVerticalAlignSuper,
        CssVerticalAlignTop,
        CssVerticalAlignTextTop,
        CssVerticalAlignMiddle,
        CssVerticalAlignBottom,
        CssVerticalAlignTextBottom,
        CssVerticalAlignDimension(CssUnit),
    }

    impl CssVerticalAlignValue {
        pub fn new(type_: u8, length: css_fixed, unit: css_unit) -> CssVerticalAlignValue {
            let type_:css_vertical_align_e = unsafe{transmute(type_ as u64)}; 
            match type_ {
                CSS_VERTICAL_ALIGN_INHERIT=> CssVerticalAlignInherit,
                CSS_VERTICAL_ALIGN_BASELINE => CssVerticalAlignBaseline,
                CSS_VERTICAL_ALIGN_SUB => CssVerticalAlignSub,
                CSS_VERTICAL_ALIGN_SUPER => CssVerticalAlignSuper,
                CSS_VERTICAL_ALIGN_TOP => CssVerticalAlignTop,
                CSS_VERTICAL_ALIGN_TEXT_TOP => CssVerticalAlignTextTop,
                CSS_VERTICAL_ALIGN_MIDDLE => CssVerticalAlignMiddle,
                CSS_VERTICAL_ALIGN_BOTTOM => CssVerticalAlignBottom,
                CSS_VERTICAL_ALIGN_TEXT_BOTTOM => CssVerticalAlignTextBottom,
                CSS_VERTICAL_ALIGN_SET => CssVerticalAlignDimension(ll_unit_to_hl_unit(unit, length)),
            }
        }
    }

    fn unimpl(what: &str) -> ! {
        fail!(fmt!("unimplemented css value: %?", what));
    }

}

pub mod computed {
    // use hint::CssHint;
    use helpers::select::CssSelectResults;
    use helpers::values::{CssColorValue, CssMarginValue, CssPaddingValue, CssBorderStyleValue, CssBorderWidthValue, CssDisplayValue};
    use helpers::values::{CssFloatValue, CssClearValue, CssPositionValue, CssWidthValue, CssHeightValue, CssFontFamilyValue};
    use helpers::values::{CssFontSizeValue, CssFontStyleValue, CssFontWeightValue, CssTextAlignValue, CssTextDecorationValue};
    use helpers::values::{CssLineHeightValue, CssVerticalAlignValue};
    use srid_css::include::types::*;
    use srid_css::select::computed::*;
    use srid_css::select::common::css_computed_style;
    use std::cast::transmute;
    

    //#[deriving(DeepClone)]
    pub struct CssComputedStyle<'self> {
        // A borrowed back reference to ensure this outlives the results
        result_backref: &'self CssSelectResults,
        computed_style: *css_computed_style,
    }

     impl<'self> CssComputedStyle<'self> {
        #[fixed_stack_segment]
        pub fn color(&self) -> CssColorValue {
            //println(fmt!("helpers.rs :: color"));
            let mut color: u32 = 0;
            let type_ = css_computed_color(unsafe{transmute(self.computed_style)} , &mut color);
            // let type_ = type_ as css_color_e;

            CssColorValue::new(type_, color)
        }

        #[fixed_stack_segment]
        pub fn background_color(&self) -> CssColorValue {
            //println(fmt!("helpers.rs :: background_color"));
            let mut color: u32 = 0;
            let type_ = css_computed_background_color(unsafe{transmute(self.computed_style)} , &mut color);
            // let type_ = type_ as css_color_e;

            CssColorValue::new(type_, color)
        }

        #[fixed_stack_segment]
        pub fn border_top_style(&self) -> CssBorderStyleValue {
            //println(fmt!("helpers.rs :: border_top_style"));
            let type_ = css_computed_border_top_style(unsafe{transmute(self.computed_style)}) ;
            // let type_ = type_ as css_border_style_e;

            CssBorderStyleValue::new(type_)
        }

        #[fixed_stack_segment]
        pub fn border_right_style(&self) -> CssBorderStyleValue {
            //println(fmt!("helpers.rs :: border_right_style"));
            let type_ = css_computed_border_right_style(unsafe{transmute(self.computed_style)});
            // let type_ = type_ as css_border_style_e;

            CssBorderStyleValue::new(type_)
        }

        #[fixed_stack_segment]
       pub fn border_bottom_style(&self) -> CssBorderStyleValue {
        //println(fmt!("helpers.rs :: border_bottom_style"));
            let type_ = css_computed_border_bottom_style(unsafe{transmute(self.computed_style)});
            // let type_ = type_ as css_border_style_e;

            CssBorderStyleValue::new(type_)
        }

        #[fixed_stack_segment]
        pub fn border_left_style(&self) -> CssBorderStyleValue {
            //println(fmt!("helpers.rs :: border_left_style"));
            let type_ = css_computed_border_left_style(unsafe{transmute(self.computed_style)});
            // let type_ = type_ as css_border_style_e;

            CssBorderStyleValue::new(type_)
        }

        #[fixed_stack_segment]
        pub fn border_top_width(&self) -> CssBorderWidthValue {
            //println(fmt!("helpers.rs :: border_top_width"));

            let mut length = 0;
            
            let mut unit = CSS_UNIT_PX;
            let type_= css_computed_border_top_width(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);
            // let type_ = type_ as css_border_width_e;

            CssBorderWidthValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn border_right_width(&self) -> CssBorderWidthValue {
            //println(fmt!("helpers.rs :: border_right_width"));

            let mut length = 0;
            let mut unit = CSS_UNIT_PX;
            let type_  = css_computed_border_right_width(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);
            // let type_ = type_ as css_border_width_e;

            CssBorderWidthValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn border_bottom_width(&self) -> CssBorderWidthValue {
            // println(fmt!("helpers.rs :: border_bottom_width"));
            let mut length = 0;
            let mut unit = CSS_UNIT_PX;
            let type_  = css_computed_border_bottom_width(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);
            // let type_ = type_ as css_border_width_e;

            CssBorderWidthValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn border_left_width(&self) -> CssBorderWidthValue {
            // println(fmt!("helpers.rs :: border_left_width"));
            let mut length = 0;
            let mut unit = CSS_UNIT_PX;
            let type_  = css_computed_border_left_width(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);
            // let type_ = type_ as css_border_width_e;

            CssBorderWidthValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn border_top_color(&self) -> CssColorValue {
            //println(fmt!("helpers.rs :: border_top_color"));
            let mut color = 0;
            let type_  = css_computed_border_top_color(unsafe{transmute(self.computed_style)} , &mut color);
            // let type_ = type_ as css_color_e;
            CssColorValue::new(type_, color)
        }

        #[fixed_stack_segment]
        pub fn border_right_color(&self) -> CssColorValue {
            //println(fmt!("helpers.rs :: border_right_color"));
            let mut color = 0;
            let type_ = css_computed_border_right_color(unsafe{transmute(self.computed_style)} , &mut color);
            // let type_ = type_ as css_color_e;
            CssColorValue::new(type_, color)
        }

        #[fixed_stack_segment]
        pub fn border_bottom_color(&self) -> CssColorValue {
            //println(fmt!("helpers.rs :: border_bottom_color"));
            let mut color = 0;
            let type_ = css_computed_border_bottom_color(unsafe{transmute(self.computed_style)} , &mut color);
            // let type_ = type_ as css_color_e;
            CssColorValue::new(type_, color)
        }

        #[fixed_stack_segment]
        pub fn border_left_color(&self) -> CssColorValue {
            //println(fmt!("helpers.rs :: border_left_color"));
            let mut color = 0;
            let type_ = css_computed_border_left_color(unsafe{transmute(self.computed_style)} , &mut color);
            CssColorValue::new(type_, color)
        }

        #[fixed_stack_segment]
        pub fn margin_top(&self) -> CssMarginValue {
            // println(fmt!("helpers.rs :: margin_top"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_ = css_computed_margin_top(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);
            // let type_ = type_ as css_margin_e;

            CssMarginValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn margin_right(&self) -> CssMarginValue {
            // println(fmt!("helpers.rs :: margin_right"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_ = css_computed_margin_right(unsafe{transmute(self.computed_style)}, &mut length , &mut unit);
            //let type_ = type_ as css_margin_e;

            CssMarginValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn margin_bottom(&self) -> CssMarginValue {
            // println(fmt!("helpers.rs :: margin_bottom"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_ = css_computed_margin_bottom(unsafe{transmute(self.computed_style)}, &mut length , &mut unit);
            //let type_ = type_ as css_margin_e;

            CssMarginValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn margin_left(&self) -> CssMarginValue {
            // println(fmt!("helpers.rs :: margin_left"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_ = css_computed_margin_left(unsafe{transmute(self.computed_style)}, &mut length , &mut unit);
            
            CssMarginValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn padding_top(&self) -> CssPaddingValue {
            //println(fmt!("helpers.rs :: padding_top"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_ = css_computed_padding_top(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);
            
            CssPaddingValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn padding_right(&self) -> CssPaddingValue {
            //println(fmt!("helpers.rs :: padding_right"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_ = css_computed_padding_right(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);

            CssPaddingValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn padding_bottom(&self) -> CssPaddingValue {
            //println(fmt!("helpers.rs :: padding_bottom"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_ = css_computed_padding_bottom(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);

            CssPaddingValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn padding_left(&self) -> CssPaddingValue {
            //println(fmt!("helpers.rs :: padding_left"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_ = css_computed_padding_left(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);

            CssPaddingValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn display(&self, root: bool) -> CssDisplayValue {
            //println(fmt!("helpers.rs :: display"));
            let type_ = css_computed_display(unsafe{transmute(self.computed_style)}, root);
            // println(fmt!("helpers.rs :: display :: type_ == %? " , type_));
            // println(fmt!("helpers.rs :: display :: return type_ == %? " , CssDisplayValue::new(type_)));
            CssDisplayValue::new(type_)
        }

        #[fixed_stack_segment]
        pub fn position(&self) -> CssPositionValue {
            //println(fmt!("helpers.rs :: position"));
            let type_ = css_computed_position(unsafe{transmute(self.computed_style)});

            CssPositionValue::new(type_)
        }

        #[fixed_stack_segment]
        pub fn width(&self) -> CssWidthValue {
            // println(fmt!("helpers.rs :: width"));
            let mut length: i32 = 0;
            let mut unit: css_unit = CSS_UNIT_PX;
            let type_ = css_computed_width(unsafe{transmute(self.computed_style)}, &mut length , &mut unit);

            CssWidthValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn height(&self) -> CssHeightValue {
            // println(fmt!("helpers.rs :: height"));
            let mut length: i32 = 0;
            let mut unit: css_unit = CSS_UNIT_PX;
            let type_ = css_computed_height(unsafe{transmute(self.computed_style)}, &mut length , &mut unit);

            CssHeightValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn float(&self) -> CssFloatValue {
            //println(fmt!("helpers.rs :: float"));
            let type_ = css_computed_float(unsafe{transmute(self.computed_style)});

            CssFloatValue::new(type_)
        }

        #[fixed_stack_segment]
        pub fn clear(&self) -> CssClearValue {
            //println(fmt!("helpers.rs :: clear"));
            let type_ = css_computed_clear(unsafe{transmute(self.computed_style)});

            CssClearValue::new(type_)
        }

        #[fixed_stack_segment]
        pub fn font_family(&self) -> CssFontFamilyValue {
            // println(fmt!("helpers.rs :: font_family"));
            // println(fmt!("font_family :: computed_style == %? " , (self.computed_style)));
            let mut names: ~[uint] = ~[];
            let type_ = css_computed_font_family(unsafe{transmute(self.computed_style)} , &mut names);
            // let type_ = type_ as css_font_family_e;

            CssFontFamilyValue::new(type_, names)
        }

        #[fixed_stack_segment]
        pub fn font_size(&self) -> CssFontSizeValue {
            //println(fmt!("helpers.rs :: font_size"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_ = css_computed_font_size(unsafe{transmute(self.computed_style)} , &mut length , &mut unit) ;

            CssFontSizeValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn font_style(&self) -> CssFontStyleValue {
            //println(fmt!("helpers.rs :: font_style"));
            let type_ = css_computed_font_style(unsafe{transmute(self.computed_style)});

            CssFontStyleValue::new(type_)
        }

        #[fixed_stack_segment]
        pub fn font_weight(&self) -> CssFontWeightValue {
            //println(fmt!("helpers.rs :: font_weight"));
            let type_ = css_computed_font_weight(unsafe{transmute(self.computed_style)});

            CssFontWeightValue::new(type_)
        }

        #[fixed_stack_segment]
        pub fn text_align(&self) -> CssTextAlignValue {
            //println(fmt!("helpers.rs :: text_align"));
            let type_ = css_computed_text_align(unsafe{transmute(self.computed_style)}) ;

            CssTextAlignValue::new(type_)
        }

        #[fixed_stack_segment]
        pub fn text_decoration(&self) -> CssTextDecorationValue {
            //println(fmt!("helpers.rs :: text_decoration"));
            let type_ = css_computed_text_decoration(unsafe{transmute(self.computed_style)});
            debug!("Getting text-decoration raw: %?", type_);

            CssTextDecorationValue::new(type_)
        }


        #[fixed_stack_segment]
        pub fn line_height(&self) -> CssLineHeightValue {
            //println(fmt!("helpers.rs :: line_height"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_  = css_computed_line_height(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);

            CssLineHeightValue::new(type_, length, unit)
        }

        #[fixed_stack_segment]
        pub fn vertical_align(&self) -> CssVerticalAlignValue {
            //println(fmt!("helpers.rs :: vertical_align"));
            let mut length: i32 = 0;
            let mut unit = CSS_UNIT_PX;
            let type_  = css_computed_vertical_align(unsafe{transmute(self.computed_style)} , &mut length , &mut unit);

            CssVerticalAlignValue::new(type_, length, unit)
        }
    }
}


