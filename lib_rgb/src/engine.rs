use std::{usize};

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
    pub const BLACK:Colour = Colour{r:0,g:0, b:0};
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
    pub buffer: Box<[Colour]>
}

impl Channel {
        fn new(length: usize) -> Self {
            Channel { buffer: vec![Colour::BLACK; length].into_boxed_slice()}
        } // where Self: Sized;
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

pub struct Engine {
    // renderer: Box<dyn Renderer>,
    pub renderer: fn(&Box<Channel>),
    pub channels: [Box<Channel>; 8],
    pub render_fps: u8,
}

impl Engine {
    pub fn new(channel_lengths: [u8; 8], renderer: fn(&Box<Channel>)) -> Self {
        // let mut channels = [Channel; 8];

        // for i in 0..8 {
        //     channels[i] = Channel::new(channelLengths[i].into());
        // }
        // for length in channelLengths {
        //     channels[]
        // }
        // let c = Channel::new(8);
        let channels = [
            Box::new(Channel::new(channel_lengths[0].into())),
            Box::new(Channel::new(channel_lengths[1].into())),
            Box::new(Channel::new(channel_lengths[2].into())),
            Box::new(Channel::new(channel_lengths[3].into())),
            Box::new(Channel::new(channel_lengths[4].into())),
            Box::new(Channel::new(channel_lengths[5].into())),
            Box::new(Channel::new(channel_lengths[6].into())),
            Box::new(Channel::new(channel_lengths[7].into())),
        ];
        Engine {
            channels,
            render_fps: 10,
            renderer,
        }
    }
}

impl Renderable for Engine {
    fn render(&self) {
        for i  in 0..self.channels.len() {
            (self.renderer)(&self.channels[i]);
        }
    }
}
