use select::common::*;

use std::cast::*;

use include::fpmath::*;
use include::types::*;
use include::properties::*;

pub fn css_computed_letter_spacing(style : &~css_computed_style , length: &mut i32 , unit: &mut css_unit) 
        -> u8 {

    if style.uncommon.is_some() {
        let mut bits:u8= style.uncommon.get_ref().bits[CSS_LETTER_SPACING_INDEX];
        bits = bits & (CSS_LETTER_SPACING_MASK as u8);
        bits = bits >> CSS_LETTER_SPACING_SHIFT;

        if (bits&3) == (CSS_LETTER_SPACING_SET as u8) { 
            *length = style.uncommon.get_ref().letter_spacing;
            *unit = unsafe { transmute((bits >> 2)as int) };
        }

        return (bits&3)
        
    }
    return (CSS_LETTER_SPACING_NORMAL as u8)
}

pub fn css_computed_outline_color(
                    style: &~css_computed_style , color : &mut u32) 
                    -> u8 {

    if style.uncommon.is_some() {
        let mut bits:u8= style.uncommon.get_ref().bits[CSS_OUTLINE_COLOR_INDEX];
        bits = bits & (CSS_OUTLINE_COLOR_MASK as u8) ;
        bits = bits >> CSS_OUTLINE_COLOR_SHIFT ;

        if (bits&3) == (CSS_OUTLINE_COLOR_COLOR as u8) { 
            *color = style.uncommon.get_ref().outline_color;
        }

        return (bits&3)
        
    }
    return (CSS_OUTLINE_COLOR_INVERT as u8)
}


pub fn css_computed_outline_width(
        style : &~css_computed_style , length: &mut i32 , unit: &mut css_unit) 
        -> u8 {

    
    if style.uncommon.is_some() {
        let mut bits:u8= style.uncommon.get_ref().bits[CSS_OUTLINE_WIDTH_INDEX];
        bits = bits & (CSS_OUTLINE_WIDTH_MASK as u8);
        bits = bits >> CSS_OUTLINE_WIDTH_SHIFT;

        if (bits&7) == (CSS_OUTLINE_WIDTH_WIDTH as u8) { 
            *length = style.uncommon.get_ref().outline_width;
            *unit = unsafe { transmute((bits >> 3)as int) };
        }
        return (bits &7)

    }
    
    *length = css_int_to_fixed(2);
    *unit = CSS_UNIT_PX;

    return CSS_OUTLINE_WIDTH_WIDTH as u8;

}

pub fn css_computed_border_spacing(
                    style : &~css_computed_style, hlength: &mut i32 , hunit: &mut css_unit , vlength: &mut i32 , vunit: &mut css_unit)
                    -> u8 {
    let mut bits: u8 = 0;
    if style.uncommon.is_some() {
        bits = style.uncommon.get_ref().bits[CSS_BORDER_SPACING_INDEX];
        bits = bits & (CSS_BORDER_SPACING_MASK as u8);
        bits = bits >> CSS_BORDER_SPACING_SHIFT ;

        if bits == (CSS_BORDER_SPACING_SET as u8) { 
            let mut bits1 = style.uncommon.get_ref().bits[CSS_BORDER_SPACING_INDEX1];
            bits1 = bits1 & (CSS_BORDER_SPACING_MASK1 as u8);
            bits1 = bits1 >> CSS_BORDER_SPACING_SHIFT1 ;

            *hlength = style.uncommon.get_ref().border_spacing[0];
            *hunit = unsafe { transmute((bits1 >> 4)as int) } ;

            *vlength = style.uncommon.get_ref().border_spacing[1];
            *vunit = unsafe { transmute((bits1 & 0xf)as int) } ;
        }
        *hlength = 0;
        *vlength = 0;
        *hunit = CSS_UNIT_PX;
        *vunit = CSS_UNIT_PX;
    }
    bits
}

pub fn css_computed_word_spacing(
                    style : &~css_computed_style , length: &mut i32 , unit: &mut css_unit)
                    -> u8 {

    if style.uncommon.is_some() {
        let mut bits:u8= style.uncommon.get_ref().bits[CSS_WORD_SPACING_INDEX];
        bits = bits & (CSS_WORD_SPACING_MASK as u8);
        bits = bits >> CSS_WORD_SPACING_SHIFT;

        if (bits&3) == (CSS_WORD_SPACING_SET as u8) { 
            *length = style.uncommon.get_ref().word_spacing;
            *unit = unsafe { transmute((bits >> 2)as int) };
        }

        return (bits&3);
        
    }
    return (CSS_WORD_SPACING_NORMAL as u8);
}

pub fn css_computed_counter_increment(
                        style : &~css_computed_style , counter : &mut ~[~css_computed_counter])
                        -> u8 {

    if style.uncommon.is_some() {
        let mut bits:u8= style.uncommon.get_ref().bits[CSS_COUNTER_INCREMENT_INDEX];
        bits = bits & (CSS_COUNTER_INCREMENT_MASK as u8);
        bits = bits >> CSS_COUNTER_INCREMENT_SHIFT;

        *counter = style.uncommon.get_ref().counter_increment.clone();

        return bits ;
        
    }
    return CSS_COUNTER_INCREMENT_NONE as u8 ;
}

