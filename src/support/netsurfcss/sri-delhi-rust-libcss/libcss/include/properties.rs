// properties.h
pub enum css_properties_e {
    CSS_PROP_AZIMUTH            = 0x000,
    CSS_PROP_BACKGROUND_ATTACHMENT      = 0x001,
    CSS_PROP_BACKGROUND_COLOR       = 0x002,
    CSS_PROP_BACKGROUND_IMAGE       = 0x003,
    CSS_PROP_BACKGROUND_POSITION        = 0x004,
    CSS_PROP_BACKGROUND_REPEAT      = 0x005,
    CSS_PROP_BORDER_COLLAPSE        = 0x006,
    CSS_PROP_BORDER_SPACING         = 0x007,
    CSS_PROP_BORDER_TOP_COLOR       = 0x008,
    CSS_PROP_BORDER_RIGHT_COLOR     = 0x009,
    CSS_PROP_BORDER_BOTTOM_COLOR        = 0x00a,
    CSS_PROP_BORDER_LEFT_COLOR      = 0x00b,
    CSS_PROP_BORDER_TOP_STYLE       = 0x00c,
    CSS_PROP_BORDER_RIGHT_STYLE     = 0x00d,
    CSS_PROP_BORDER_BOTTOM_STYLE        = 0x00e,
    CSS_PROP_BORDER_LEFT_STYLE      = 0x00f,
    CSS_PROP_BORDER_TOP_WIDTH       = 0x010,
    CSS_PROP_BORDER_RIGHT_WIDTH     = 0x011,
    CSS_PROP_BORDER_BOTTOM_WIDTH        = 0x012,
    CSS_PROP_BORDER_LEFT_WIDTH      = 0x013,
    CSS_PROP_BOTTOM             = 0x014,
    CSS_PROP_CAPTION_SIDE           = 0x015,
    CSS_PROP_CLEAR              = 0x016,
    CSS_PROP_CLIP               = 0x017,
    CSS_PROP_COLOR              = 0x018,
    CSS_PROP_CONTENT            = 0x019,
    CSS_PROP_COUNTER_INCREMENT      = 0x01a,
    CSS_PROP_COUNTER_RESET          = 0x01b,
    CSS_PROP_CUE_AFTER          = 0x01c,
    CSS_PROP_CUE_BEFORE         = 0x01d,
    CSS_PROP_CURSOR             = 0x01e,
    CSS_PROP_DIRECTION          = 0x01f,
    CSS_PROP_DISPLAY            = 0x020,
    CSS_PROP_ELEVATION          = 0x021,
    CSS_PROP_EMPTY_CELLS            = 0x022,
    CSS_PROP_FLOAT              = 0x023,
    CSS_PROP_FONT_FAMILY            = 0x024,
    CSS_PROP_FONT_SIZE          = 0x025,
    CSS_PROP_FONT_STYLE         = 0x026,
    CSS_PROP_FONT_VARIANT           = 0x027,
    CSS_PROP_FONT_WEIGHT            = 0x028,
    CSS_PROP_HEIGHT             = 0x029,
    CSS_PROP_LEFT               = 0x02a,
    CSS_PROP_LETTER_SPACING         = 0x02b,
    CSS_PROP_LINE_HEIGHT            = 0x02c,
    CSS_PROP_LIST_STYLE_IMAGE       = 0x02d,
    CSS_PROP_LIST_STYLE_POSITION        = 0x02e,
    CSS_PROP_LIST_STYLE_TYPE        = 0x02f,
    CSS_PROP_MARGIN_TOP         = 0x030,
    CSS_PROP_MARGIN_RIGHT           = 0x031,
    CSS_PROP_MARGIN_BOTTOM          = 0x032,
    CSS_PROP_MARGIN_LEFT            = 0x033,
    CSS_PROP_MAX_HEIGHT         = 0x034,
    CSS_PROP_MAX_WIDTH          = 0x035,
    CSS_PROP_MIN_HEIGHT         = 0x036,
    CSS_PROP_MIN_WIDTH          = 0x037,
    CSS_PROP_ORPHANS            = 0x038,
    CSS_PROP_OUTLINE_COLOR          = 0x039,
    CSS_PROP_OUTLINE_STYLE          = 0x03a,
    CSS_PROP_OUTLINE_WIDTH          = 0x03b,
    CSS_PROP_OVERFLOW           = 0x03c,
    CSS_PROP_PADDING_TOP            = 0x03d,
    CSS_PROP_PADDING_RIGHT          = 0x03e,
    CSS_PROP_PADDING_BOTTOM         = 0x03f,
    CSS_PROP_PADDING_LEFT           = 0x040,
    CSS_PROP_PAGE_BREAK_AFTER       = 0x041,
    CSS_PROP_PAGE_BREAK_BEFORE      = 0x042,
    CSS_PROP_PAGE_BREAK_INSIDE      = 0x043,
    CSS_PROP_PAUSE_AFTER            = 0x044,
    CSS_PROP_PAUSE_BEFORE           = 0x045,
    CSS_PROP_PITCH_RANGE            = 0x046,
    CSS_PROP_PITCH              = 0x047,
    CSS_PROP_PLAY_DURING            = 0x048,
    CSS_PROP_POSITION           = 0x049,
    CSS_PROP_QUOTES             = 0x04a,
    CSS_PROP_RICHNESS           = 0x04b,
    CSS_PROP_RIGHT              = 0x04c,
    CSS_PROP_SPEAK_HEADER           = 0x04d,
    CSS_PROP_SPEAK_NUMERAL          = 0x04e,
    CSS_PROP_SPEAK_PUNCTUATION      = 0x04f,
    CSS_PROP_SPEAK              = 0x050,
    CSS_PROP_SPEECH_RATE            = 0x051,
    CSS_PROP_STRESS             = 0x052,
    CSS_PROP_TABLE_LAYOUT           = 0x053,
    CSS_PROP_TEXT_ALIGN         = 0x054,
    CSS_PROP_TEXT_DECORATION        = 0x055,
    CSS_PROP_TEXT_INDENT            = 0x056,
    CSS_PROP_TEXT_TRANSFORM         = 0x057,
    CSS_PROP_TOP                = 0x058,
    CSS_PROP_UNICODE_BIDI           = 0x059,
    CSS_PROP_VERTICAL_ALIGN         = 0x05a,
    CSS_PROP_VISIBILITY         = 0x05b,
    CSS_PROP_VOICE_FAMILY           = 0x05c,
    CSS_PROP_VOLUME             = 0x05d,
    CSS_PROP_WHITE_SPACE            = 0x05e,
    CSS_PROP_WIDOWS             = 0x05f,
    CSS_PROP_WIDTH              = 0x060,
    CSS_PROP_WORD_SPACING           = 0x061,
    CSS_PROP_Z_INDEX            = 0x062,
    CSS_PROP_OPACITY            = 0x063,
    CSS_PROP_BREAK_AFTER            = 0x064,
    CSS_PROP_BREAK_BEFORE           = 0x065,
    CSS_PROP_BREAK_INSIDE           = 0x066,
    CSS_PROP_COLUMN_COUNT           = 0x067,
    CSS_PROP_COLUMN_FILL            = 0x068,
    CSS_PROP_COLUMN_GAP         = 0x069,
    CSS_PROP_COLUMN_RULE_COLOR      = 0x06a,
    CSS_PROP_COLUMN_RULE_STYLE      = 0x06b,
    CSS_PROP_COLUMN_RULE_WIDTH      = 0x06c,
    CSS_PROP_COLUMN_SPAN            = 0x06d,
    CSS_PROP_COLUMN_WIDTH           = 0x06e,

