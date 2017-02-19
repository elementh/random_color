extern crate rand;

use pick::rand::Rng;
use color_dictionary::ColorDictionary;


pub fn hue() -> i32 {
    let hue = rand::thread_rng().gen_range(0, 361);
    hue
}

pub fn saturation(hue: &i32) -> i32 {
    let cd = ColorDictionary::new();
    let s_range = cd.get_saturation_range(hue);
    
    // let mut s_min = s_range.0;
    // let mut s_max = s_range.1;
    let s_min = s_range.0;
    let s_max = s_range.1;
    
    rand::thread_rng().gen_range(s_min, s_max)
}
pub fn value(hue: &i32, saturation: &i32) -> i32 {
    let cd = ColorDictionary::new();
    
    let v_min = cd.get_minimum_value(hue, saturation);
    let v_max = 100;
    
    rand::thread_rng().gen_range(v_min, v_max)
}
