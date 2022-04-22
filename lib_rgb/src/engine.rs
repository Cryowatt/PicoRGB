use std::{rc::Rc, sync::Arc, usize};

pub trait Shader {
    fn apply(&mut self, channel: &mut [Colour], delta: f32);
}

#[derive(Clone, Copy, Default)]
pub struct NoOpShader();

impl Shader for NoOpShader {
    fn apply(&mut self, channel: &mut [Colour], _delta: f32) {
        let mut i = 0;
        // TODO: Not quite a no-op now is it?
        for colour in channel.iter_mut() {
            colour.r = colour.r.wrapping_add(i);
            i = i.wrapping_add(1);
            colour.g = colour.g.wrapping_add(i);
            i = i.wrapping_add(1);
            colour.b = colour.b.wrapping_add(i);
            i = i.wrapping_add(1);
        }
    }
}

pub trait Gradient {
    fn get(&self, position: f32) -> Colour;
}

pub struct UnicornVomit {}

impl Gradient for UnicornVomit {
    fn get(&self, position: f32) -> Colour {
        // ⁅256(2|2(𝑥+1/2−floor((𝑥+1/2)+1/2))|−2/3)⁆
        // let r = 256 * (2*(2*(position+0.5f)-(position+0.5f))+0.5f)).floor())).abs()-(2/3));
        
        let r = (765.0f32*(2.0f32*(position - 0.5f32 - position.floor())).abs()-255.0f32).clamp(0.0f32, 255.0f32) as u8;
        let position = position + (2.0/3.0);
        let g = (765.0f32*(2.0f32*(position - 0.5f32 - position.floor())).abs()-255.0f32).clamp(0.0f32, 255.0f32) as u8;
        let position = position + (2.0/3.0);
        let b = (765.0f32*(2.0f32*(position - 0.5f32 - position.floor())).abs()-255.0f32).clamp(0.0f32, 255.0f32) as u8;
        Colour { r, g, b }
        // let g = 2 * (2*(position-0.5-(position).floor())).abs() - (2/3);
        // let b = 2 * (2*(position-0.5-(position).floor())).abs() - (2/3);
    }
}

pub struct ChaseShader {
    chase: Arc<dyn Gradient>,
    position: f32,
}

impl ChaseShader {
    pub fn new(gradient: Arc<dyn Gradient>) -> Self {
        ChaseShader { chase: gradient, position: 0f32 }
    }
}

impl Shader for ChaseShader {
    fn apply(&mut self, channel: &mut [Colour], delta: f32) {
        self.position = self.position + delta;

        for i in 0..channel.len() {
            let o = i as f32 / channel.len() as f32;
            channel[i] = self.chase.get(self.position + o);
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
    position: f32,
}

static DEFAULT_SHADER: NoOpShader = NoOpShader();

impl Channel {
    fn new(length: usize) -> Self {
        Channel {
            buffer: vec![Colour::MAGENTA; length].into_boxed_slice(),
            shader: Box::new(DEFAULT_SHADER),
            position: 0f32,
        }
    }

    fn resize(&mut self, length: usize) {
        self.buffer = vec![Colour::RED; length].into_boxed_slice();
    }

    fn update(&mut self, delta: f32) {
        self.shader.apply(self.buffer.as_mut(), delta);
    }
}

pub struct Engine<const CHANNEL_COUNT: usize> {
    renderer: fn(&Channel),
    channels: [Channel; CHANNEL_COUNT],
    render_fps: u8,
}

impl<const CHANNEL_COUNT: usize> Engine<CHANNEL_COUNT> {
    pub fn resize_channel(&mut self, channel_id: usize, length: usize) {
        self.channels[channel_id].resize(length);
    }

    pub fn set_shader(&mut self, channel_id: usize, shader: Box<dyn Shader>) {
        self.channels[channel_id].shader = shader;
    }

    pub fn new(channel_lengths: [usize; CHANNEL_COUNT], renderer: fn(&Channel)) -> Self {
        Engine {
            channels: channel_lengths.map(Channel::new),
            render_fps: 10,
            renderer,
        }
    }

    pub fn update(&mut self, delta: f32) {
        for channel in self.channels.iter_mut() {
            channel.update(delta);
        }
    }

    pub fn render(&mut self) {
        for channel in self.channels.iter_mut() {
            (self.renderer)(channel);
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
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
