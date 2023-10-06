use console_engine::Color;

pub const WIDTH: u32 = 100;
pub const HEIGHT: u32 = 25;
pub const TARGET_FPS: u32 = 20;

pub const DARK: Color = Color::Rgb {
    r: 148,
    g: 93,
    b: 6,
};
pub const NORMAL: Color = Color::Rgb {
    r: 247,
    g: 165,
    b: 0,
};
pub const LIGHT: Color = Color::Rgb {
    r: 255,
    g: 211,
    b: 122,
};
pub const DULL: Color = Color::Rgb {
    r: 84,
    g: 66,
    b: 29,
};
pub const DISABLED: Color = Color::Rgb {
    r: 87,
    g: 87,
    b: 87,
};
pub const DARKENAB: Color = Color::Rgb {
    r: 117,
    g: 117,
    b: 117,
};
pub const ENABLED: Color = Color::Rgb {
    r: 187,
    g: 187,
    b: 187,
};
pub const HIGHLIGHT: Color = Color::Rgb {
    r: 230,
    g: 80,
    b: 0,
};
