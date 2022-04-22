// mod engine;

// pub struct Engine<const ChannelCount: usize> {
//     renderer: fn(&Channel),
//     channels: [Channel; ChannelCount],
//     render_fps: u8,
// }

// impl<const ChannelCount: usize> Engine<ChannelCount> {
//     pub fn resize_channel(&mut self, channel_id: usize, length: usize) {
//         self.channels[channel_id].resize(length);
//     }

//     pub fn new(channel_lengths: [usize; ChannelCount], renderer: fn(&Channel)) -> Self {
//         Engine {
//             channels: channel_lengths.map(Channel::new),
//             render_fps: 10,
//             renderer,
//         }
//     }

//     pub fn update(&mut self) {
//         let i = 1;
//         for colour in self.channels[0].buffer.iter_mut() {
//             colour.b = colour.b.wrapping_add(i);
//             let i = i + 1;
//         }
//         // let buffer = &self.channels[0].buffer(0);
//         // buffer. += 1;
//     }
// }

// pub trait Renderable {
//     fn render(&self);
// }

// impl<const ChannelCount: usize> Renderable for Engine<ChannelCount> {
//     fn render(&self) {
//         for i in 0..ChannelCount {
//             (self.renderer)(&self.channels[i]);
//         }
//     }
// }



// use std::{usize, rc::{self, Rc}};

// // pub trait Renderer {

// // }

// // pub trait Channel {

// // }

// // pub trait R8G8B8 : Sized {}
// #[derive(Clone, Copy, Default)]
// pub struct Colour {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
// }

// impl Colour {
//     pub const BLACK: Colour = Colour { r: 0, g: 0, b: 0 };
//     pub const WHITE: Colour = Colour {
//         r: 255,
//         g: 255,
//         b: 255,
//     };
//     pub const RED: Colour = Colour { r: 255, g: 0, b: 0 };
//     pub const YELLOW: Colour = Colour {
//         r: 255,
//         g: 255,
//         b: 0,
//     };
//     pub const GREEN: Colour = Colour { r: 0, g: 255, b: 0 };
//     pub const CYAN: Colour = Colour {
//         r: 0,
//         g: 255,
//         b: 255,
//     };
//     pub const BLUE: Colour = Colour { r: 0, g: 0, b: 255 };
//     pub const MAGENTA: Colour = Colour {
//         r: 255,
//         g: 0,
//         b: 255,
//     };
// }

// // impl Display for Colour {
// //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// //         todo!()
// //     }
// // }

// // trait Colour : Sized {

// // }

// // impl Colour for R8G8B8 {}
// // pub const BLACK: Colour::new();
// // impl Colour {
// //     pub const Black: Colour{r:0,g:0,b:0};
// // }
// // impl R8G8B8 for Colour {}

// // pub trait Channel {
// //     fn new<const LENGTH: usize>() -> Self {
// //         ColourBuffer::new::<LENGTH>();
// //     } // where Self: Sized;
// // }

// pub trait Shader {

// }

// pub struct NoOpShader{

// }

// impl Shader for NoOpShader {

// }

// // let DefaultShader = Rc::new(NoOpShader);



// // pub struct ColourBuffer<const LENGTH: usize> {
// //     pub data: [Colour; LENGTH],
// // }

// // pub struct ColourBuffer {
// //     pub data: Box<[Colour]>,
// // }

// // impl ColourBuffer {
// //     fn new(length: usize) -> Self {
// //         ColourBuffer {
// //             data: Vec::<Colour>::with_capacity(length).as_slice(),
// //         }
// //     } // where Self: Sized;
// // }

// // impl Channel for ColourBuffer {
// //     fn new<const LENGTH: usize>() -> Self {
// //         ColourBuffer { data: [Colour{r:0,g:0,b:0}; LENGTH]}
// //     }
// // }

// // impl<const LENGTH: usize> ColourBuffer<LENGTH> {
// //     pub fn new() -> Self {
// //         ColourBuffer {
// //             data: [Colour { r: 0, g: 0, b: 0 }; LENGTH],
// //         }
// //     }
// // }


// const CHANNELS: usize = 8;
