use iced::font::{Family, Stretch, Style, Weight};
use iced::{Font, Pixels};

// 字体：注意字符串不能改变
pub const BIGTITLE_FAMILY_NAME : &str = "方正小标宋_GBK"; //大标题：方正小标宋_GBK
pub const TITLE_FAMILY_NAME : &str = "楷体_GB2312"; //标题：楷体_GB2312
pub const TXT_FAMILY_NAME : &str = "仿宋_GB2312"; //正文：仿宋_GB2312
pub const ICON_FAMILY_NAME: &str = "Icons for Sniffnet";//图标

// 尺寸
pub const MAX_SIZE: Pixels = Pixels(20.0); //最大
pub const BIG_SIZE: Pixels = Pixels(18.0); //大
pub const NORMAL_SIZE: Pixels = Pixels(16.0); //正常
pub const SMALL_SIZE: Pixels = Pixels(14.0); //小

//大标题：方正小标宋_GBK
pub const BIGTITLE_BYTES: &[u8] =
    include_bytes!("../../resources/fonts/fzxb.ttf");

pub const BIGTITLE: Font = Font {
    family: Family::Name(BIGTITLE_FAMILY_NAME),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

//标题：楷体_GB2312
pub const TITLE_BYTES: &[u8] =
    include_bytes!("../../resources/fonts/kt.ttf");
pub const TITLE: Font = Font {
    family: Family::Name(TITLE_FAMILY_NAME),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

//正文：仿宋_GB2312
pub const TXT_BYTES: &[u8] =
    include_bytes!("../../resources/fonts/fs.ttf");
pub const TXT: Font = Font {
    family: Family::Name(TXT_FAMILY_NAME),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

//图标
pub const ICONS_BYTES: &[u8] = include_bytes!("../../resources/fonts/icons.ttf");
pub const ICONS: Font = Font::with_name(ICON_FAMILY_NAME);