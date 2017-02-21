extern crate rand;

use pick::rand::Rng;
use random_color_options::RandomColorOptions;
use random_color_options::ColorDictionary;
use random_color_options::Color;

pub fn hue(options: &RandomColorOptions) -> i32 {
    match options.hue {
        None => random_within(options, 0, 361),
        Some(ref color) => random_within(options, color.range[0], color.range[1]),
    }
}

pub fn saturation(options: &RandomColorOptions, hue: &i32) -> i32 {
    let cd = ColorDictionary::new();
    let s_range = cd.get_saturation_range(hue);

    // let mut s_min = s_range.0;
    // let mut s_max = s_range.1;
    let s_min = s_range.0;
    let s_max = s_range.1;
    
    match options.luminosity {
        None => random_within(options, s_min, s_max),
        Some("random") => random_within(options, 0, 100),
        Some("bright") => random_within(options, 55, s_max),
        Some("dark") => random_within(options, s_max-10, s_max),
        Some("light") => random_within(options, s_min, 55),
        _ => random_within(options, s_min, s_max), // maybe error??
    }


}
pub fn value(options: &RandomColorOptions, hue: &i32, saturation: &i32) -> i32 {
    let cd = ColorDictionary::new();

    let v_min = cd.get_minimum_value(hue, saturation);
    let v_max = 100;

    rand::thread_rng().gen_range(v_min, v_max)
}


fn random_within(options: &RandomColorOptions, min: i32, max: i32) -> i32 {
    match options.seed {
        None => rand::thread_rng().gen_range(min, max),
        Some(seed) => {
            // do with float
            let seed = (seed * 9301 + 49297) % 233280;
            let mut rnd = seed / 233280;
            (min + rnd * (max - min))
        }
    }
}