    CSS_N_PROPERTIES = 0x06f
} 

pub enum css_background_attachment_e {
    CSS_BACKGROUND_ATTACHMENT_INHERIT   = 0x0,
    CSS_BACKGROUND_ATTACHMENT_FIXED     = 0x1,
    CSS_BACKGROUND_ATTACHMENT_SCROLL    = 0x2
}

pub enum css_background_color_e {
    CSS_BACKGROUND_COLOR_INHERIT        = 0x0,
    CSS_BACKGROUND_COLOR_COLOR      = 0x1,
    CSS_BACKGROUND_COLOR_CURRENT_COLOR  = 0x2
}

pub enum css_background_image_e {
    CSS_BACKGROUND_IMAGE_INHERIT        = 0x0,
    /* Consult pointer in struct to determine which */
    CSS_BACKGROUND_IMAGE_NONE       = 0x1,
}

pub static CSS_BACKGROUND_IMAGE_IMAGE : uint = 0x1;

pub enum css_background_position_e {
    CSS_BACKGROUND_POSITION_INHERIT     = 0x0,
    CSS_BACKGROUND_POSITION_SET     = 0x1
}

pub enum css_background_repeat_e {
    CSS_BACKGROUND_REPEAT_INHERIT       = 0x0,
    CSS_BACKGROUND_REPEAT_REPEAT_X      = 0x1,
    CSS_BACKGROUND_REPEAT_REPEAT_Y      = 0x2,
    CSS_BACKGROUND_REPEAT_REPEAT        = 0x3,
    CSS_BACKGROUND_REPEAT_NO_REPEAT     = 0x4
}

pub enum css_border_collapse_e {
    CSS_BORDER_COLLAPSE_INHERIT     = 0x0,
    CSS_BORDER_COLLAPSE_SEPARATE        = 0x1,
    CSS_BORDER_COLLAPSE_COLLAPSE        = 0x2
}

