
use std::libc::types::common::c99::uint32_t;

pub mod stylesheet {

    pub type css_fixed = int;

}

pub mod types {
}

pub mod errors {
}

pub mod hint {
}


pub mod properties {
	use std::libc::types::common::c99::uint32_t;
//    use ll::c_enum;

    pub type css_properties_e = uint32_t;
    
    pub static CSS_PROP_AZIMUTH: css_properties_e			= 0x000;
    pub static CSS_PROP_BACKGROUND_ATTACHMENT: css_properties_e		= 0x001;
    pub static CSS_PROP_BACKGROUND_COLOR: css_properties_e		= 0x002;
    pub static CSS_PROP_BACKGROUND_IMAGE: css_properties_e		= 0x003;
    pub static CSS_PROP_BACKGROUND_POSITION: css_properties_e		= 0x004;
    pub static CSS_PROP_BACKGROUND_REPEAT: css_properties_e		= 0x005;
    pub static CSS_PROP_BORDER_COLLAPSE: css_properties_e		= 0x006;
    pub static CSS_PROP_BORDER_SPACING: css_properties_e			= 0x007;
    pub static CSS_PROP_BORDER_TOP_COLOR: css_properties_e		= 0x008;
    pub static CSS_PROP_BORDER_RIGHT_COLOR: css_properties_e		= 0x009;
    pub static CSS_PROP_BORDER_BOTTOM_COLOR: css_properties_e		= 0x00a;
    pub static CSS_PROP_BORDER_LEFT_COLOR: css_properties_e		= 0x00b;
    pub static CSS_PROP_BORDER_TOP_STYLE: css_properties_e		= 0x00c;
    pub static CSS_PROP_BORDER_RIGHT_STYLE: css_properties_e		= 0x00d;
    pub static CSS_PROP_BORDER_BOTTOM_STYLE: css_properties_e		= 0x00e;
    pub static CSS_PROP_BORDER_LEFT_STYLE: css_properties_e		= 0x00f;
    pub static CSS_PROP_BORDER_TOP_WIDTH: css_properties_e		= 0x010;
    pub static CSS_PROP_BORDER_RIGHT_WIDTH: css_properties_e		= 0x011;
    pub static CSS_PROP_BORDER_BOTTOM_WIDTH: css_properties_e		= 0x012;
    pub static CSS_PROP_BORDER_LEFT_WIDTH: css_properties_e		= 0x013;
    pub static CSS_PROP_BOTTOM: css_properties_e				= 0x014;
    pub static CSS_PROP_CAPTION_SIDE: css_properties_e			= 0x015;
    pub static CSS_PROP_CLEAR: css_properties_e				= 0x016;
    pub static CSS_PROP_CLIP: css_properties_e				= 0x017;
    pub static CSS_PROP_COLOR: css_properties_e				= 0x018;
    pub static CSS_PROP_CONTENT: css_properties_e			= 0x019;
    pub static CSS_PROP_COUNTER_INCREMENT: css_properties_e		= 0x01a;
    pub static CSS_PROP_COUNTER_RESET: css_properties_e			= 0x01b;
    pub static CSS_PROP_CUE_AFTER: css_properties_e			= 0x01c;
    pub static CSS_PROP_CUE_BEFORE: css_properties_e			= 0x01d;
    pub static CSS_PROP_CURSOR: css_properties_e				= 0x01e;
    pub static CSS_PROP_DIRECTION: css_properties_e			= 0x01f;
    pub static CSS_PROP_DISPLAY: css_properties_e			= 0x020;
    pub static CSS_PROP_ELEVATION: css_properties_e			= 0x021;
    pub static CSS_PROP_EMPTY_CELLS: css_properties_e			= 0x022;
    pub static CSS_PROP_FLOAT: css_properties_e				= 0x023;
    pub static CSS_PROP_FONT_FAMILY: css_properties_e			= 0x024;
    pub static CSS_PROP_FONT_SIZE: css_properties_e			= 0x025;
    pub static CSS_PROP_FONT_STYLE: css_properties_e			= 0x026;
    pub static CSS_PROP_FONT_VARIANT: css_properties_e			= 0x027;
    pub static CSS_PROP_FONT_WEIGHT: css_properties_e			= 0x028;
    pub static CSS_PROP_HEIGHT: css_properties_e				= 0x029;
    pub static CSS_PROP_LEFT: css_properties_e				= 0x02a;
    pub static CSS_PROP_LETTER_SPACING: css_properties_e			= 0x02b;
    pub static CSS_PROP_LINE_HEIGHT: css_properties_e			= 0x02c;
    pub static CSS_PROP_LIST_STYLE_IMAGE: css_properties_e		= 0x02d;
    pub static CSS_PROP_LIST_STYLE_POSITION: css_properties_e		= 0x02e;
    pub static CSS_PROP_LIST_STYLE_TYPE: css_properties_e		= 0x02f;
    pub static CSS_PROP_MARGIN_TOP: css_properties_e			= 0x030;
    pub static CSS_PROP_MARGIN_RIGHT: css_properties_e			= 0x031;
    pub static CSS_PROP_MARGIN_BOTTOM: css_properties_e			= 0x032;
    pub static CSS_PROP_MARGIN_LEFT: css_properties_e			= 0x033;
    pub static CSS_PROP_MAX_HEIGHT: css_properties_e			= 0x034;
    pub static CSS_PROP_MAX_WIDTH: css_properties_e			= 0x035;
    pub static CSS_PROP_MIN_HEIGHT: css_properties_e			= 0x036;
    pub static CSS_PROP_MIN_WIDTH: css_properties_e			= 0x037;
    pub static CSS_PROP_ORPHANS: css_properties_e			= 0x038;
    pub static CSS_PROP_OUTLINE_COLOR: css_properties_e			= 0x039;
    pub static CSS_PROP_OUTLINE_STYLE: css_properties_e			= 0x03a;
    pub static CSS_PROP_OUTLINE_WIDTH: css_properties_e			= 0x03b;
    pub static CSS_PROP_OVERFLOW: css_properties_e			= 0x03c;
    pub static CSS_PROP_PADDING_TOP: css_properties_e			= 0x03d;
    pub static CSS_PROP_PADDING_RIGHT: css_properties_e			= 0x03e;
    pub static CSS_PROP_PADDING_BOTTOM: css_properties_e			= 0x03f;
    pub static CSS_PROP_PADDING_LEFT: css_properties_e			= 0x040;
    pub static CSS_PROP_PAGE_BREAK_AFTER: css_properties_e		= 0x041;
    pub static CSS_PROP_PAGE_BREAK_BEFORE: css_properties_e		= 0x042;
    pub static CSS_PROP_PAGE_BREAK_INSIDE: css_properties_e		= 0x043;
    pub static CSS_PROP_PAUSE_AFTER: css_properties_e			= 0x044;
    pub static CSS_PROP_PAUSE_BEFORE: css_properties_e			= 0x045;
    pub static CSS_PROP_PITCH_RANGE: css_properties_e			= 0x046;
    pub static CSS_PROP_PITCH: css_properties_e				= 0x047;
    pub static CSS_PROP_PLAY_DURING: css_properties_e			= 0x048;
    pub static CSS_PROP_POSITION: css_properties_e			= 0x049;
    pub static CSS_PROP_QUOTES: css_properties_e				= 0x04a;
    pub static CSS_PROP_RICHNESS: css_properties_e			= 0x04b;
    pub static CSS_PROP_RIGHT: css_properties_e				= 0x04c;
    pub static CSS_PROP_SPEAK_HEADER: css_properties_e			= 0x04d;
    pub static CSS_PROP_SPEAK_NUMERAL: css_properties_e			= 0x04e;
    pub static CSS_PROP_SPEAK_PUNCTUATION: css_properties_e		= 0x04f;
    pub static CSS_PROP_SPEAK: css_properties_e				= 0x050;
    pub static CSS_PROP_SPEECH_RATE: css_properties_e			= 0x051;
    pub static CSS_PROP_STRESS: css_properties_e				= 0x052;
    pub static CSS_PROP_TABLE_LAYOUT: css_properties_e			= 0x053;
    pub static CSS_PROP_TEXT_ALIGN: css_properties_e			= 0x054;
    pub static CSS_PROP_TEXT_DECORATION: css_properties_e		= 0x055;
    pub static CSS_PROP_TEXT_INDENT: css_properties_e			= 0x056;
    pub static CSS_PROP_TEXT_TRANSFORM: css_properties_e			= 0x057;
    pub static CSS_PROP_TOP: css_properties_e				= 0x058;
    pub static CSS_PROP_UNICODE_BIDI: css_properties_e			= 0x059;
    pub static CSS_PROP_VERTICAL_ALIGN: css_properties_e			= 0x05a;
    pub static CSS_PROP_VISIBILITY: css_properties_e			= 0x05b;
    pub static CSS_PROP_VOICE_FAMILY: css_properties_e			= 0x05c;
    pub static CSS_PROP_VOLUME: css_properties_e				= 0x05d;
    pub static CSS_PROP_WHITE_SPACE: css_properties_e			= 0x05e;
    pub static CSS_PROP_WIDOWS: css_properties_e				= 0x05f;
    pub static CSS_PROP_WIDTH: css_properties_e				= 0x060;
    pub static CSS_PROP_WORD_SPACING: css_properties_e			= 0x061;
    pub static CSS_PROP_Z_INDEX: css_properties_e			= 0x062;
    pub static CSS_PROP_OPACITY: css_properties_e			= 0x063;
    pub static CSS_PROP_BREAK_AFTER: css_properties_e			= 0x064;
    pub static CSS_PROP_BREAK_BEFORE: css_properties_e			= 0x065;
    pub static CSS_PROP_BREAK_INSIDE: css_properties_e			= 0x066;
    pub static CSS_PROP_COLUMN_COUNT: css_properties_e			= 0x067;
    pub static CSS_PROP_COLUMN_FILL: css_properties_e			= 0x068;
    pub static CSS_PROP_COLUMN_GAP: css_properties_e			= 0x069;
    pub static CSS_PROP_COLUMN_RULE_COLOR: css_properties_e		= 0x06a;
    pub static CSS_PROP_COLUMN_RULE_STYLE: css_properties_e		= 0x06b;
    pub static CSS_PROP_COLUMN_RULE_WIDTH: css_properties_e		= 0x06c;
    pub static CSS_PROP_COLUMN_SPAN: css_properties_e			= 0x06d;
    pub static CSS_PROP_COLUMN_WIDTH: css_properties_e			= 0x06e;