pub fn css_computed_counter_reset(
                        style : &~css_computed_style , counter: &mut ~[~css_computed_counter])
                        -> u8 {

    if style.uncommon.is_some() {
        let mut bits:u8= style.uncommon.get_ref().bits[CSS_COUNTER_RESET_INDEX];
        bits = bits & (CSS_COUNTER_RESET_MASK as u8);
        bits = bits >> CSS_COUNTER_RESET_SHIFT;

        *counter = style.uncommon.get_ref().counter_reset.clone();

        return bits;
        
    }
    return CSS_COUNTER_RESET_NONE as u8 ;
}

pub fn css_computed_cursor(
                style : &~css_computed_style , urls: &mut ~[uint])
                -> u8 {

    if style.uncommon.is_some() {
        let mut bits:u8= style.uncommon.get_ref().bits[CSS_CURSOR_INDEX];
        bits = bits & (CSS_CURSOR_MASK as u8);
        bits = bits >> CSS_CURSOR_SHIFT;

        *urls = style.uncommon.get_ref().cursor.clone();

        return bits;
        
    }
    return CSS_CURSOR_AUTO as u8;
}

pub fn css_computed_clip(
            style : &~css_computed_style,
            result: &mut ~css_computed_clip_rect) 
            -> (u8, bool) {


    if style.uncommon.is_none() {
        (CSS_CLIP_AUTO as u8, false)
    }
    else {
        let mut bits:u8= style.uncommon.get_ref().bits[CSS_CLIP_INDEX];
        bits = bits & (CSS_CLIP_MASK as u8);
        bits = bits >> CSS_CLIP_SHIFT;

        if (bits&0x3) == (CSS_CLIP_RECT as u8) {
            let mut bits1 : u8 ;

            result.left_auto = (bits & 0x4)!=0;
            result.bottom_auto = (bits & 0x8)!=0;
            result.right_auto = (bits & 0x10)!=0;
            result.top_auto = (bits & 0x20)!=0;

            if (result.top_auto == false ||
                    result.right_auto == false) {
                /* 8bits: ttttrrrr : top | right */
                bits1 = style.uncommon.get_ref().bits[CSS_CLIP_INDEX1];
                bits1 &= (CSS_CLIP_MASK1 as u8);
                bits1 >>= CSS_CLIP_SHIFT1;
            } 
            else {
                bits1 = 0;
            }

            result.top = style.uncommon.get_ref().clip[0];
            result.tunit = unsafe { transmute((bits1 >> 4)as int)};

            result.right = style.uncommon.get_ref().clip[1];
            result.runit = unsafe { transmute((bits1 & 0xf)as int)};

            if (result.bottom_auto == false ||
                    result.left_auto == false) {
                /* 8bits: bbbbllll : bottom | left */
                bits1 = style.uncommon.get_ref().bits[CSS_CLIP_INDEX2];
                bits1 &= (CSS_CLIP_MASK2 as u8);
                bits1 >>= CSS_CLIP_SHIFT2;
            } 
            else {
                bits1 = 0;
            }

            result.bottom = style.uncommon.get_ref().clip[2];
            result.bunit = unsafe { transmute((bits1 >> 4)as int)};

            result.left = style.uncommon.get_ref().clip[3];
            result.lunit = unsafe { transmute((bits1 & 0xf)as int)} ;
        }

        ((bits&0x3), true)
    }
}

pub fn css_computed_content(
                style : &~css_computed_style , result : &mut ~[~css_computed_content_item])
                -> u8 {

    if style.uncommon.is_none() {
        (CSS_CONTENT_NORMAL as u8)
    }
    else {
        let mut bits:u8= style.uncommon.get_ref().bits[CSS_CONTENT_INDEX];
        bits = bits & (CSS_CONTENT_MASK as u8);
        bits = bits >> CSS_CONTENT_SHIFT;
        *result = style.uncommon.get_ref().content.clone();
        bits
    }
}

pub fn css_computed_vertical_align(
                    style : &~css_computed_style , length: &mut i32 , unit: &mut css_unit) 
                        ->u8 {

    let mut bits = style.bits[CSS_VERTICAL_ALIGN_INDEX];
    // println(fmt!("css_computed_vertical_align :: bits == %? " , bits));
    bits = bits & (CSS_VERTICAL_ALIGN_MASK as u8);
    bits = bits >> CSS_VERTICAL_ALIGN_SHIFT;
    // println(fmt!("css_computed_vertical_align :: bits after mask and shift == %? " , bits));

    if ((bits&0xf)==(CSS_VERTICAL_ALIGN_SET as u8)) {
        *length = style.vertical_align;
        *unit = unsafe { transmute((bits >> 4) as int)};
    }
    // println(fmt!("css_computed_vertical_align  return values :: bits == %?  , length == %?  , unit == %? " , bits&0xf , length , unit));
    return (bits&0xf)
}

pub fn css_computed_font_size(
                            style : &~css_computed_style , length: &mut i32 , unit: &mut css_unit)
                            -> u8 {

    let mut bits = style.bits[CSS_FONT_SIZE_INDEX];
    bits = bits & (CSS_FONT_SIZE_MASK as u8);
    bits = bits >> CSS_FONT_SIZE_SHIFT;

    if ((bits&0xf)==(CSS_FONT_SIZE_DIMENSION as u8)) {
        *length = style.font_size;
        *unit = unsafe { transmute((bits >> 4) as int)};
    }
    return (bits&0xf);
}

pub fn css_computed_border_top_width(style : &~css_computed_style , length: &mut i32 , unit: &mut css_unit)
                            -> u8 {

    // let mut length : Option<i32> = None;
    // let mut unit : Option<css_unit> = None;
    let mut bits = style.bits[CSS_BORDER_TOP_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_TOP_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_TOP_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        *length = style.border_width[0];
        *unit = unsafe { transmute((bits >> 3) as int)};
        // return ((bits&0x7),length,unit);
    }

    return (bits & 0x7);
}

