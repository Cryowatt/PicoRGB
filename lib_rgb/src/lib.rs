#![no_std]
type FixedTime = fixed::types::I16F16;

mod engine;
pub use engine::Engine;
pub use engine::Channel;
pub use engine::Renderer;

pub mod graphics;