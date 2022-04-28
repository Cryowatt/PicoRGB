extern crate alloc;

use alloc::rc::Rc;

use crate::{Gradient, FixedTime};

use super::Colour;


pub trait Shader {
    fn apply(&mut self, channel: &mut [Colour], delta: FixedTime);
}

#[derive(Clone, Copy, Default)]
pub struct NoOpShader();

impl Shader for NoOpShader {
    fn apply(&mut self, _channel: &mut [Colour], _delta: FixedTime) { }
}

pub struct ChaseShader {
    chase: Rc<dyn Gradient>,
    position: FixedTime,
}

impl ChaseShader {
    pub fn new(gradient: Rc<dyn Gradient>) -> Self {
        ChaseShader {
            chase: gradient,
            position: FixedTime::ZERO,
        }
    }
}

impl Shader for ChaseShader {
    fn apply(&mut self, channel: &mut [Colour], delta: FixedTime) {
        self.position += delta;        
        let channel_length = FixedTime::from_num(channel.len());
 
        for i in 0..channel.len() {
            let degrees = FixedTime::from_num(i) / channel_length;
            channel[i as usize] = self.chase.get(self.position + degrees);
        }
    }
}