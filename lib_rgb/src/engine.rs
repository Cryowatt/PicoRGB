extern crate alloc;
//type float = f32;

use alloc::{rc::Rc, boxed::Box, vec};
use fixed::types::I16F16;
// use fixed::traits::Fixed;
// use std::{rc::Rc, sync::Arc, usize};

pub trait Shader {
    fn apply(&mut self, channel: &mut [Colour], delta: I16F16);
}

#[derive(Clone, Copy, Default)]
pub struct NoOpShader();

impl Shader for NoOpShader {
    fn apply(&mut self, _channel: &mut [Colour], _delta: I16F16) { }
}

pub trait Gradient {    
    fn get(&self, position: I16F16 ) -> Colour;
}

pub struct UnicornVomit {}

// impl Gradient for UnicornVomit2 {
//     fn get(&self, position: i32) -> Colour {
//         // const AMPLITUDE: float =  float::from_num(765);
//         let max_byte: I16F16 =  I16F16::from_num(255);
//         // const DOUBLE: float = float::from_num(2);
//         let half: I16F16 = I16F16::from_num(0.5);
        
//         // let r = (765 * (2 * (position - HALF - position.floor())).abs() - MAX_BYTE).clamp(float::ZERO, MAX_BYTE);
//         let r = ((position - half - position.floor()).saturating_mul_int(2).abs().saturating_mul_int(765) - max_byte).clamp(I16F16::ZERO, max_byte);
//         let position = position + I16F16::from_num (2.0 / 3.0);
//         let g = ((position - half - position.floor()).saturating_mul_int(2).abs().saturating_mul_int(765) - max_byte).clamp(I16F16::ZERO, max_byte);
//         let position = position + I16F16::from_num (2.0 / 3.0);
//         let b = ((position - half - position.floor()).saturating_mul_int(2).abs().saturating_mul_int(765) - max_byte).clamp(I16F16::ZERO, max_byte);
        
//         // let position = position + (2.0 / 3.0);
//         // let g = (AMPLITUDE * UnicornVomit::abs(2.0f32 * (position - 0.5f32 - UnicornVomit::floor(position))) - MAX_BYTE).clamp(0.0f32, MAX_BYTE) as u8;
//         // let position = position + (2.0 / 3.0);
//         // let b = (AMPLITUDE * UnicornVomit::abs(2.0f32 * (position - 0.5f32 - UnicornVomit::floor(position))) - MAX_BYTE).clamp(0.0f32, MAX_BYTE) as u8;
//         //Colour { r, g, b }
//         Colour { r:r.to_num::<u8>(), g: g.to_num::<u8>(), b: b.to_num::<u8>() }
//     }
// }

impl Gradient for UnicornVomit {
    fn get(&self, position: I16F16) -> Colour {
        let half: I16F16 = I16F16::from_num(0.5);
        let x = 2*255*(3*position-(3*position+half).floor()).abs();
        let c = 255;
        // let x = ((position % 511)-255).abs();
        match ((position * 6) % 6).int().to_num() {
            0 => Colour {r:c, g:x.to_num(), b:0}, // R -> Y
            1 => Colour {r:x.to_num(), g:c, b:0}, // Y -> G
            2 => Colour {r:0, g:c, b:x.to_num()}, // G -> C
            3 => Colour {r:0, g:x.to_num(), b:c}, // C -> B
            4 => Colour {r:x.to_num(), g:0, b:c}, // B -> M
            _ => Colour {r:c, g:0, b:x.to_num()}, // M -> R
        }
        // // let c = 1.0;
        // // let x = ;
        // // const AMPLITUDE: float =  float::from_num(765);
        // let max_byte: I16F16 =  I16F16::from_num(255);
        // // const DOUBLE: float = float::from_num(2);
        // let half: I16F16 = I16F16::from_num(0.5);
        
        // // let r = (765 * (2 * (position - HALF - position.floor())).abs() - MAX_BYTE).clamp(float::ZERO, MAX_BYTE);
        // let r = ((position - half - position.floor()).saturating_mul_int(2).abs().saturating_mul_int(765) - max_byte).clamp(I16F16::ZERO, max_byte);
        // let position = position + I16F16::from_num (2.0 / 3.0);
        // let g = ((position - half - position.floor()).saturating_mul_int(2).abs().saturating_mul_int(765) - max_byte).clamp(I16F16::ZERO, max_byte);
        // let position = position + I16F16::from_num (2.0 / 3.0);
        // let b = ((position - half - position.floor()).saturating_mul_int(2).abs().saturating_mul_int(765) - max_byte).clamp(I16F16::ZERO, max_byte);
        
        // // let position = position + (2.0 / 3.0);
        // // let g = (AMPLITUDE * UnicornVomit::abs(2.0f32 * (position - 0.5f32 - UnicornVomit::floor(position))) - MAX_BYTE).clamp(0.0f32, MAX_BYTE) as u8;
        // // let position = position + (2.0 / 3.0);
        // // let b = (AMPLITUDE * UnicornVomit::abs(2.0f32 * (position - 0.5f32 - UnicornVomit::floor(position))) - MAX_BYTE).clamp(0.0f32, MAX_BYTE) as u8;
        // //Colour { r, g, b }
        // Colour { r:r.to_num::<u8>(), g: g.to_num::<u8>(), b: b.to_num::<u8>() }
        // Colour::RED
    }
}

