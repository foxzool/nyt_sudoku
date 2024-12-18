use bevy::color::Color;
use bevy::prelude::Srgba;
use std::sync::LazyLock;

pub const WHITE_COLOR: Color = Color::WHITE;
// #fbd300
pub const GAME_YELLOW: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("fbd300").unwrap()));
// #121212
pub const DARK_BLACK: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("121212").unwrap()));
// #1b1b1b
pub const MED_BLACK: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("1b1b1b").unwrap()));
// #2a2a2a
pub const BLACK: LazyLock<Color> = LazyLock::new(|| Color::Srgba(Srgba::hex("2a2a2a").unwrap()));
// #363636
pub const DARKEST_GRAY: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("363636").unwrap()));
// #424242
pub const EXTRA_DARK_GRAY: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("424242").unwrap()));
// #5a5a5a
pub const DARKER_GRAY: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("5a5a5a").unwrap()));
// #727272
pub const DARK_GRAY: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("727272").unwrap()));
// #8b8b8b
pub const GRAY2: LazyLock<Color> = LazyLock::new(|| Color::Srgba(Srgba::hex("8b8b8b").unwrap()));
// #979797
pub const GRAY: LazyLock<Color> = LazyLock::new(|| Color::Srgba(Srgba::hex("979797").unwrap()));
// #a3a3a3
pub const LIGHT_GRAY: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("a3a3a3").unwrap()));
// #bbb
pub const LIGHTER_GRAY: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("bbb").unwrap()));
// #dfdfdf
pub const EXTRA_LIGHT_GRAY: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("dfdfdf").unwrap()));
// #ebebeb
pub const LIGHTEST_GRAY: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("ebebeb").unwrap()));
// #f8f8f8
pub const COOL_GRAY: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("f8f8f8").unwrap()));
// #12121260
pub const SCRIM: LazyLock<Color> = LazyLock::new(|| Color::Srgba(Srgba::hex("12121260").unwrap()));
// #ffffff60
pub const LIGHT_SCRIM: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("ffffff60").unwrap()));
// #4f85e5
pub const ACCENT_XD_BLUE: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("4f85e5").unwrap()));
// #346eb7
pub const ACCENT_BLUE: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("346eb7").unwrap()));
// #6ba1dd
pub const ACCENT_LIGHT_BLUE: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("6ba1dd").unwrap()));
// #fff0
pub const TRANSPARENT: Color = Color::linear_rgba(1.0, 1.0, 1.0, 0.0);

/// #f8cd05
pub const STRANDS_YELLOW: LazyLock<Color> =
    LazyLock::new(|| Color::Srgba(Srgba::hex("f8cd05").unwrap()));