pub enum border_side_e {
    BORDER_SIDE_TOP = 0,
    BORDER_SIDE_RIGHT = 1, 
    BORDER_SIDE_BOTTOM = 2, 
    BORDER_SIDE_LEFT = 3 
}

pub enum css_border_spacing_e {
    CSS_BORDER_SPACING_INHERIT      = 0x0,
    CSS_BORDER_SPACING_SET          = 0x1
}

pub enum css_border_color_e {
    CSS_BORDER_COLOR_INHERIT        = 0x0,
    CSS_BORDER_COLOR_COLOR          = 0x1,
    CSS_BORDER_COLOR_CURRENT_COLOR      = 0x2
}

pub enum css_border_style_e {
    CSS_BORDER_STYLE_INHERIT        = 0x0,
    CSS_BORDER_STYLE_NONE           = 0x1,
    CSS_BORDER_STYLE_HIDDEN         = 0x2,
    CSS_BORDER_STYLE_DOTTED         = 0x3,
    CSS_BORDER_STYLE_DASHED         = 0x4,
    CSS_BORDER_STYLE_SOLID          = 0x5,
    CSS_BORDER_STYLE_DOUBLE         = 0x6,
    CSS_BORDER_STYLE_GROOVE         = 0x7,
    CSS_BORDER_STYLE_RIDGE          = 0x8,
    CSS_BORDER_STYLE_INSET          = 0x9,
    CSS_BORDER_STYLE_OUTSET         = 0xa
}

pub enum css_border_width_e {
    CSS_BORDER_WIDTH_INHERIT        = 0x0,
    CSS_BORDER_WIDTH_THIN           = 0x1,
    CSS_BORDER_WIDTH_MEDIUM         = 0x2,
    CSS_BORDER_WIDTH_THICK          = 0x3,
    CSS_BORDER_WIDTH_WIDTH          = 0x4
}

pub enum css_bottom_e {
    CSS_BOTTOM_INHERIT          = 0x0,
    CSS_BOTTOM_SET              = 0x1,
    CSS_BOTTOM_AUTO             = 0x2
}

pub enum css_break_after_e {
    CSS_BREAK_AFTER_INHERIT         = 0x0,
    CSS_BREAK_AFTER_AUTO            = 0x1,
    CSS_BREAK_AFTER_AVOID           = 0x2,
    CSS_BREAK_AFTER_ALWAYS          = 0x3,
    CSS_BREAK_AFTER_LEFT            = 0x4,
    CSS_BREAK_AFTER_RIGHT           = 0x5,
    CSS_BREAK_AFTER_PAGE            = 0x6,
    CSS_BREAK_AFTER_COLUMN          = 0x7,
    CSS_BREAK_AFTER_AVOID_PAGE      = 0x8,
    CSS_BREAK_AFTER_AVOID_COLUMN        = 0x9
}   

pub enum css_break_before_e {
    CSS_BREAK_BEFORE_INHERIT        = 0x0,
    CSS_BREAK_BEFORE_AUTO           = 0x1,
    CSS_BREAK_BEFORE_AVOID          = 0x2,
    CSS_BREAK_BEFORE_ALWAYS         = 0x3,
    CSS_BREAK_BEFORE_LEFT           = 0x4,
    CSS_BREAK_BEFORE_RIGHT          = 0x5,
    CSS_BREAK_BEFORE_PAGE           = 0x6,
    CSS_BREAK_BEFORE_COLUMN         = 0x7,
    CSS_BREAK_BEFORE_AVOID_PAGE     = 0x8,
    CSS_BREAK_BEFORE_AVOID_COLUMN       = 0x9
}

pub enum css_break_inside_e {
    CSS_BREAK_INSIDE_INHERIT        = 0x0,
    CSS_BREAK_INSIDE_AUTO           = 0x1,
    CSS_BREAK_INSIDE_AVOID          = 0x2,
    CSS_BREAK_INSIDE_AVOID_PAGE     = 0x8,
    CSS_BREAK_INSIDE_AVOID_COLUMN       = 0x9
}

pub enum css_caption_side_e {
    CSS_CAPTION_SIDE_INHERIT        = 0x0,
    CSS_CAPTION_SIDE_TOP            = 0x1,
    CSS_CAPTION_SIDE_BOTTOM         = 0x2
}

pub enum css_clear_e {
    CSS_CLEAR_INHERIT           = 0x0,
    CSS_CLEAR_NONE              = 0x1,
    CSS_CLEAR_LEFT              = 0x2,
    CSS_CLEAR_RIGHT             = 0x3,
    CSS_CLEAR_BOTH              = 0x4
}

