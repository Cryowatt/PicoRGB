#![no_std]
type FixedTime = fixed::types::I16F16;

mod engine;
pub use engine::Engine;
pub use engine::Channel;
pub use engine::Renderer;
pub use engine::Gradient;
pub use engine::UnicornVomit;

pub mod graphics;