    pub type css_font_style_e = uint32_t;

    pub static CSS_FONT_STYLE_INHERIT: css_font_style_e = 0;
    pub static CSS_FONT_STYLE_NORMAL: css_font_style_e = 1;
    pub static CSS_FONT_STYLE_ITALIC: css_font_style_e = 2;
    pub static CSS_FONT_STYLE_OBLIQUE: css_font_style_e = 3;

    pub type css_font_variant_e = uint32_t;

    pub static CSS_FONT_VARIANT_INHERIT: css_font_variant_e = 0;
    pub static CSS_FONT_VARIANT_NORMAL: css_font_variant_e = 1;
    pub static CSS_FONT_VARIANT_SMALL_CAPS: css_font_variant_e = 2;

    pub type css_font_weight_e = uint32_t;

    pub static CSS_FONT_WEIGHT_INHERIT: css_font_weight_e = 0x0;
    pub static CSS_FONT_WEIGHT_NORMAL: css_font_weight_e = 0x1;
    pub static CSS_FONT_WEIGHT_BOLD: css_font_weight_e = 0x2;
    pub static CSS_FONT_WEIGHT_BOLDER: css_font_weight_e = 0x3;
    pub static CSS_FONT_WEIGHT_LIGHTER: css_font_weight_e = 0x4;
    pub static CSS_FONT_WEIGHT_100: css_font_weight_e = 0x5;
    pub static CSS_FONT_WEIGHT_200: css_font_weight_e = 0x6;
    pub static CSS_FONT_WEIGHT_300: css_font_weight_e = 0x7;
    pub static CSS_FONT_WEIGHT_400: css_font_weight_e = 0x8;
    pub static CSS_FONT_WEIGHT_500: css_font_weight_e = 0x9;
    pub static CSS_FONT_WEIGHT_600: css_font_weight_e = 0xa;
    pub static CSS_FONT_WEIGHT_700: css_font_weight_e = 0xb;
    pub static CSS_FONT_WEIGHT_800: css_font_weight_e = 0xc;
    pub static CSS_FONT_WEIGHT_900: css_font_weight_e = 0xd;