pub fn css_computed_border_right_width(style : &~css_computed_style , length: &mut i32 , unit: &mut css_unit)
                            -> u8 {

    let mut bits = style.bits[CSS_BORDER_RIGHT_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_RIGHT_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_RIGHT_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        *length = style.border_width[1];
        *unit = unsafe { transmute((bits >> 3) as int)};
    }
    return (bits & 0x7);
}

pub fn css_computed_border_bottom_width(
                            style : &~css_computed_style , length: &mut i32 , unit: &mut css_unit)
                            -> u8 {

    let mut bits = style.bits[CSS_BORDER_BOTTOM_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_BOTTOM_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_BOTTOM_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        *length = style.border_width[2];
        *unit = unsafe { transmute((bits >> 3) as int)};
    }

    return (bits & 0x7)
}

pub fn css_computed_border_left_width(
                            style : &~css_computed_style , length: &mut i32 , unit : &mut css_unit)
                            -> u8 {

    let mut bits = style.bits[CSS_BORDER_LEFT_WIDTH_INDEX];
    bits = bits & (CSS_BORDER_LEFT_WIDTH_MASK as u8);
    bits = bits >> CSS_BORDER_LEFT_WIDTH_SHIFT;

    if ((bits&0x7)==(CSS_BORDER_WIDTH_WIDTH as u8)) {
        *length = style.border_width[3];
        *unit = unsafe { transmute((bits >> 3) as int)};
    }

    return (bits & 0x7)
}


pub fn css_computed_background_image(
                                    style:&~css_computed_style , url: &mut uint)
                                    -> u8 {

    let mut bits = style.bits[CSS_BACKGROUND_IMAGE_INDEX];
    bits = bits & (CSS_BACKGROUND_IMAGE_MASK as u8);
    bits = bits >> CSS_BACKGROUND_IMAGE_SHIFT;

    *url = style.background_image.unwrap();

    return bits;
}

pub fn css_computed_color(
                        style : &~css_computed_style , color: &mut u32)
                        ->u8 {

    let mut bits = style.bits[CSS_COLOR_INDEX];
    bits = bits & (CSS_COLOR_MASK as u8);
    bits = bits >> CSS_COLOR_SHIFT ;
    *color = style.color;
    return bits;
}

pub fn css_computed_list_style_image(
                                    style : &~css_computed_style , url : &mut uint)
                                    ->u8 {

    let mut bits = style.bits[CSS_LIST_STYLE_IMAGE_INDEX];
    bits = bits & (CSS_LIST_STYLE_IMAGE_MASK as u8);
    bits = bits >> CSS_LIST_STYLE_IMAGE_SHIFT;

    *url = style.list_style_image.unwrap();

    bits
}

pub fn css_computed_quotes(
                        style:&~css_computed_style , names: &mut ~[uint])
                        -> u8 {
    
    let mut bits = style.bits[CSS_QUOTES_INDEX];
    bits = bits & (CSS_QUOTES_MASK as u8);
    bits = bits >> CSS_QUOTES_SHIFT;
    *names = style.quotes.clone();

    bits
}