pub enum css_clip_e {
    CSS_CLIP_INHERIT            = 0x0,
    CSS_CLIP_AUTO               = 0x1,
    CSS_CLIP_RECT               = 0x2
}

pub enum css_color_e {
    CSS_COLOR_INHERIT           = 0x0,
    CSS_COLOR_COLOR             = 0x1
}

pub enum css_column_count_e {
    CSS_COLUMN_COUNT_INHERIT        = 0x0,
    CSS_COLUMN_COUNT_AUTO           = 0x1,
    CSS_COLUMN_COUNT_SET            = 0x2
}

pub enum css_column_fill_e {
    CSS_COLUMN_FILL_INHERIT         = 0x0,
    CSS_COLUMN_FILL_BALANCE         = 0x1,
    CSS_COLUMN_FILL_AUTO            = 0x2
}

pub enum css_column_gap_e {
    CSS_COLUMN_GAP_INHERIT          = 0x0,
    CSS_COLUMN_GAP_NORMAL           = 0x1,
    CSS_COLUMN_GAP_SET          = 0x2
}

pub enum css_column_rule_color_e {
    CSS_COLUMN_RULE_COLOR_INHERIT       = 0x0,
    CSS_COLUMN_RULE_COLOR_COLOR     = 0x1,
    CSS_COLUMN_RULE_COLOR_CURRENT_COLOR = 0x2
}

pub enum css_column_rule_style_e {
    CSS_COLUMN_RULE_STYLE_INHERIT       = 0x0,
    CSS_COLUMN_RULE_STYLE_NONE      = 0x1,
    CSS_COLUMN_RULE_STYLE_DOTTED        = 0x3,
    CSS_COLUMN_RULE_STYLE_DASHED        = 0x4,
    CSS_COLUMN_RULE_STYLE_SOLID     = 0x5,
    CSS_COLUMN_RULE_STYLE_DOUBLE        = 0x6,
    CSS_COLUMN_RULE_STYLE_GROOVE        = 0x7,
    CSS_COLUMN_RULE_STYLE_RIDGE     = 0x8,
    CSS_COLUMN_RULE_STYLE_INSET     = 0x9,
    CSS_COLUMN_RULE_STYLE_OUTSET        = 0xa
}

pub enum css_column_rule_width_e {
    CSS_COLUMN_RULE_WIDTH_INHERIT       = 0x0,
    CSS_COLUMN_RULE_WIDTH_THIN      = 0x1,
    CSS_COLUMN_RULE_WIDTH_MEDIUM        = 0x2,
    CSS_COLUMN_RULE_WIDTH_THICK     = 0x3,
    CSS_COLUMN_RULE_WIDTH_WIDTH     = 0x4
}

pub enum css_column_span_e {
    CSS_COLUMN_SPAN_INHERIT         = 0x0,
    CSS_COLUMN_SPAN_NONE            = 0x1,
    CSS_COLUMN_SPAN_ALL         = 0x2
}

pub enum css_column_width_e {
    CSS_COLUMN_WIDTH_INHERIT        = 0x0,
    CSS_COLUMN_WIDTH_AUTO           = 0x1,
    CSS_COLUMN_WIDTH_SET            = 0x2
}

pub enum css_content_e {
    CSS_CONTENT_INHERIT         = 0x0,
    CSS_CONTENT_NONE            = 0x1,
    CSS_CONTENT_NORMAL          = 0x2,
    CSS_CONTENT_SET             = 0x3
}

pub enum css_counter_increment_e {
    CSS_COUNTER_INCREMENT_INHERIT       = 0x0,
    /* Consult pointer in struct to determine which */
    CSS_COUNTER_INCREMENT_NAMED     = 0x1,
}

pub static CSS_COUNTER_INCREMENT_NONE : uint = 0x1 ;

pub enum css_counter_reset_e {
    CSS_COUNTER_RESET_INHERIT       = 0x0,
    /* Consult pointer in struct to determine which */
    CSS_COUNTER_RESET_NAMED         = 0x1,
}

pub static CSS_COUNTER_RESET_NONE : uint = 0x1 ;

pub enum css_cursor_e {
    CSS_CURSOR_INHERIT          = 0x00,
    /* URLs exist if pointer is non-NULL */
    CSS_CURSOR_AUTO             = 0x01,
    CSS_CURSOR_CROSSHAIR            = 0x02,
    CSS_CURSOR_DEFAULT          = 0x03,
    CSS_CURSOR_POINTER          = 0x04,
    CSS_CURSOR_MOVE             = 0x05,
    CSS_CURSOR_E_RESIZE         = 0x06,
    CSS_CURSOR_NE_RESIZE            = 0x07,
    CSS_CURSOR_NW_RESIZE            = 0x08,
    CSS_CURSOR_N_RESIZE         = 0x09,
    CSS_CURSOR_SE_RESIZE            = 0x0a,
    CSS_CURSOR_SW_RESIZE            = 0x0b,
    CSS_CURSOR_S_RESIZE         = 0x0c,
    CSS_CURSOR_W_RESIZE         = 0x0d,
    CSS_CURSOR_TEXT             = 0x0e,
    CSS_CURSOR_WAIT             = 0x0f,
    CSS_CURSOR_HELP             = 0x10,
    CSS_CURSOR_PROGRESS         = 0x11
}

