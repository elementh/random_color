extern crate rand;

use pick::rand::Rng;

pub fn hue() -> u32 {
    let hue = rand::thread_rng().gen_range(0, 361);
    (hue)
}

pub fn saturation(hue: &u32) -> u32 {
    hue + hue // TODO
}
pub fn value(hue: &u32, saturation: &u32) -> u32 {
    hue + saturation // TODO
}