pub fn css_computed_top(style : &~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                    -> u8{

    let mut bits : u8 = style.bits[CSS_TOP_INDEX];
    bits = bits & (CSS_TOP_MASK as u8);
    bits = bits >> CSS_TOP_SHIFT;   

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_TOP_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut bottom : u8 = style.bits[CSS_BOTTOM_INDEX];
        bottom = bottom & (CSS_BOTTOM_MASK as u8);
        bottom = bottom >> CSS_BOTTOM_SHIFT;  

        if( (bits&0x3)==(CSS_TOP_AUTO as u8) &&
            (bottom&0x3)==(CSS_BOTTOM_AUTO as u8) ) {
            *length = 0;
            *unit = CSS_UNIT_PX;
        }
        else if ( (bits&0x3)==(CSS_TOP_AUTO as u8) ) {
            *length = -style.bottom;
            *unit = unsafe { transmute((bottom >> 2) as int)};
        }
        else {
            *length = style.top;
            *unit = unsafe { transmute((bits >> 2) as int)};
        }

        bits = (CSS_TOP_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_TOP_SET as u8) ) {
        *length = style.top;
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    (bits&0x3)
}

pub fn css_computed_right(
                        style: &~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                        -> u8{

    let mut bits : u8 = style.bits[CSS_RIGHT_INDEX];
    bits = bits & (CSS_RIGHT_MASK as u8);
    bits = bits >> CSS_RIGHT_SHIFT;   

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_RIGHT_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut left : u8 = style.bits[CSS_LEFT_INDEX];
        left = left & (CSS_LEFT_MASK as u8);
        left = left >> CSS_LEFT_SHIFT;  

        if( (bits&0x3)==(CSS_RIGHT_AUTO as u8) &&
            (left&0x3)==(CSS_LEFT_AUTO as u8) ) {
            *length = 0;
            *unit = CSS_UNIT_PX;
        }
        else if ( (bits&0x3)==(CSS_RIGHT_AUTO as u8) ) {
            *length = -style.left;
            *unit = unsafe { transmute((left >> 2) as int)};
        }
        else {
            *length = style.right;
            *unit = unsafe { transmute((bits >> 2) as int)};
        }

        bits = (CSS_RIGHT_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_RIGHT_SET as u8) ) {
        *length = style.right;
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    (bits&0x3)
}

pub fn css_computed_bottom(
                        style: &~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                        -> u8{

    let mut bits : u8 = style.bits[CSS_BOTTOM_INDEX];
    bits = bits & (CSS_BOTTOM_MASK as u8);
    bits = bits >> CSS_BOTTOM_SHIFT;   

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_BOTTOM_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut top : u8 = style.bits[CSS_TOP_INDEX];
        top = top & (CSS_TOP_MASK as u8);
        top = top >> CSS_TOP_SHIFT;  

        if( (bits&0x3)==(CSS_BOTTOM_AUTO as u8) &&
            (top&0x3)==(CSS_TOP_AUTO as u8) ) {
            *length = 0;
            *unit = CSS_UNIT_PX;
        }
        else if ( (bits&0x3)==(CSS_BOTTOM_AUTO as u8) || 
                   (top&0x3)==(CSS_TOP_AUTO as u8) ) {
            *length = -style.top;
            *unit = unsafe { transmute((top >> 2) as int)};
        }
        else {
            *length = style.bottom;
            *unit = unsafe { transmute((bits >> 2) as int)};
        }

        bits = (CSS_BOTTOM_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_BOTTOM_SET as u8) ) {
        *length = style.bottom;
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    (bits&0x3)
}

pub fn css_computed_left(
                        style: &~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                        -> u8{

    let mut bits : u8 = style.bits[CSS_LEFT_INDEX];
    bits = bits & (CSS_LEFT_MASK as u8);
    bits = bits >> CSS_LEFT_SHIFT;   

    if( css_computed_position(style)==(CSS_POSITION_STATIC as u8) ){
        bits = (CSS_LEFT_AUTO as u8);
    }
    else if ( css_computed_position(style)==(CSS_POSITION_RELATIVE as u8) ) {
        let mut right : u8 = style.bits[CSS_RIGHT_INDEX];
        right = right & (CSS_RIGHT_MASK as u8);
        right = right >> CSS_RIGHT_SHIFT;  

        if( (bits&0x3)==(CSS_LEFT_AUTO as u8) &&
            (right&0x3)==(CSS_RIGHT_AUTO as u8) ) {
            *length = 0;
            *unit = CSS_UNIT_PX;
        }
        else if ( (bits&0x3)==(CSS_LEFT_AUTO as u8) ) {
            *length = -style.right;
            *unit = unsafe { transmute((right >> 2) as int)};
        }
        else {
            *length = style.left;
            *unit = unsafe { transmute((bits >> 2) as int)};
        }

        bits = (CSS_LEFT_SET as u8);
    }
    else if ( (bits&0x3)==(CSS_LEFT_SET as u8) ) {
        *length = style.left;
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    (bits&0x3)
}

pub fn css_computed_border_top_color(style: &~css_computed_style , color: &mut u32)
                                    -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_TOP_COLOR_INDEX];
    bits = bits & (CSS_BORDER_TOP_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_TOP_COLOR_SHIFT; 
    *color = style.border_color[0];
    return bits;
}

pub fn css_computed_border_right_color(style: &~css_computed_style, color: &mut u32)
                                    -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_RIGHT_COLOR_INDEX];
    bits = bits & (CSS_BORDER_RIGHT_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_RIGHT_COLOR_SHIFT; 
    *color = style.border_color[1];
    return bits;
}

pub fn css_computed_border_bottom_color(style: &~css_computed_style, color: &mut u32)
                                    -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_BOTTOM_COLOR_INDEX];
    bits = bits & (CSS_BORDER_BOTTOM_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_BOTTOM_COLOR_SHIFT; 
    *color = style.border_color[2];
    return bits; 
}

pub fn css_computed_border_left_color(style: &~css_computed_style, color: &mut u32)
                                    -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_LEFT_COLOR_INDEX];
    bits = bits & (CSS_BORDER_LEFT_COLOR_MASK as u8);
    bits = bits >> CSS_BORDER_LEFT_COLOR_SHIFT; 
    *color = style.border_color[3];
    return bits; 
}

