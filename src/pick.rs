extern crate rand;


use pick::rand::Rng;
// use color_dictionary::ColorDictionary;

pub fn hue() -> u32 {
    hue = rand::thread_rng().gen_range(0, 361);
}

pub fn saturation(hue: &u32) -> u32 {
    hue + hue // TODO
}
pub fn value(hue: &u32, saturation: &u32) -> u32 {
    hue + saturation // TODO
}