pub enum css_direction_e {
    CSS_DIRECTION_INHERIT           = 0x0,
    CSS_DIRECTION_LTR           = 0x1,
    CSS_DIRECTION_RTL           = 0x2
}


pub enum css_display_e {
    CSS_DISPLAY_INHERIT         = 0x00,
    CSS_DISPLAY_INLINE          = 0x01,
    CSS_DISPLAY_BLOCK           = 0x02,
    CSS_DISPLAY_LIST_ITEM           = 0x03,
    CSS_DISPLAY_RUN_IN          = 0x04,
    CSS_DISPLAY_INLINE_BLOCK        = 0x05,
    CSS_DISPLAY_TABLE           = 0x06,
    CSS_DISPLAY_INLINE_TABLE        = 0x07,
    CSS_DISPLAY_TABLE_ROW_GROUP     = 0x08,
    CSS_DISPLAY_TABLE_HEADER_GROUP      = 0x09,
    CSS_DISPLAY_TABLE_FOOTER_GROUP      = 0x0a,
    CSS_DISPLAY_TABLE_ROW           = 0x0b,
    CSS_DISPLAY_TABLE_COLUMN_GROUP      = 0x0c,
    CSS_DISPLAY_TABLE_COLUMN        = 0x0d,
    CSS_DISPLAY_TABLE_CELL          = 0x0e,
    CSS_DISPLAY_TABLE_CAPTION       = 0x0f,
    CSS_DISPLAY_NONE            = 0x10
}

pub enum css_empty_cells_e {
    CSS_EMPTY_CELLS_INHERIT         = 0x0,
    CSS_EMPTY_CELLS_SHOW            = 0x1,
    CSS_EMPTY_CELLS_HIDE            = 0x2
}

pub enum css_float_e {
    CSS_FLOAT_INHERIT           = 0x0,
    CSS_FLOAT_LEFT              = 0x1,
    CSS_FLOAT_RIGHT             = 0x2,
    CSS_FLOAT_NONE              = 0x3
}

pub enum css_font_family_e {
    CSS_FONT_FAMILY_INHERIT         = 0x0,
    /* Named fonts exist if pointer is non-NULL */
    CSS_FONT_FAMILY_SERIF           = 0x1,
    CSS_FONT_FAMILY_SANS_SERIF      = 0x2,
    CSS_FONT_FAMILY_CURSIVE         = 0x3,
    CSS_FONT_FAMILY_FANTASY         = 0x4,
    CSS_FONT_FAMILY_MONOSPACE       = 0x5
}

pub enum css_font_size_e {
    CSS_FONT_SIZE_INHERIT           = 0x0,
    CSS_FONT_SIZE_XX_SMALL          = 0x1,
    CSS_FONT_SIZE_X_SMALL           = 0x2,
    CSS_FONT_SIZE_SMALL         = 0x3,
    CSS_FONT_SIZE_MEDIUM            = 0x4,
    CSS_FONT_SIZE_LARGE         = 0x5,
    CSS_FONT_SIZE_X_LARGE           = 0x6,
    CSS_FONT_SIZE_XX_LARGE          = 0x7,
    CSS_FONT_SIZE_LARGER            = 0x8,
    CSS_FONT_SIZE_SMALLER           = 0x9,
    CSS_FONT_SIZE_DIMENSION         = 0xa
}

pub enum css_font_style_e {
    CSS_FONT_STYLE_INHERIT          = 0x0,
    CSS_FONT_STYLE_NORMAL           = 0x1,
    CSS_FONT_STYLE_ITALIC           = 0x2,
    CSS_FONT_STYLE_OBLIQUE          = 0x3
}

pub enum css_font_variant_e {
    CSS_FONT_VARIANT_INHERIT        = 0x0,
    CSS_FONT_VARIANT_NORMAL         = 0x1,
    CSS_FONT_VARIANT_SMALL_CAPS     = 0x2
}