pub fn css_computed_height(style: &~css_computed_style , length: &mut i32 , unit: &mut css_unit)
                        -> u8 {

    let mut bits : u8 = style.bits[CSS_HEIGHT_INDEX];
    bits = bits & (CSS_HEIGHT_MASK as u8);
    bits = bits >> CSS_HEIGHT_SHIFT;  

    if ( (bits&0x3) == (CSS_HEIGHT_SET as u8) ) {
        *length = style.height;
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    (bits&0x3)
}

pub fn css_computed_line_height(style: &~css_computed_style , length: &mut i32 , unit: &mut css_unit)
                        -> u8 {

    let mut bits : u8 = style.bits[CSS_LINE_HEIGHT_INDEX];
    bits = bits & (CSS_LINE_HEIGHT_MASK as u8);
    bits = bits >> CSS_LINE_HEIGHT_SHIFT;  

    if ( (bits&0x3) == (CSS_LINE_HEIGHT_NUMBER as u8) || 
         (bits&0x3) == (CSS_LINE_HEIGHT_DIMENSION as u8)) {
        *length = style.line_height;
    }

    if ( (bits&0x3) == (CSS_LINE_HEIGHT_DIMENSION as u8) ) {
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    return (bits&0x3);
}

pub fn css_computed_background_color(style: &~css_computed_style , color: &mut u32)
                                    -> u8 {

    let mut bits : u8 = style.bits[CSS_BACKGROUND_COLOR_INDEX];
    bits = bits & (CSS_BACKGROUND_COLOR_MASK as u8);
    bits = bits >> CSS_BACKGROUND_COLOR_SHIFT; 
    *color = style.background_color;
    return bits;
}

pub fn css_computed_z_index(style: &~css_computed_style , length: &mut i32)
                            -> u8 {

    let mut bits : u8 = style.bits[CSS_Z_INDEX_INDEX];
    bits = bits & (CSS_Z_INDEX_MASK as u8);
    bits = bits >> CSS_Z_INDEX_SHIFT; 
    *length = style.z_index;
    bits
}

pub fn css_computed_margin_top(style: &~css_computed_style , length: &mut i32 , unit: &mut css_unit)
                        -> u8 {

    let mut bits : u8 = style.bits[CSS_MARGIN_TOP_INDEX];
    bits = bits & (CSS_MARGIN_TOP_MASK as u8);
    bits = bits >> CSS_MARGIN_TOP_SHIFT;  

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        *length = style.margin[0];
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    return (bits&0x3)
}

pub fn css_computed_margin_right(style: &~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                        -> u8 {

    let mut bits : u8 = style.bits[CSS_MARGIN_RIGHT_INDEX];
    bits = bits & (CSS_MARGIN_RIGHT_MASK as u8);
    bits = bits >> CSS_MARGIN_RIGHT_SHIFT;  

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        *length = style.margin[1];
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    (bits&0x3)
}

pub fn css_computed_margin_bottom(style: &~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                        -> u8 {

    let mut bits : u8 = style.bits[CSS_MARGIN_BOTTOM_INDEX];
    bits = bits & (CSS_MARGIN_BOTTOM_MASK as u8);
    bits = bits >> CSS_MARGIN_BOTTOM_SHIFT;  

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        *length = style.margin[2];
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    (bits&0x3)
}

pub fn css_computed_margin_left(style: &~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                        -> u8 {

    let mut bits : u8 = style.bits[CSS_MARGIN_LEFT_INDEX];
    bits = bits & (CSS_MARGIN_LEFT_MASK as u8);
    bits = bits >> CSS_MARGIN_LEFT_SHIFT;  

    if ( (bits&0x3) == (CSS_MARGIN_SET as u8) ) {
        *length = style.margin[3];
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    (bits&0x3)
}

pub fn css_computed_background_attachment(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BACKGROUND_ATTACHMENT_INDEX];
    bits = bits & (CSS_BACKGROUND_ATTACHMENT_MASK as u8);
    bits = bits >> CSS_BACKGROUND_ATTACHMENT_SHIFT;      

    bits
}

pub fn css_computed_border_collapse(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_COLLAPSE_INDEX];
    bits = bits & (CSS_BORDER_COLLAPSE_MASK as u8);
    bits = bits >> CSS_BORDER_COLLAPSE_SHIFT;      

    bits
}

pub fn css_computed_caption_side(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_CAPTION_SIDE_INDEX];
    bits = bits & (CSS_CAPTION_SIDE_MASK as u8);
    bits = bits >> CSS_CAPTION_SIDE_SHIFT;      

    bits
}

pub fn css_computed_direction(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_DIRECTION_INDEX];
    bits = bits & (CSS_DIRECTION_MASK as u8);
    bits = bits >> CSS_DIRECTION_SHIFT;      

    bits
}

pub fn css_computed_max_height(style: &~css_computed_style , length: &mut i32 , unit: &mut css_unit)
                        -> u8{

    let mut bits : u8 = style.bits[CSS_MAX_HEIGHT_INDEX];
    bits = bits & (CSS_MAX_HEIGHT_MASK as u8);
    bits = bits >> CSS_MAX_HEIGHT_SHIFT;  

    if ( (bits&0x3) == (CSS_MAX_HEIGHT_SET as u8) ) {
        *length = style.max_height;
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    return (bits&0x3);
}

pub fn css_computed_max_width(style: &~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                        -> u8{

    let mut bits : u8 = style.bits[CSS_MAX_WIDTH_INDEX];
    bits = bits & (CSS_MAX_WIDTH_MASK as u8);
    bits = bits >> CSS_MAX_WIDTH_SHIFT;  

    if ( (bits&0x3) == (CSS_MAX_WIDTH_SET as u8) ) {
        *length = style.max_width;
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    return (bits&0x3);
}

pub fn css_computed_width(style: &~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                        -> u8{

    let mut bits : u8 = style.bits[CSS_WIDTH_INDEX];
    bits = bits & (CSS_WIDTH_MASK as u8);
    bits = bits >> CSS_WIDTH_SHIFT;  

    if ( (bits&0x3) == (CSS_WIDTH_SET as u8) ) {
        *length = style.width;
        *unit = unsafe { transmute((bits >> 2) as int)};
    }

    (bits&0x3)
}

pub fn css_computed_empty_cells(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_EMPTY_CELLS_INDEX];
    bits = bits & (CSS_EMPTY_CELLS_MASK as u8);
    bits = bits >> CSS_EMPTY_CELLS_SHIFT;      

    bits
}

pub fn css_computed_float(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_FLOAT_INDEX];
    bits = bits & (CSS_FLOAT_MASK as u8);
    bits = bits >> CSS_FLOAT_SHIFT;   

    if ( css_computed_position(style) == (CSS_POSITION_ABSOLUTE as u8) ||
            css_computed_position(style) == (CSS_POSITION_FIXED as u8) ) {
        return (CSS_FLOAT_NONE as u8);
    }

    bits
}

pub fn css_computed_font_style(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_FONT_STYLE_INDEX];
    bits = bits & (CSS_FONT_STYLE_MASK as u8);
    bits = bits >> CSS_FONT_STYLE_SHIFT;   

    bits
}

pub fn css_computed_min_height(style:&~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                            -> u8 {

    let mut bits : u8 = style.bits[CSS_MIN_HEIGHT_INDEX];
    bits = bits & (CSS_MIN_HEIGHT_MASK as u8);
    bits = bits >> CSS_MIN_HEIGHT_SHIFT;  

    if ( (bits&0x1) == (CSS_MIN_HEIGHT_SET as u8) ) {
        *length = style.min_height;
        *unit = unsafe { transmute((bits >> 1) as int)};
    }

    (bits&0x1)
}

pub fn css_computed_min_width(style:&~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                            -> u8{

    let mut bits : u8 = style.bits[CSS_MIN_WIDTH_INDEX];
    bits = bits & (CSS_MIN_WIDTH_MASK as u8);
    bits = bits >> CSS_MIN_WIDTH_SHIFT;  

    if ( (bits&0x1) == (CSS_MIN_WIDTH_SET as u8) ) {
        *length = style.min_width;
        *unit = unsafe { transmute((bits >> 1) as int)};
    }

    return (bits&0x1);
}

pub fn css_computed_background_repeat(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BACKGROUND_REPEAT_INDEX];
    bits = bits & (CSS_BACKGROUND_REPEAT_MASK as u8);
    bits = bits >> CSS_BACKGROUND_REPEAT_SHIFT;   

    bits
}

pub fn css_computed_clear(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_CLEAR_INDEX];
    bits = bits & (CSS_CLEAR_MASK as u8);
    bits = bits >> CSS_CLEAR_SHIFT;   

    bits
}

pub fn css_computed_padding_top(style:&~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                            -> u8{

    let mut bits : u8 = style.bits[CSS_PADDING_TOP_INDEX];
    bits = bits & (CSS_PADDING_TOP_MASK as u8);
    bits = bits >> CSS_PADDING_TOP_SHIFT;  

    if ( (bits&0x1) == (CSS_PADDING_SET as u8) ) {
        *length = style.padding[0];
        *unit = unsafe { transmute((bits >> 1) as int)};
    }

    return (bits&0x1);
}

pub fn css_computed_padding_right(style:&~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                            -> u8{

    let mut bits : u8 = style.bits[CSS_PADDING_RIGHT_INDEX];
    bits = bits & (CSS_PADDING_RIGHT_MASK as u8);
    bits = bits >> CSS_PADDING_RIGHT_SHIFT;  

    if ( (bits&0x1) == (CSS_PADDING_SET as u8) ) {
        *length = style.padding[1];
        *unit = unsafe { transmute((bits >> 1) as int)};
    }

    return (bits&0x1);
}

pub fn css_computed_padding_bottom(style:&~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                            -> u8{

    let mut bits : u8 = style.bits[CSS_PADDING_BOTTOM_INDEX];
    bits = bits & (CSS_PADDING_BOTTOM_MASK as u8);
    bits = bits >> CSS_PADDING_BOTTOM_SHIFT;  

    if ( (bits&0x1) == (CSS_PADDING_SET as u8) ) {
        *length = style.padding[2];
        *unit = unsafe { transmute((bits >> 1) as int)};
    }

    return (bits&0x1);
}

pub fn css_computed_padding_left(style:&~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                            -> u8{

    let mut bits : u8 = style.bits[CSS_PADDING_LEFT_INDEX];
    bits = bits & (CSS_PADDING_LEFT_MASK as u8);
    bits = bits >> CSS_PADDING_LEFT_SHIFT;  

    if ( (bits&0x1) == (CSS_PADDING_SET as u8) ) {
        *length = style.padding[3];
        *unit = unsafe { transmute((bits >> 1) as int)};
    }

    return (bits&0x1);
}

pub fn css_computed_overflow(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_OVERFLOW_INDEX];
    bits = bits & (CSS_OVERFLOW_MASK as u8);
    bits = bits >> CSS_OVERFLOW_SHIFT;   

    bits
}

pub fn css_computed_position(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_POSITION_INDEX];
    bits = bits & (CSS_POSITION_MASK as u8);
    bits = bits >> CSS_POSITION_SHIFT;   

    bits
}

pub fn css_computed_opacity(style:&~css_computed_style , opacity: &mut i32)
                            -> u8 {

    let mut bits : u8 = style.bits[CSS_OPACITY_INDEX];
    bits = bits & (CSS_OPACITY_MASK as u8);
    bits = bits >> CSS_OPACITY_SHIFT;  

    if ( (bits&0x1) == (CSS_OPACITY_SET as u8) ) {
        *opacity = style.opacity;
    }

    (bits&0x1)
}

pub fn css_computed_text_transform(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_TEXT_TRANSFORM_INDEX];
    bits = bits & (CSS_TEXT_TRANSFORM_MASK as u8);
    bits = bits >> CSS_TEXT_TRANSFORM_SHIFT;   

    bits
}

pub fn css_computed_text_indent(style:&~css_computed_style, length: &mut i32 , unit: &mut css_unit)
                            -> u8{

    let mut bits : u8 = style.bits[CSS_TEXT_INDENT_INDEX];
    bits = bits & (CSS_TEXT_INDENT_MASK as u8);
    bits = bits >> CSS_TEXT_INDENT_SHIFT;  

    if ( (bits&0x1) == (CSS_TEXT_INDENT_SET as u8) ) {
        *length = style.text_indent;
        *unit = unsafe { transmute((bits >> 1) as int)};
    }

    return (bits&0x1);
}

pub fn css_computed_white_space(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_WHITE_SPACE_INDEX];
    bits = bits & (CSS_WHITE_SPACE_MASK as u8);
    bits = bits >> CSS_WHITE_SPACE_SHIFT;   

    bits
}

pub fn css_computed_background_position(
                    style : &~css_computed_style , hlength: &mut i32 , hunit: &mut css_unit , vlength: &mut i32 , vunit: &mut css_unit) 
                    -> u8 {

    let mut bits = style.bits[CSS_BACKGROUND_POSITION_INDEX];
    bits = bits & (CSS_BACKGROUND_POSITION_MASK as u8);
    bits = bits >> CSS_BACKGROUND_POSITION_SHIFT ;

    if ( bits == (CSS_BACKGROUND_POSITION_SET as u8) ) {

        let mut bits1 = style.bits[CSS_BACKGROUND_POSITION_INDEX1];
        bits1 = bits1 & (CSS_BACKGROUND_POSITION_MASK1 as u8);
        bits1 = bits1 >> CSS_BACKGROUND_POSITION_SHIFT1 ;

        *hlength = style.background_position[0];
        *hunit = unsafe { transmute((bits1 >> 4)as int) } ;

        *vlength = style.background_position[1];
        *vunit = unsafe { transmute((bits1 & 0xf)as int) } ;
        
    }
    bits
}

pub fn css_computed_display(style: &~css_computed_style,
                        root: bool) -> u8 {

    let mut position: u8 ;
    let mut bits = style.bits[CSS_DISPLAY_INDEX];
    println(fmt!("css_computed_display :: bits == %? " , bits));
    println(fmt!("css_computed_display :: bits == %? " , style.bits[32]));
    bits = bits & (CSS_DISPLAY_MASK as u8);
    bits = bits >> CSS_DISPLAY_SHIFT ;

    println(fmt!("css_computed_display :: after mask and shift bits == %? " , bits));
    position = css_computed_position(style);
    println(fmt!("css_computed_display :: position == %? " , position));
    if ( bits == (CSS_DISPLAY_NONE as u8) ) {
        return bits;
    }

    if ( (position == (CSS_POSITION_ABSOLUTE as u8)) || 
            (position == (CSS_POSITION_FIXED as u8))  ||
            (css_computed_float(style) != (CSS_FLOAT_NONE as u8)) ||
            (root == true) ) {
        println("css_computed_display :: in ifposition == (CSS_POSITION_ABSOLUTE as u8 ");

        if ( bits == (CSS_DISPLAY_INLINE_TABLE as u8) ) {
            println("css_computed_display :: in if   bits == (CSS_DISPLAY_INLINE_TABLE as u8");
            return (CSS_DISPLAY_TABLE as u8);
        } 
        else if (bits == (CSS_DISPLAY_INLINE as u8) ||
                bits == (CSS_DISPLAY_RUN_IN as u8) ||
                bits == (CSS_DISPLAY_TABLE_ROW_GROUP as u8) ||
                bits == (CSS_DISPLAY_TABLE_COLUMN as u8) ||
                bits == (CSS_DISPLAY_TABLE_COLUMN_GROUP as u8) ||
                bits == (CSS_DISPLAY_TABLE_HEADER_GROUP as u8) ||
                bits == (CSS_DISPLAY_TABLE_FOOTER_GROUP as u8) ||
                bits == (CSS_DISPLAY_TABLE_ROW as u8) ||
                bits == (CSS_DISPLAY_TABLE_CELL as u8) ||
                bits == (CSS_DISPLAY_TABLE_CAPTION as u8) ||
                bits == (CSS_DISPLAY_INLINE_BLOCK as u8)) {
            println("css_computed_display :: in ele if");
            return (CSS_DISPLAY_BLOCK as u8);
        }
    }

    return bits;
}

pub fn css_computed_display_static(style:&~css_computed_style)
                                        -> u8 {
    
    println("css_computed_display_static");
    let mut bits : u8 = style.bits[CSS_DISPLAY_INDEX];
    println(fmt!("css_computed_display_static :: bits == %? " , bits));

    bits = bits & (CSS_DISPLAY_MASK as u8);
    bits = bits >> CSS_DISPLAY_SHIFT;   
    println(fmt!("css_computed_display_static :: bits after mask and shift  = %? " , bits));

    bits
}

pub fn css_computed_font_variant(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_FONT_VARIANT_INDEX];
    bits = bits & (CSS_FONT_VARIANT_MASK as u8);
    bits = bits >> CSS_FONT_VARIANT_SHIFT;   

    bits
}

pub fn css_computed_text_decoration(style:&~css_computed_style)
                                        -> u8 {
    // println(fmt!("css_computed_text_decoration"));
    let mut bits : u8 = style.bits[CSS_TEXT_DECORATION_INDEX];
    // println(fmt!("css_computed_text_decoration :: bits == %? " , bits));
    bits = bits & (CSS_TEXT_DECORATION_MASK as u8);

    bits = bits >> CSS_TEXT_DECORATION_SHIFT;   
    // println(fmt!("css_computed_text_decoration :: bits ater mask and shift == %? " , bits));
    bits
}


pub fn css_computed_font_family(style:&~css_computed_style , names: &mut ~[uint])
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_FONT_FAMILY_INDEX];
    // println(fmt!("css_computed_font_family :: bits == %? " , bits));
    bits = bits & (CSS_FONT_FAMILY_MASK as u8);
    // println(fmt!("css_computed_font_family :: bits after mask == %? " , bits));
    bits = bits >> CSS_FONT_FAMILY_SHIFT;
    // println(fmt!("css_computed_font_family :: bits after mask and shift == %? " , bits));
    *names = style.font_family.clone();
    bits
}