    pub type css_font_family_e = uint32_t;

    pub static CSS_FONT_FAMILY_INHERIT: css_font_family_e			= 0x0;
    pub static CSS_FONT_FAMILY_SERIF: css_font_family_e			= 0x1;
    pub static CSS_FONT_FAMILY_SANS_SERIF: css_font_family_e		= 0x2;
    pub static CSS_FONT_FAMILY_CURSIVE: css_font_family_e			= 0x3;
    pub static CSS_FONT_FAMILY_FANTASY: css_font_family_e			= 0x4;
    pub static CSS_FONT_FAMILY_MONOSPACE: css_font_family_e		= 0x5;

    pub type css_quotes_e = uint32_t;

    pub static CSS_QUOTES_INHERIT: css_quotes_e			= 0x0;
    /* Consult pointer in struct to determine which */
    pub static CSS_QUOTES_STRING: css_quotes_e			= 0x1;
    pub static CSS_QUOTES_NONE: css_quotes_e				= 0x1;

    pub type css_color_e = uint32_t;

    pub static CSS_COLOR_INHERIT: css_color_e = 0x0;
    pub static CSS_COLOR_COLOR: css_color_e = 0x1;

    pub type css_border_style_e = uint32_t;

    pub static CSS_BORDER_STYLE_INHERIT: css_border_style_e = 0x0;
    pub static CSS_BORDER_STYLE_NONE: css_border_style_e = 0x1;
    pub static CSS_BORDER_STYLE_HIDDEN: css_border_style_e = 0x2;
    pub static CSS_BORDER_STYLE_DOTTED: css_border_style_e = 0x3;
    pub static CSS_BORDER_STYLE_DASHED: css_border_style_e = 0x4;
    pub static CSS_BORDER_STYLE_SOLID: css_border_style_e = 0x5;
    pub static CSS_BORDER_STYLE_DOUBLE: css_border_style_e = 0x6;
    pub static CSS_BORDER_STYLE_GROOVE: css_border_style_e = 0x7;
    pub static CSS_BORDER_STYLE_RIDGE: css_border_style_e = 0x8;
    pub static CSS_BORDER_STYLE_INSET: css_border_style_e = 0x9;
    pub static CSS_BORDER_STYLE_OUTSET: css_border_style_e = 0xa;

