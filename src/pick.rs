extern crate rand;

use pick::rand::Rng;
use color_dictionary::ColorDictionary;


pub fn hue() -> i32 {
    let hue = rand::thread_rng().gen_range(0, 361);
    hue
}

pub fn saturation(hue: &i32) -> i32 {
    hue + hue // TODO
}
pub fn value(hue: &i32, saturation: &i32) -> i32 {
    hue + saturation // TODO
}