pub fn css_computed_border_top_style(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_TOP_STYLE_INDEX];
    bits = bits & (CSS_BORDER_TOP_STYLE_MASK as u8);
    bits = bits >> CSS_BORDER_TOP_STYLE_SHIFT;   

    bits
}

pub fn css_computed_border_right_style(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_RIGHT_STYLE_INDEX];
    bits = bits & (CSS_BORDER_RIGHT_STYLE_MASK as u8);
    bits = bits >> CSS_BORDER_RIGHT_STYLE_SHIFT;   

    bits
}

pub fn css_computed_border_bottom_style(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_BOTTOM_STYLE_INDEX];
    bits = bits & (CSS_BORDER_BOTTOM_STYLE_MASK as u8);
    bits = bits >> CSS_BORDER_BOTTOM_STYLE_SHIFT;   

    bits
}

pub fn css_computed_border_left_style(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_BORDER_LEFT_STYLE_INDEX];
    bits = bits & (CSS_BORDER_LEFT_STYLE_MASK as u8);
    bits = bits >> CSS_BORDER_LEFT_STYLE_SHIFT;   

    bits
}

pub fn css_computed_font_weight(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_FONT_WEIGHT_INDEX];
    bits = bits & (CSS_FONT_WEIGHT_MASK as u8);
    bits = bits >> CSS_FONT_WEIGHT_SHIFT;   

    bits
}

