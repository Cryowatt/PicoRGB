use std::usize;

// pub trait Renderer {

// }

// pub trait Channel {

// }

// pub trait R8G8B8 : Sized {}
#[derive(Clone, Copy, Default)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

// impl Display for Colour {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

// trait Colour : Sized {

// }

// impl Colour for R8G8B8 {}
// pub const BLACK: Colour::new();
// impl Colour {
//     pub const Black: Colour{r:0,g:0,b:0};
// }
// impl R8G8B8 for Colour {}

// pub trait Channel {
//     fn new<const LENGTH: usize>() -> Self {
//         ColourBuffer::new::<LENGTH>();
//     } // where Self: Sized;
// }

pub struct Channel {
    pub buffer: Box<[Colour]>,
}

impl Channel {
    fn new(length: usize) -> Self {
        Channel {
            buffer: vec![Colour::MAGENTA; length].into_boxed_slice(),
        }
    } // where Self: Sized;

    fn resize(&mut self, length: usize) {
        self.buffer = vec![Colour::RED; length].into_boxed_slice();
    }
}

// pub struct ColourBuffer<const LENGTH: usize> {
//     pub data: [Colour; LENGTH],
// }

// pub struct ColourBuffer {
//     pub data: Box<[Colour]>,
// }

// impl ColourBuffer {
//     fn new(length: usize) -> Self {
//         ColourBuffer {
//             data: Vec::<Colour>::with_capacity(length).as_slice(),
//         }
//     } // where Self: Sized;
// }

// impl Channel for ColourBuffer {
//     fn new<const LENGTH: usize>() -> Self {
//         ColourBuffer { data: [Colour{r:0,g:0,b:0}; LENGTH]}
//     }
// }

// impl<const LENGTH: usize> ColourBuffer<LENGTH> {
//     pub fn new() -> Self {
//         ColourBuffer {
//             data: [Colour { r: 0, g: 0, b: 0 }; LENGTH],
//         }
//     }
// }

pub trait Renderable {
    fn render(&self);
}

const CHANNELS: usize = 8;

pub struct Engine {
    renderer: fn(&Channel),
    channels: [Channel; 8],
    render_fps: u8,
}

impl Engine {
    pub fn resize_channel(&mut self, channel_id: usize, length: usize) {
        self.channels[channel_id].resize(length);
    }

    pub fn new(channel_lengths: [usize; CHANNELS], renderer: fn(&Channel)) -> Self {
        Engine {
            channels: channel_lengths.map(Channel::new),
            render_fps: 10,
            renderer,
        }
    }
}

impl Renderable for Engine {
    fn render(&self) {
        for i in 0..self.channels.len() {
            (self.renderer)(&self.channels[i]);
        }
    }
}
