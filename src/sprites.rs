use embedded_graphics::{image::ImageRaw, pixelcolor::BinaryColor};

pub type ImgRawType = ImageRaw<'static, BinaryColor>;

// 'player-jet', WxH Pixel = 16 x 16 px
const SPRITE_PLAYER_JET: [u8; 32] = [
    0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x3F, 0xF0, 0x3C, 0x00, 0x3C, 0x00, 0xFF, 0x00, 0x7F, 0xFF,
    0x7F, 0xFF, 0xFF, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x1F, 0xF0, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00,
];
pub const RAW_PLAYER_JET: ImgRawType = ImageRaw::new(&SPRITE_PLAYER_JET, 16);

/// 'planet-killer', WxH Pixel = 24 x 24 px
const SPRITE_PLANET_KILLER: [u8; 72] = [
    0x00, 0x7e, 0x00, 0x01, 0xff, 0x80, 0x07, 0xff, 0xe0, 0x0f, 0xff, 0xf0, 0x1f, 0xff, 0xf8, 0x3f,
    0xff, 0xfc, 0x3f, 0xff, 0x3c, 0x7f, 0xfc, 0x0e, 0x7f, 0xfc, 0x0e, 0x7f, 0xf8, 0xce, 0xff, 0xfc,
    0x4e, 0xff, 0xfc, 0x0f, 0xff, 0xfe, 0x1f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0x7f, 0xff, 0xfe,
    0x7f, 0xff, 0xfe, 0x7f, 0xff, 0xfc, 0x3f, 0xff, 0xfc, 0x1f, 0xff, 0xf8, 0x0f, 0xff, 0xf0, 0x07,
    0xff, 0xe0, 0x01, 0xff, 0x80, 0x00, 0x3e, 0x00,
];

pub const RAW_PLANET_KILLER: ImgRawType = ImageRaw::new(&SPRITE_PLANET_KILLER, 24);

const HEART_SPRITE: [u8; 8] = [0x00, 0x6e, 0xff, 0xef, 0x7e, 0x3c, 0x18, 0x00];
pub const RAW_HEART: ImgRawType = ImageRaw::<BinaryColor>::new(&HEART_SPRITE, 8);

// 'game-over', WxH Pixel = 100 x 7 px
const SPRITE_GAME_OVER: [u8; 91] = [
    0x7c, 0x03, 0x80, 0x66, 0x07, 0xf0, 0x00, 0x7c, 0x06, 0x60, 0x7f, 0x03, 0xe0, 0x7c, 0x07, 0xc0,
    0x7e, 0x07, 0xf0, 0x00, 0xfe, 0x06, 0x60, 0x7f, 0x03, 0xf0, 0xc0, 0x0c, 0xe0, 0x7e, 0x07, 0x00,
    0x00, 0xce, 0x06, 0x60, 0x70, 0x03, 0x30, 0xdc, 0x0c, 0xe0, 0x7e, 0x07, 0xe0, 0x00, 0xce, 0x07,
    0xe0, 0x7e, 0x03, 0x70, 0xcc, 0x0f, 0xe0, 0x7e, 0x07, 0x00, 0x00, 0xce, 0x03, 0xc0, 0x70, 0x03,
    0xe0, 0x7c, 0x0c, 0xe0, 0x66, 0x07, 0xf0, 0x00, 0xfe, 0x03, 0xc0, 0x7f, 0x03, 0x70, 0x7c, 0x0c,
    0xe0, 0x66, 0x07, 0xf0, 0x00, 0x7c, 0x01, 0x80, 0x7f, 0x03, 0x70,
];
pub const RAW_GAME_OVER: ImgRawType = ImageRaw::new(&SPRITE_GAME_OVER, 100);

// 'bow-arrow', WxH Pixel = 35 x 64 px
const SPRITE_BOW_ARROW: [u8; 320] = [
    0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00,
    0x38, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x00, 0x00, 0x1e,
    0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x80, 0x00, 0x00, 0x00, 0x13, 0xc0,
    0x00, 0x00, 0x00, 0x11, 0xf0, 0x00, 0x00, 0x00, 0x30, 0xfc, 0x00, 0x00, 0x00, 0x20, 0x3f, 0x00,
    0x00, 0x00, 0x20, 0x1f, 0x80, 0x00, 0x00, 0x40, 0x0f, 0xc0, 0x00, 0x00, 0x40, 0x07, 0xc0, 0x00,
    0x00, 0x80, 0x03, 0xe0, 0x00, 0x00, 0x80, 0x01, 0xe0, 0x00, 0x01, 0x00, 0x01, 0xf0, 0x00, 0x01,
    0x00, 0x00, 0xf0, 0x00, 0x02, 0x00, 0x00, 0xf0, 0x00, 0x02, 0x00, 0x00, 0xf0, 0x00, 0x04, 0x00,
    0x00, 0xf0, 0x00, 0x04, 0x00, 0x01, 0xf0, 0x00, 0x0c, 0x00, 0x01, 0xf0, 0x00, 0x08, 0x00, 0x03,
    0xe0, 0x00, 0x18, 0x00, 0x03, 0xc0, 0x00, 0x10, 0x00, 0x03, 0x80, 0x00, 0x7e, 0x00, 0x03, 0x90,
    0x00, 0x7f, 0x00, 0x03, 0x9e, 0x00, 0x3f, 0x80, 0x03, 0x8f, 0x80, 0x7f, 0xff, 0xff, 0xff, 0xc0,
    0x7f, 0xff, 0xff, 0xff, 0xc0, 0x3f, 0x80, 0x03, 0x8f, 0x80, 0x7f, 0x00, 0x03, 0x9e, 0x00, 0x10,
    0x00, 0x03, 0x90, 0x00, 0x18, 0x00, 0x03, 0x80, 0x00, 0x08, 0x00, 0x03, 0xc0, 0x00, 0x08, 0x00,
    0x03, 0xe0, 0x00, 0x04, 0x00, 0x01, 0xf0, 0x00, 0x04, 0x00, 0x01, 0xf0, 0x00, 0x02, 0x00, 0x00,
    0xf0, 0x00, 0x02, 0x00, 0x00, 0xf0, 0x00, 0x03, 0x00, 0x00, 0xf0, 0x00, 0x01, 0x00, 0x00, 0xf0,
    0x00, 0x01, 0x80, 0x01, 0xf0, 0x00, 0x00, 0x80, 0x01, 0xe0, 0x00, 0x00, 0x80, 0x03, 0xe0, 0x00,
    0x00, 0x40, 0x07, 0xc0, 0x00, 0x00, 0x40, 0x0f, 0xc0, 0x00, 0x00, 0x20, 0x1f, 0x80, 0x00, 0x00,
    0x20, 0x3f, 0x00, 0x00, 0x00, 0x30, 0xfc, 0x00, 0x00, 0x00, 0x11, 0xf0, 0x00, 0x00, 0x00, 0x13,
    0xc0, 0x00, 0x00, 0x00, 0x0f, 0x80, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x00,
    0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00,
    0x00, 0x00, 0x78, 0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00,
];
pub const RAW_BOW_ARROW: ImgRawType = ImageRaw::<BinaryColor>::new(&SPRITE_BOW_ARROW, 35);