pub enum css_font_weight_e {
    CSS_FONT_WEIGHT_INHERIT         = 0x0,
    CSS_FONT_WEIGHT_NORMAL          = 0x1,
    CSS_FONT_WEIGHT_BOLD            = 0x2,
    CSS_FONT_WEIGHT_BOLDER          = 0x3,
    CSS_FONT_WEIGHT_LIGHTER         = 0x4,
    CSS_FONT_WEIGHT_100         = 0x5,
    CSS_FONT_WEIGHT_200         = 0x6,
    CSS_FONT_WEIGHT_300         = 0x7,
    CSS_FONT_WEIGHT_400         = 0x8,
    CSS_FONT_WEIGHT_500         = 0x9,
    CSS_FONT_WEIGHT_600         = 0xa,
    CSS_FONT_WEIGHT_700         = 0xb,
    CSS_FONT_WEIGHT_800         = 0xc,
    CSS_FONT_WEIGHT_900         = 0xd
}

pub enum css_height_e {
    CSS_HEIGHT_INHERIT          = 0x0,
    CSS_HEIGHT_SET              = 0x1,
    CSS_HEIGHT_AUTO             = 0x2
}

pub enum css_left_e {
    CSS_LEFT_INHERIT            = 0x0,
    CSS_LEFT_SET                = 0x1,
    CSS_LEFT_AUTO               = 0x2
}

pub enum css_letter_spacing_e {
    CSS_LETTER_SPACING_INHERIT      = 0x0,
    CSS_LETTER_SPACING_SET          = 0x1,
    CSS_LETTER_SPACING_NORMAL       = 0x2
}

pub enum css_line_height_e {
    CSS_LINE_HEIGHT_INHERIT         = 0x0,
    CSS_LINE_HEIGHT_NUMBER          = 0x1,
    CSS_LINE_HEIGHT_DIMENSION       = 0x2,
    CSS_LINE_HEIGHT_NORMAL          = 0x3
}

pub enum css_list_style_image_e {
    CSS_LIST_STYLE_IMAGE_INHERIT        = 0x0,
    /* Consult pointer in struct to determine which */
    CSS_LIST_STYLE_IMAGE_URI_OR_NONE       = 0x1,
}

pub enum css_list_style_position_e {
    CSS_LIST_STYLE_POSITION_INHERIT     = 0x0,
    CSS_LIST_STYLE_POSITION_INSIDE      = 0x1,
    CSS_LIST_STYLE_POSITION_OUTSIDE     = 0x2
}

pub enum css_list_style_type_e {
    CSS_LIST_STYLE_TYPE_INHERIT     = 0x0,
    CSS_LIST_STYLE_TYPE_DISC        = 0x1,
    CSS_LIST_STYLE_TYPE_CIRCLE      = 0x2,
    CSS_LIST_STYLE_TYPE_SQUARE      = 0x3,
    CSS_LIST_STYLE_TYPE_DECIMAL     = 0x4,
    CSS_LIST_STYLE_TYPE_DECIMAL_LEADING_ZERO= 0x5,
    CSS_LIST_STYLE_TYPE_LOWER_ROMAN     = 0x6,
    CSS_LIST_STYLE_TYPE_UPPER_ROMAN     = 0x7,
    CSS_LIST_STYLE_TYPE_LOWER_GREEK     = 0x8,
    CSS_LIST_STYLE_TYPE_LOWER_LATIN     = 0x9,
    CSS_LIST_STYLE_TYPE_UPPER_LATIN     = 0xa,
    CSS_LIST_STYLE_TYPE_ARMENIAN        = 0xb,
    CSS_LIST_STYLE_TYPE_GEORGIAN        = 0xc,
    CSS_LIST_STYLE_TYPE_LOWER_ALPHA     = 0xd,
    CSS_LIST_STYLE_TYPE_UPPER_ALPHA     = 0xe,
    CSS_LIST_STYLE_TYPE_NONE        = 0xf
}

pub enum css_margin_e {
    CSS_MARGIN_INHERIT          = 0x0,
    CSS_MARGIN_SET              = 0x1,
    CSS_MARGIN_AUTO             = 0x2
}

pub enum css_max_height_e {
    CSS_MAX_HEIGHT_INHERIT          = 0x0,
    CSS_MAX_HEIGHT_SET          = 0x1,
    CSS_MAX_HEIGHT_NONE         = 0x2
}

pub enum css_max_width_e {
    CSS_MAX_WIDTH_INHERIT           = 0x0,
    CSS_MAX_WIDTH_SET           = 0x1,
    CSS_MAX_WIDTH_NONE          = 0x2
}

pub enum css_min_height_e {
    CSS_MIN_HEIGHT_INHERIT          = 0x0,
    CSS_MIN_HEIGHT_SET          = 0x1
}

pub enum css_min_width_e {
    CSS_MIN_WIDTH_INHERIT           = 0x0,
    CSS_MIN_WIDTH_SET           = 0x1
}

pub enum css_opacity_e {
    CSS_OPACITY_INHERIT         = 0x0,
    CSS_OPACITY_SET             = 0x1
}

