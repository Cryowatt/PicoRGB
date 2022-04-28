extern crate alloc;
//type float = f32;

use alloc::{boxed::Box, vec};
use crate::{graphics::{colour::Colour, shader::Shader, NoOpShader}, FixedTime};
// use fixed::traits::Fixed;
// use std::{rc::Rc, sync::Arc, usize};


pub struct Channel {
    pub buffer: Box<[Colour]>,
    pub shader: Box<dyn Shader>,
    pub renderer: Box<dyn Renderer>,
    position: FixedTime,
}

static DEFAULT_SHADER: NoOpShader = NoOpShader();

impl Channel {
    fn new(length: usize) -> Self {
        Channel {
            buffer: vec![Colour::MAGENTA; length].into_boxed_slice(),
            shader: Box::new(DEFAULT_SHADER),
            position: FixedTime::ZERO,
            renderer: Box::new(NullRenderer()),
        }
    }

    fn resize(&mut self, length: usize) {
        self.buffer = vec![Colour::RED; length].into_boxed_slice();
    }

    fn update(&mut self, delta: FixedTime) {
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

    pub fn update(&mut self, delta: FixedTime) {
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