    pub type css_border_width_e = uint32_t;

    pub static CSS_BORDER_WIDTH_INHERIT: css_border_width_e = 0x0;
    pub static CSS_BORDER_WIDTH_THIN: css_border_width_e = 0x1;
    pub static CSS_BORDER_WIDTH_MEDIUM: css_border_width_e = 0x2;
    pub static CSS_BORDER_WIDTH_THICK: css_border_width_e = 0x3;
    pub static CSS_BORDER_WIDTH_WIDTH: css_border_width_e = 0x4;

    pub type css_margin_e = uint32_t;

    pub static CSS_MARGIN_INHERIT: css_margin_e = 0x0;
    pub static CSS_MARGIN_SET: css_margin_e = 0x1;
    pub static CSS_MARGIN_AUTO: css_margin_e = 0x2;

    pub type css_padding_e = uint32_t;

    pub static CSS_PADDING_INHERIT: css_padding_e = 0x0;
    pub static CSS_PADDING_SET: css_padding_e = 0x1;

    pub type css_display_e = uint32_t;
    
    pub static CSS_DISPLAY_INHERIT: css_display_e = 0x00;
    pub static CSS_DISPLAY_INLINE: css_display_e = 0x01;
    pub static CSS_DISPLAY_BLOCK: css_display_e = 0x02;
    pub static CSS_DISPLAY_LIST_ITEM: css_display_e = 0x03;
    pub static CSS_DISPLAY_RUN_IN: css_display_e = 0x04;
    pub static CSS_DISPLAY_INLINE_BLOCK: css_display_e = 0x05;
    pub static CSS_DISPLAY_TABLE: css_display_e = 0x06;
    pub static CSS_DISPLAY_INLINE_TABLE: css_display_e = 0x07;
    pub static CSS_DISPLAY_TABLE_ROW_GROUP: css_display_e = 0x08;
    pub static CSS_DISPLAY_TABLE_HEADER_GROUP: css_display_e = 0x09;
    pub static CSS_DISPLAY_TABLE_FOOTER_GROUP: css_display_e = 0x0a;
    pub static CSS_DISPLAY_TABLE_ROW: css_display_e = 0x0b;
    pub static CSS_DISPLAY_TABLE_COLUMN_GROUP: css_display_e = 0x0c;
    pub static CSS_DISPLAY_TABLE_COLUMN: css_display_e = 0x0d;
    pub static CSS_DISPLAY_TABLE_CELL: css_display_e = 0x0e;
    pub static CSS_DISPLAY_TABLE_CAPTION: css_display_e = 0x0f;
    pub static CSS_DISPLAY_NONE: css_display_e = 0x10;