pub enum css_outline_color_e {
    CSS_OUTLINE_COLOR_INHERIT       = 0x0,
    CSS_OUTLINE_COLOR_COLOR         = 0x1,
    CSS_OUTLINE_COLOR_CURRENT_COLOR     = 0x2,
    CSS_OUTLINE_COLOR_INVERT        = 0x3
}

pub enum css_outline_style_e {
    CSS_OUTLINE_STYLE_INHERIT       = 0x0,
    CSS_OUTLINE_STYLE_NONE          = 0x1,
    CSS_OUTLINE_STYLE_DOTTED        = 0x3,
    CSS_OUTLINE_STYLE_DASHED        = 0x4,
    CSS_OUTLINE_STYLE_SOLID         = 0x5,
    CSS_OUTLINE_STYLE_DOUBLE        = 0x6,
    CSS_OUTLINE_STYLE_GROOVE        = 0x7,
    CSS_OUTLINE_STYLE_RIDGE         = 0x8,
    CSS_OUTLINE_STYLE_INSET         = 0x9,
    CSS_OUTLINE_STYLE_OUTSET        = 0xa
}

pub enum css_outline_width_e {
    CSS_OUTLINE_WIDTH_INHERIT       = 0x0,
    CSS_OUTLINE_WIDTH_THIN          = 0x1,
    CSS_OUTLINE_WIDTH_MEDIUM        = 0x2,
    CSS_OUTLINE_WIDTH_THICK         = 0x3,
    CSS_OUTLINE_WIDTH_WIDTH         = 0x4
}

pub enum css_overflow_e {
    CSS_OVERFLOW_INHERIT            = 0x0,
    CSS_OVERFLOW_VISIBLE            = 0x1,
    CSS_OVERFLOW_HIDDEN         = 0x2,
    CSS_OVERFLOW_SCROLL         = 0x3,
    CSS_OVERFLOW_AUTO           = 0x4
}

pub enum css_orphans_e {
    CSS_ORPHANS_INHERIT         = 0x0,
    CSS_ORPHANS_SET             = 0x1
}

pub enum css_padding_e {
    CSS_PADDING_INHERIT         = 0x0,
    CSS_PADDING_SET             = 0x1
}

pub enum css_page_break_after_e {
    CSS_PAGE_BREAK_AFTER_INHERIT        = 0x0,
    CSS_PAGE_BREAK_AFTER_AUTO       = 0x1,
    CSS_PAGE_BREAK_AFTER_AVOID      = 0x2,
    CSS_PAGE_BREAK_AFTER_ALWAYS     = 0x3,
    CSS_PAGE_BREAK_AFTER_LEFT       = 0x4,
    CSS_PAGE_BREAK_AFTER_RIGHT      = 0x5
}   

pub enum css_page_break_before_e {
    CSS_PAGE_BREAK_BEFORE_INHERIT       = 0x0,
    CSS_PAGE_BREAK_BEFORE_AUTO      = 0x1,
    CSS_PAGE_BREAK_BEFORE_AVOID     = 0x2,
    CSS_PAGE_BREAK_BEFORE_ALWAYS        = 0x3,
    CSS_PAGE_BREAK_BEFORE_LEFT      = 0x4,
    CSS_PAGE_BREAK_BEFORE_RIGHT     = 0x5
}

pub enum css_page_break_inside_e {
    CSS_PAGE_BREAK_INSIDE_INHERIT       = 0x0,
    CSS_PAGE_BREAK_INSIDE_AUTO      = 0x1,
    CSS_PAGE_BREAK_INSIDE_AVOID     = 0x2
}

pub enum css_position_e {
    CSS_POSITION_INHERIT            = 0x0,
    CSS_POSITION_STATIC         = 0x1,
    CSS_POSITION_RELATIVE           = 0x2,
    CSS_POSITION_ABSOLUTE           = 0x3,
    CSS_POSITION_FIXED          = 0x4
}

pub enum css_quotes_e {
    CSS_QUOTES_INHERIT          = 0x0,
/* Consult pointer in struct to determine which */
    CSS_QUOTES_STRING_OR_NONE           = 0x1,
}

pub enum css_right_e {
    CSS_RIGHT_INHERIT           = 0x0,
    CSS_RIGHT_SET               = 0x1,
    CSS_RIGHT_AUTO              = 0x2
}

pub enum css_table_layout_e {
    CSS_TABLE_LAYOUT_INHERIT        = 0x0,
    CSS_TABLE_LAYOUT_AUTO           = 0x1,
    CSS_TABLE_LAYOUT_FIXED          = 0x2
}

