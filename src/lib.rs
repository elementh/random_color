extern crate rand;

mod pick;
mod color_picker;

use rand::Rng;
use color_picker::{ColorDictionary, Color};

// #[derive(Clone, Copy, Debug)]
pub struct RandomColor {
    pub hue: Option<Color>,
    pub luminosity: Option<&'static str>,
    pub count: Option<u32>,
    pub seed: Option<i32>,
    pub format: Option<&'static str>,
    pub alpha: Option<f32>,
}
impl RandomColor {
    pub fn new() -> RandomColor {
        RandomColor {
            hue: None,
            luminosity: None,
            count: None,
            seed: None,
            format: None,
            alpha: Some(1.0),
        }
    }
    pub fn hue(&mut self, hue: &'static str) -> &mut RandomColor {
        let cd = ColorDictionary::new();
        match hue {
            "monochrome" => self.hue = Some(cd.monochrome),
            "red" => self.hue = Some(cd.red),
            "orange" => self.hue = Some(cd.orange),
            "yellow" => self.hue = Some(cd.yellow),
            "green" => self.hue = Some(cd.green),
            "blue" => self.hue = Some(cd.blue),
            "purple" => self.hue = Some(cd.purple),
            "pink" => self.hue = Some(cd.pink),
            _ => self.hue = None,
        }
        self
    }
    pub fn luminosity(&mut self, luminosity: &'static str) -> &mut RandomColor {
        match luminosity {
            "bright" => self.luminosity = Some("bright"),
            "light" => self.luminosity = Some("light"),
            "dark" => self.luminosity = Some("dark"),
            _ => self.luminosity = None,
        }
        self
    }
    pub fn count(&mut self, count: u32) -> &mut RandomColor {
        self.count = Some(count);
        self
    }
    pub fn seed<T>(&mut self, seed: T) {
        unimplemented!()
    }
    pub fn to_rgb(&self) -> String {
        let (h, s, b) = self.generate_color();
        unimplemented!()
    }
    pub fn to_rgba(&self) -> String {
        let (h, s, b) = self.generate_color();

        String::new()
    }
    pub fn to_rgb_vector(&self) -> Vec<[u32; 3]> {
        let (h, s, b) = self.generate_color();
        unimplemented!()
    }
    pub fn to_hsl(&self) -> String {
        let (h, s, b) = self.generate_color();
        unimplemented!()
    }
    pub fn to_hsla(&self) -> String {
        let (h, s, b) = self.generate_color();
        unimplemented!()
    }
    pub fn to_hsl_vector(&self) -> Vec<[u32; 3]> {
        let (h, s, b) = self.generate_color();
        unimplemented!()
    }
    pub fn to_hex(&self) -> String {
        let (h, s, b) = self.generate_color();
        unimplemented!()
    }
    fn generate_color(&self) -> (u32, u32, u32) {
        let h = self.pick_hue();
        (23, 23, 23)
    }
    fn pick_hue(&self) -> i32 {
        match self.hue {
            None => self.random_within(0, 361),
            Some(ref color) => self.random_within(color.range[0], color.range[1]),
        }
    }
    fn random_within(&self, min: i32, max: i32) -> i32 {
        match self.seed {
            None => rand::thread_rng().gen_range(min, max),
            Some(seed) => {
                // do with float
                let seed = (seed * 9301 + 49297) % 233280;
                let mut rnd = seed / 233280;
                (min + rnd * (max - min))
            }
        }
    }
    // pub fn with_hue(&mut options: RandomColor, hue: &'static str) -> RandomColor {
    //     let cd = ColorDictionary::new();
    //     match hue {
    //         "monochrome" => options.hue = Some(cd.monochrome),
    //         "red" => options.hue = Some(cd.red),
    //         "orange" => options.hue = Some(cd.orange),
    //         "yellow" => options.hue = Some(cd.yellow),
    //         "green" => options.hue = Some(cd.green),
    //         "blue" => options.hue = Some(cd.blue),
    //         "purple" => options.hue = Some(cd.purple),
    //         "pink" => options.hue = Some(cd.pink),
    //         _ => options.hue = None
    //     }
    //     (options)
    // }
    // pub fn with_luminosity(mut options: RandomColor,
    //                        luminosity: &'static str)
    //                        -> RandomColor {
    //     options.luminosity = Some(luminosity);
    //     (options)
    // }
    // pub fn with_seed(mut options: RandomColor, seed: i32) -> RandomColor {
    //     options.seed = Some(seed);
    //     (options)
    // }
    // pub fn with_alpha(mut options: RandomColor, alpha: f32) -> RandomColor {
    //     options.alpha = Some(alpha);
    //     (options)
    // }
    // pub fn generate(options: RandomColor) -> String {

    //     let hue = pick::hue(&options);

    //     let saturation = pick::saturation(&options, &hue);

    //     let value = pick::value(&options, &hue, &saturation);

    // }
    // pub fn to_hsl(randomcolor: RandomColor) -> String{
    //     let h = randomcolor.h;
    //     let s = randomcolor.s/100;
    //     let v = randomcolor.v/100;
    //     let k = (2 - s) * v;
    //
    //     let s = s * v / (k<1 ? k : 2-k) * 10000;
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