    pub type css_float_e = uint32_t;

    pub static CSS_FLOAT_INHERIT: css_float_e = 0x0;
    pub static CSS_FLOAT_LEFT: css_float_e = 0x1;
    pub static CSS_FLOAT_RIGHT: css_float_e = 0x2;
    pub static CSS_FLOAT_NONE: css_float_e = 0x3;

    pub type css_clear_e = uint32_t;

    pub static CSS_CLEAR_INHERIT: css_clear_e = 0x0;
    pub static CSS_CLEAR_NONE: css_clear_e = 0x1;
    pub static CSS_CLEAR_LEFT: css_clear_e = 0x2;
    pub static CSS_CLEAR_RIGHT: css_clear_e = 0x3;
    pub static CSS_CLEAR_BOTH: css_clear_e = 0x4;

    pub type css_position_e = uint32_t;

    pub static CSS_POSITION_INHERIT: css_position_e = 0x0;
    pub static CSS_POSITION_STATIC: css_position_e = 0x1;
    pub static CSS_POSITION_RELATIVE: css_position_e = 0x2;
    pub static CSS_POSITION_ABSOLUTE: css_position_e = 0x3;
    pub static CSS_POSITION_FIXED: css_position_e = 0x4;

    pub type css_width_e = uint32_t;

    pub static CSS_WIDTH_INHERIT: css_width_e = 0x0;
    pub static CSS_WIDTH_SET: css_width_e = 0x1;
    pub static CSS_WIDTH_AUTO: css_width_e = 0x2;

    pub type css_height_e = uint32_t;

    pub static CSS_HEIGHT_INHERIT: css_height_e = 0x0;
    pub static CSS_HEIGHT_SET: css_height_e = 0x1;
    pub static CSS_HEIGHT_AUTO: css_height_e = 0x2;

    pub type css_font_size_e = uint32_t;