pub enum css_text_align_e {
    CSS_TEXT_ALIGN_INHERIT          = 0x0,
    CSS_TEXT_ALIGN_INHERIT_IF_NON_MAGIC = 0x1,
    CSS_TEXT_ALIGN_LEFT         = 0x2,
    CSS_TEXT_ALIGN_RIGHT            = 0x3,
    CSS_TEXT_ALIGN_CENTER           = 0x4,
    CSS_TEXT_ALIGN_JUSTIFY          = 0x5,
    CSS_TEXT_ALIGN_DEFAULT          = 0x6,
    CSS_TEXT_ALIGN_LIBCSS_LEFT      = 0x7,
    CSS_TEXT_ALIGN_LIBCSS_CENTER        = 0x8,
    CSS_TEXT_ALIGN_LIBCSS_RIGHT     = 0x9
}

pub enum css_text_decoration_e {
    CSS_TEXT_DECORATION_INHERIT     = 0x00,
    CSS_TEXT_DECORATION_NONE        = 0x10,
    CSS_TEXT_DECORATION_BLINK       = (1<<3),
    CSS_TEXT_DECORATION_LINE_THROUGH    = (1<<2),
    CSS_TEXT_DECORATION_OVERLINE        = (1<<1),
    CSS_TEXT_DECORATION_UNDERLINE       = (1<<0)
}

pub enum css_text_indent_e {
    CSS_TEXT_INDENT_INHERIT         = 0x0,
    CSS_TEXT_INDENT_SET         = 0x1
}

pub enum css_text_transform_e {
    CSS_TEXT_TRANSFORM_INHERIT      = 0x0,
    CSS_TEXT_TRANSFORM_CAPITALIZE       = 0x1,
    CSS_TEXT_TRANSFORM_UPPERCASE        = 0x2,
    CSS_TEXT_TRANSFORM_LOWERCASE        = 0x3,
    CSS_TEXT_TRANSFORM_NONE         = 0x4
}

pub enum css_top_e {
    CSS_TOP_INHERIT             = 0x0,
    CSS_TOP_SET             = 0x1,
    CSS_TOP_AUTO                = 0x2
}

pub enum css_unicode_bidi_e {
    CSS_UNICODE_BIDI_INHERIT        = 0x0,
    CSS_UNICODE_BIDI_NORMAL         = 0x1,
    CSS_UNICODE_BIDI_EMBED          = 0x2,
    CSS_UNICODE_BIDI_BIDI_OVERRIDE      = 0x3
}

pub enum css_vertical_align_e {
    CSS_VERTICAL_ALIGN_INHERIT      = 0x0,
    CSS_VERTICAL_ALIGN_BASELINE     = 0x1,
    CSS_VERTICAL_ALIGN_SUB          = 0x2,
    CSS_VERTICAL_ALIGN_SUPER        = 0x3,
    CSS_VERTICAL_ALIGN_TOP          = 0x4,
    CSS_VERTICAL_ALIGN_TEXT_TOP     = 0x5,
    CSS_VERTICAL_ALIGN_MIDDLE       = 0x6,
    CSS_VERTICAL_ALIGN_BOTTOM       = 0x7,
    CSS_VERTICAL_ALIGN_TEXT_BOTTOM      = 0x8,
    CSS_VERTICAL_ALIGN_SET          = 0x9
}

pub enum css_visibility_e {
    CSS_VISIBILITY_INHERIT          = 0x0,
    CSS_VISIBILITY_VISIBLE          = 0x1,
    CSS_VISIBILITY_HIDDEN           = 0x2,
    CSS_VISIBILITY_COLLAPSE         = 0x3
}

pub enum css_white_space_e {
    CSS_WHITE_SPACE_INHERIT         = 0x0,
    CSS_WHITE_SPACE_NORMAL          = 0x1,
    CSS_WHITE_SPACE_PRE         = 0x2,
    CSS_WHITE_SPACE_NOWRAP          = 0x3,
    CSS_WHITE_SPACE_PRE_WRAP        = 0x4,
    CSS_WHITE_SPACE_PRE_LINE        = 0x5
}

pub enum css_widows_e {
    CSS_WIDOWS_INHERIT          = 0x0,
    CSS_WIDOWS_SET              = 0x1
}   

pub enum css_width_e {
    CSS_WIDTH_INHERIT           = 0x0,
    CSS_WIDTH_SET               = 0x1,
    CSS_WIDTH_AUTO              = 0x2
}

pub enum css_word_spacing_e {
    CSS_WORD_SPACING_INHERIT        = 0x0,
    CSS_WORD_SPACING_SET            = 0x1,
    CSS_WORD_SPACING_NORMAL         = 0x2
}

pub enum css_z_index_e {
    CSS_Z_INDEX_INHERIT         = 0x0,
    CSS_Z_INDEX_SET             = 0x1,
    CSS_Z_INDEX_AUTO            = 0x2
}

