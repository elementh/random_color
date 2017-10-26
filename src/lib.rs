extern crate rand;

mod color_dictionary;

use rand::Rng;
use color_dictionary::{ColorDictionary, Color};

// #[derive(Clone, Copy, Debug)]
pub struct RandomColor {
    pub hue: Option<Color>,
    pub luminosity: Option<&'static str>,
    // pub count: Option<u32>,
    pub seed: Option<i32>,
    pub alpha: Option<f32>,
}
impl RandomColor {
    pub fn new() -> RandomColor {
        RandomColor {
            hue: None,
            luminosity: None,
            // count: None,
            seed: None,
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
            "random" => self.luminosity = Some("random"),
            "bright" => self.luminosity = Some("bright"),
            "light" => self.luminosity = Some("light"),
            "dark" => self.luminosity = Some("dark"),
            _ => self.luminosity = None,
        }
        self
    }
    // pub fn count(&mut self, count: u32) -> &mut RandomColor {
    //     self.count = Some(count);
    //     self
    // }
    pub fn seed(&mut self, seed: i32) -> &mut RandomColor {
        self.seed = Some(seed);
        self
    }
    pub fn alpha(&mut self, alpha: f32) -> &mut RandomColor {
        if alpha < 1.0 {
            self.alpha = Some(alpha);
        }
        self
    }
    pub fn to_rgb(&self) -> String {
        let (h, s, b) = self.generate_color();
        unimplemented!()
    }
    pub fn to_rgba(&self) -> String {
        let (h, s, b) = self.generate_color();

        String::new()
    }
    pub fn to_rgb_array(&self) -> [u32; 3] {
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
    pub fn to_hsl_array(&self) -> [u32; 3] {
        let (h, s, b) = self.generate_color();
        unimplemented!()
    }
    pub fn to_hex(&self) -> String {
        let (h, s, b) = self.generate_color();
        unimplemented!()
    }
    fn generate_color(&self) -> (i32, i32, i32) {
        let h = self.pick_hue();
        let s = self.pick_saturation(&h);
        let b = self.pick_brightness(&h, &s);
        (23, 23, 23)
    }
    fn pick_hue(&self) -> i32 {
        match self.hue {
            None => self.random_within(0, 361),
            Some(ref color) => self.random_within(color.range[0], color.range[1]),
        }
    }
    fn pick_saturation(&self, hue: &i32) -> i32 {
        let cd = ColorDictionary::new();
        let s_range = cd.get_saturation_range(hue);

        let s_min = s_range.0;
        let s_max = s_range.1;

        match self.luminosity {
            None => self.random_within(s_min, s_max),
            Some("random") => self.random_within(0, 100),
            Some("bright") => self.random_within(55, s_max),
            Some("dark") => self.random_within(s_max - 10, s_max),
            Some("light") => self.random_within(s_min, 55),
            _ => self.random_within(s_min, s_max), // maybe error??
        }
    }
    fn pick_brightness(&self, hue: &i32, saturation: &i32) -> i32 {
        let cd = ColorDictionary::new();

        let b_min = cd.get_minimum_value(hue, saturation);
        let b_max = 100;

        match self.luminosity {
            None => self.random_within(b_min, b_max),
            Some("random") => self.random_within(0, 100),
            Some("light") => self.random_within((b_max + b_min) / 2, b_max),
            Some("dark") => self.random_within(b_min, b_min + 20),
            _ => self.random_within(b_min, b_max),
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
    fn hsv_to_rgb(mut hue: i32, saturation: i32, brightness: i32) -> [u32; 3] {
        let r: f32;
        let g: f32;
        let b: f32;

        if hue == 0 {
            hue = 1;
        }
        if hue == 360 {
            hue = 359;
        }

        let h: f32 = hue as f32 / 360.0;
        let s: f32 = saturation as f32 / 100.0;
        let v: f32 = brightness as f32 / 100.0;

        let h_i = (h * 6.0).floor();
        let f = h * 6.0 - h_i;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);


        match h_i as i32 {
            0 => {
                r = v;
                g = t;
                b = p;
            }
            1 => {
                r = q;
                g = v;
                b = p;
            }
            2 => {
                r = p;
                g = v;
                b = t;
            }
            3 => {
                r = p;
                g = q;
                b = v;
            }
            4 => {
                r = t;
                g = p;
                b = v;
            }
            5 => {
                r = v;
                g = p;
                b = q;
            }  
            _ => {
                r = v;
                g = p;
                b = q;
            }
        }
        
        [(r * 255.0).floor() as u32,(r * 255.0).floor() as u32,(r * 255.0).floor() as u32]

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