pub fn css_computed_list_style_type(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_LIST_STYLE_TYPE_INDEX];
    bits = bits & (CSS_LIST_STYLE_TYPE_MASK as u8);
    bits = bits >> CSS_LIST_STYLE_TYPE_SHIFT;   

    bits
}

pub fn css_computed_outline_style(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_OUTLINE_STYLE_INDEX];
    bits = bits & (CSS_OUTLINE_STYLE_MASK as u8);
    bits = bits >> CSS_OUTLINE_STYLE_SHIFT;   

    bits
}

pub fn css_computed_table_layout(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_TABLE_LAYOUT_INDEX];
    bits = bits & (CSS_TABLE_LAYOUT_MASK as u8);
    bits = bits >> CSS_TABLE_LAYOUT_SHIFT;   

    bits
}

pub fn css_computed_unicode_bidi(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_UNICODE_BIDI_INDEX];
    bits = bits & (CSS_UNICODE_BIDI_MASK as u8);
    bits = bits >> CSS_UNICODE_BIDI_SHIFT;   

    bits
}

pub fn css_computed_visibility(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_VISIBILITY_INDEX];
    bits = bits & (CSS_VISIBILITY_MASK as u8);
    bits = bits >> CSS_VISIBILITY_SHIFT;   

    bits
}

