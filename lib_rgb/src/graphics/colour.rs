#[derive(Clone, Copy, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Colour {
    // Ordered for WS2812B GRB format
    pub b: u8,
    pub r: u8,
    pub g: u8,
}

impl Colour {
    pub const BLACK: Colour = Colour { r: 0, g: 0, b: 0 };
    pub const WHITE: Colour = Colour {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Colour = Colour { r: 255, g: 0, b: 0 };
    pub const YELLOW: Colour = Colour {
        r: 255,
        g: 255,
        b: 0,
    };
    pub const GREEN: Colour = Colour { r: 0, g: 255, b: 0 };
    pub const CYAN: Colour = Colour {
        r: 0,
        g: 255,
        b: 255,
    };
    pub const BLUE: Colour = Colour { r: 0, g: 0, b: 255 };
    pub const MAGENTA: Colour = Colour {
        r: 255,
        g: 0,
        b: 255,
    };
}