pub struct ChaseShader {
    chase: Rc<dyn Gradient>,
    position: I16F16,
}

impl ChaseShader {
    pub fn new(gradient: Rc<dyn Gradient>) -> Self {
        ChaseShader {
            chase: gradient,
            position: I16F16::ZERO,
        }
    }
}

impl Shader for ChaseShader {
    fn apply(&mut self, channel: &mut [Colour], delta: I16F16) {
        self.position += delta;        
        let channel_length = I16F16::from_num(channel.len());
 
        for i in 0..channel.len() {
            let degrees = I16F16::from_num(i) / channel_length;
            channel[i as usize] = self.chase.get(self.position + degrees);
        }
    }
}

// class ChaseShader(Shader):
//     def __init__(self, source: ColorBuffer, cycles_per_second: float):
//         self.chase = source
//         # self.chase = rgb.colorbuffer.ColorBuffer(256)
//         # i = 0
//         # for pixel in [colorsys.hls_to_rgb888(x / 256.0, 0.5, 1.0) for x in range(256)]:
//         #     bpixel = self.chase.buffer[i]
//         #     bpixel.r, bpixel.g, bpixel.b = pixel[0], pixel[1], pixel[2]
//         #     i += 1
//         self.position = 0.0
//         self.cycles_per_second = cycles_per_second

//     def reset(self, channel):
//         self.position = 0.0

//     def render(self, channel, delta: float):
//         self.position += (len(self.chase) * self.cycles_per_second) * delta
//         period = len(self.chase) / channel.length
//         p = self.position
//         for i in range(0, channel.length):
//             if abs(p) >= self.chase.length:
//                 p %= self.chase.length
//             # p = int((self.position + (i * (len(self.chase) / channel.length))) % self.chase.length))
//             # print(p)
//             # cpixel = self.chase.buffer[int(p)]
//             channel.buffer[i].data = self.chase.buffer[int(p)].data
//             # bpixel = channel.buffer[i]
//             # bpixel.data = cpixel.data
//             p += period

pub struct Channel {
    pub buffer: Box<[Colour]>,
    pub shader: Box<dyn Shader>,
    pub renderer: Box<dyn Renderer>,
    position: I16F16,
}

static DEFAULT_SHADER: NoOpShader = NoOpShader();

impl Channel {
    fn new(length: usize) -> Self {
        Channel {
            buffer: vec![Colour::MAGENTA; length].into_boxed_slice(),
            shader: Box::new(DEFAULT_SHADER),
            position: I16F16::ZERO,
            renderer: Box::new(NullRenderer()),
        }
    }

    fn resize(&mut self, length: usize) {
        self.buffer = vec![Colour::RED; length].into_boxed_slice();
    }

    fn update(&mut self, delta: I16F16) {
        self.position += delta;
        self.shader.apply(self.buffer.as_mut(), delta);
    }

    fn render(&mut self) {
        let renderer = self.renderer.as_mut();
        renderer.render(&self.buffer);
    }
}

pub struct Engine<const CHANNEL_COUNT: usize> {
    // renderfn: fn(&Channel),
    channels: [Channel; CHANNEL_COUNT],
    // renderer: Box<dyn Renderer>,
    // render_fps: u8,
}

pub trait Renderer {
    fn render(&mut self, channel: &[Colour]);
}

pub struct NullRenderer();

impl Renderer for NullRenderer {
    fn render(&mut self, _channel: &[Colour]) {}
}

impl<const CHANNEL_COUNT: usize> Engine<CHANNEL_COUNT> {
    pub fn resize_channel(&mut self, channel_id: usize, length: usize) {
        self.channels[channel_id].resize(length);
    }

    pub fn set_shader(&mut self, channel_id: usize, shader: Box<dyn Shader>) {
        self.channels[channel_id].shader = shader;
    }

    pub fn set_renderer(&mut self, channel_id: usize, renderer: Box<dyn Renderer>) {
        self.channels[channel_id].renderer = renderer;
    }

//    pub fn new(channel_lengths: [usize; CHANNEL_COUNT], renderer: fn(&Channel)) -> Self {
    pub fn new(channel_lengths: [usize; CHANNEL_COUNT]) -> Self {
        Engine {
            channels: channel_lengths.map(Channel::new),
            // render_fps: 10,
            // renderer,
        }
    }

    pub fn update(&mut self, delta: I16F16) {
        for channel in self.channels.iter_mut() {
            channel.update(delta);
        }
    }

    pub fn render(&mut self) {
        for channel in self.channels.iter_mut() {
            channel.render();
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
#[repr(C)]
pub struct Colour {
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