    pub static CSS_FONT_SIZE_INHERIT: css_font_size_e = 0x0;
    pub static CSS_FONT_SIZE_XX_SMALL: css_font_size_e = 0x1;
    pub static CSS_FONT_SIZE_X_SMALL: css_font_size_e = 0x2;
    pub static CSS_FONT_SIZE_SMALL: css_font_size_e = 0x3;
    pub static CSS_FONT_SIZE_MEDIUM: css_font_size_e = 0x4;
    pub static CSS_FONT_SIZE_LARGE: css_font_size_e = 0x5;
    pub static CSS_FONT_SIZE_X_LARGE: css_font_size_e = 0x6;
    pub static CSS_FONT_SIZE_XX_LARGE: css_font_size_e = 0x7;
    pub static CSS_FONT_SIZE_LARGER: css_font_size_e = 0x8;
    pub static CSS_FONT_SIZE_SMALLER: css_font_size_e = 0x9;
    pub static CSS_FONT_SIZE_DIMENSION: css_font_size_e = 0xa;

    pub type css_text_align_e = uint32_t;

    pub static CSS_TEXT_ALIGN_INHERIT: css_text_align_e = 0x0;
    pub static CSS_TEXT_ALIGN_INHERIT_IF_NON_MAGIC: css_text_align_e = 0x1;
    pub static CSS_TEXT_ALIGN_LEFT: css_text_align_e = 0x2;
    pub static CSS_TEXT_ALIGN_RIGHT: css_text_align_e = 0x3;
    pub static CSS_TEXT_ALIGN_CENTER: css_text_align_e = 0x4;
    pub static CSS_TEXT_ALIGN_JUSTIFY: css_text_align_e = 0x5;
    pub static CSS_TEXT_ALIGN_DEFAULT: css_text_align_e = 0x6;
    pub static CSS_TEXT_ALIGN_LIBCSS_LEFT: css_text_align_e = 0x7;
    pub static CSS_TEXT_ALIGN_LIBCSS_CENTER: css_text_align_e = 0x8;
    pub static CSS_TEXT_ALIGN_LIBCSS_RIGHT: css_text_align_e = 0x9;

    pub type css_text_decoration_e = uint32_t;

    pub static CSS_TEXT_DECORATION_INHERIT: css_text_decoration_e = 0x00;
    pub static CSS_TEXT_DECORATION_NONE: css_text_decoration_e = 0x10;
    pub static CSS_TEXT_DECORATION_BLINK: css_text_decoration_e = (1<<3);
    pub static CSS_TEXT_DECORATION_LINE_THROUGH: css_text_decoration_e = (1<<2);
    pub static CSS_TEXT_DECORATION_OVERLINE: css_text_decoration_e = (1<<1);
    pub static CSS_TEXT_DECORATION_UNDERLINE: css_text_decoration_e = (1<<0);

    pub type css_line_height_e = uint32_t;

    pub static CSS_LINE_HEIGHT_INHERIT: css_line_height_e = 0x0;
    pub static CSS_LINE_HEIGHT_NUMBER: css_line_height_e = 0x1;
    pub static CSS_LINE_HEIGHT_DIMENSION: css_line_height_e = 0x2;
    pub static CSS_LINE_HEIGHT_NORMAL: css_line_height_e = 0x3;

    pub type css_vertical_align_e = uint32_t;

    pub static CSS_VERTICAL_ALIGN_INHERIT: css_vertical_align_e = 0x0;
    pub static CSS_VERTICAL_ALIGN_BASELINE: css_vertical_align_e = 0x1;
    pub static CSS_VERTICAL_ALIGN_SUB: css_vertical_align_e = 0x2;
    pub static CSS_VERTICAL_ALIGN_SUPER: css_vertical_align_e = 0x3;
    pub static CSS_VERTICAL_ALIGN_TOP: css_vertical_align_e = 0x4;
    pub static CSS_VERTICAL_ALIGN_TEXTTOP: css_vertical_align_e = 0x5;
    pub static CSS_VERTICAL_ALIGN_MIDDLE: css_vertical_align_e = 0x6;
    pub static CSS_VERTICAL_ALIGN_BOTTOM: css_vertical_align_e = 0x7;
    pub static CSS_VERTICAL_ALIGN_TEXTBOTTOM: css_vertical_align_e = 0x8;
    pub static CSS_VERTICAL_ALIGN_DIMENSION: css_vertical_align_e = 0x9;
}


pub mod select {
}

pub mod computed {
}
