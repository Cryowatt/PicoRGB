use crate::FixedTime;

use super::Colour;

pub trait Gradient {
    fn get(&self, position: FixedTime) -> Colour;
}

pub struct UnicornVomit {}

impl Gradient for UnicornVomit {
    fn get(&self, position: FixedTime) -> Colour {
        let half: FixedTime = FixedTime::from_num(0.5);
        let x = 511 * (3 * position - (3 * position + half).floor()).abs();
        const C: u8 = 255;

        match ((position * 6) % 6).int().to_num() {
            0 => Colour {
                r: C,
                g: x.to_num(),
                b: 0,
            }, // R -> Y
            1 => Colour {
                r: x.to_num(),
                g: C,
                b: 0,
            }, // Y -> G
            2 => Colour {
                r: 0,
                g: C,
                b: x.to_num(),
            }, // G -> C
            3 => Colour {
                r: 0,
                g: x.to_num(),
                b: C,
            }, // C -> B
            4 => Colour {
                r: x.to_num(),
                g: 0,
                b: C,
            }, // B -> M
            _ => Colour {
                r: C,
                g: 0,
                b: x.to_num(),
            }, // M -> R
        }
    }
}