pub fn css_computed_list_style_position(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_LIST_STYLE_POSITION_INDEX];
    bits = bits & (CSS_LIST_STYLE_POSITION_MASK as u8);
    bits = bits >> CSS_LIST_STYLE_POSITION_SHIFT;   

    bits
}


pub fn css_computed_text_align(style:&~css_computed_style)
                                        -> u8 {

    let mut bits : u8 = style.bits[CSS_TEXT_ALIGN_INDEX];
    bits = bits & (CSS_TEXT_ALIGN_MASK as u8);
    bits = bits >> CSS_TEXT_ALIGN_SHIFT;   

    bits
}

pub fn css_computed_page_break_after(style:&~css_computed_style)
                                        -> u8 {

    if style.page.is_some() {
        let mut bits : u8 = style.page.get_ref().bits[CSS_PAGE_BREAK_AFTER_INDEX];
        bits = bits & (CSS_PAGE_BREAK_AFTER_MASK as u8);
        bits = bits >> CSS_PAGE_BREAK_AFTER_SHIFT;   
        bits
    }
    else {
        (CSS_PAGE_BREAK_AFTER_AUTO as u8)
    }
}

pub fn css_computed_page_break_before(style:&~css_computed_style)
                                        -> u8 {

    if style.page.is_some() {
        let mut bits : u8 = style.page.get_ref().bits[CSS_PAGE_BREAK_BEFORE_INDEX];
        bits = bits & (CSS_PAGE_BREAK_BEFORE_MASK as u8);
        bits = bits >> CSS_PAGE_BREAK_BEFORE_SHIFT;   
        bits
    }
    else {
        (CSS_PAGE_BREAK_BEFORE_AUTO as u8)
    }
}

pub fn css_computed_page_break_inside(style:&~css_computed_style)
                                        -> u8 {

    if style.page.is_some() {
        let mut bits : u8 = style.page.get_ref().bits[CSS_PAGE_BREAK_INSIDE_INDEX];
        bits = bits & (CSS_PAGE_BREAK_INSIDE_MASK as u8);
        bits = bits >> CSS_PAGE_BREAK_INSIDE_SHIFT;   
        bits
    }
    else {
        (CSS_PAGE_BREAK_INSIDE_AUTO as u8)
    }
}

//////////////////////////////////////////////////////////////////////

