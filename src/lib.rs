//! Rust crate for generating random attractive colors.
//! Inspired by [RandomColor](https://github.com/davidmerfield/randomColor).
//! ### Usage
//! ```rust
//! use random_color::{Color, Luminosity, RandomColor};
//!
//! let color = RandomColor::new()
//!   .hue(Color::Blue) // Optional
//!   .luminosity(Luminosity::Light) // Optional
//!   .seed(42) // Optional
//!   .alpha(1.0) // Optional
//!   .to_hsl_string(); // 
//!
//! // color => "hsl(179, 99%, 10%)"
//! ```

extern crate rand;

mod color_dictionary;

use rand::Rng;
use color_dictionary::{ColorDictionary, ColorInformation};

pub enum Color {
    Monochrome,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink
}
#[derive(Debug, PartialEq)]
pub enum Luminosity {
    Random,
    Bright,
    Light,
    Dark
}

#[derive(Debug, PartialEq)]
pub struct RandomColor {
    /// Can take values of `Color` enum.
    pub hue: Option<ColorInformation>,
    /// Can take values of `Luminosity` enum.
    pub luminosity: Option<Luminosity>,
    /// Can take any value of `i32`.
    pub seed: Option<i32>,
    /// Can take values `f32` from 0 to 1.
    pub alpha: Option<f32>,
}
impl RandomColor {
    /// Generates a new RandomColor
    pub fn new() -> RandomColor {
        RandomColor {
            hue: None,
            luminosity: None,
            seed: None,
            alpha: Some(1.0),
        }
    }
    /// Sets `RandomColor.hue` colorspace.
    pub fn hue(&mut self, hue: Color) -> &mut RandomColor {
        let cd = ColorDictionary::new();

        self.hue = match hue {
            Color::Monochrome => Some(cd.monochrome),
            Color::Red => Some(cd.red),
            Color::Orange => Some(cd.orange),
            Color::Yellow => Some(cd.yellow),
            Color::Green => Some(cd.green),
            Color::Blue => Some(cd.blue),
            Color::Purple => Some(cd.purple),
            Color::Pink => Some(cd.pink),
        };

        self
        
    }
    /// Sets `RandomColor.luminosity`.
    pub fn luminosity(&mut self, luminosity: Luminosity) -> &mut RandomColor {
        self.luminosity = Some(luminosity);
        self
    }
    /// Sets `RandomColor.seed` used to generate a color.
    pub fn seed(&mut self, seed: i32) -> &mut RandomColor {
        self.seed = Some(seed);
        self
    }
    /// Sets `RandomColor.aplha` to the value passed if it's lower than *1.0*.
    pub fn alpha(&mut self, alpha: f32) -> &mut RandomColor {
        if alpha < 1.0 {
            self.alpha = Some(alpha);
        }
        self
    }
    /// Sets `RandomColor.alpha` to None, aka random.
    pub fn random_alpha(&mut self) -> &mut RandomColor {
        self.alpha = None;
        self
    }
    pub fn to_hsv_array(&self) -> [u32; 3] {
        let (h, s, b) = self.generate_color();
        [h as u32, s as u32, b as u32]
    }
    pub fn to_rgb_string(&self) -> String {
        let (h, s, b) = self.generate_color();
        let rgb = self.hsv_to_rgb(h, s, b);
        //'rgb(' + rgb.join(', ') + ')'
        format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2])
    }
    pub fn to_rgba_string(&self) -> String {
        let a: f32;
        let (h, s, b) = self.generate_color();
        let rgb = self.hsv_to_rgb(h, s, b);
        match self.alpha {
            Some(alpha) => a = alpha,
            None => a = rand::random(),
        }

        format!("rgba({}, {}, {}, {})", rgb[0], rgb[1], rgb[2], a)
    }
    pub fn to_rgb_array(&self) -> [u32; 3] {
        let (h, s, b) = self.generate_color();
        self.hsv_to_rgb(h, s, b)
    }
    pub fn to_hsl_string(&self) -> String {
        let (h, s, b) = self.generate_color();
        let hsv = self.hsv_to_hsl(h, s, b);

        format!("hsl({}, {}%, {}%)", hsv[0], hsv[1], hsv[2])
    }
    pub fn to_hsla_string(&self) -> String {
        let a: f32;
        let (h, s, b) = self.generate_color();
        let hsv = self.hsv_to_hsl(h, s, b);
        match self.alpha {
            Some(alpha) => a = alpha,
            None => a = rand::random(),
        }
        format!("hsl({}, {}%, {}%, {})", hsv[0], hsv[1], hsv[2], a)
    }
    pub fn to_hsl_array(&self) -> [u32; 3] {
        let (h, s, b) = self.generate_color();
        self.hsv_to_hsl(h, s, b)
    }
    pub fn to_hex(&self) -> String {
        let (h, s, b) = self.generate_color();
        format!("#{:x}{:x}{:x}", h, s, b)
    }
    fn generate_color(&self) -> (i32, i32, i32) {
        let h = self.pick_hue();
        let s = self.pick_saturation(&h);
        let b = self.pick_brightness(&h, &s);
        (h, s, b)
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
            Some(Luminosity::Random) => self.random_within(0, 100),
            Some(Luminosity::Bright) => self.random_within(55, s_max),
            Some(Luminosity::Dark) => self.random_within(s_max - 10, s_max),
            Some(Luminosity::Light) => self.random_within(s_min, 55),
            _ => self.random_within(s_min, s_max),
        }
    }
    fn pick_brightness(&self, hue: &i32, saturation: &i32) -> i32 {
        let cd = ColorDictionary::new();

        let b_min = cd.get_minimum_value(hue, saturation);
        let b_max = 100;

        match self.luminosity {
            Some(Luminosity::Random) => self.random_within(0, 100),
            Some(Luminosity::Light) => self.random_within((b_max + b_min) / 2, b_max),
            Some(Luminosity::Dark) => self.random_within(b_min, b_min + 20),
            _ => self.random_within(b_min, b_max),
        }

    }
    fn random_within(&self, mut min: i32, mut max: i32) -> i32 {
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }
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
    fn hsv_to_rgb(&self, mut hue: i32, saturation: i32, brightness: i32) -> [u32; 3] {
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

        let (r, g, b) = match h_i as i32 {
            0 => (v, t, p), 
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };

        [
            (r * 255.0).floor() as u32,
            (g * 255.0).floor() as u32,
            (b * 255.0).floor() as u32,
        ]
    }
    fn hsv_to_hsl(&self, hue: i32, saturation: i32, brightness: i32) -> [u32; 3] {
        let h = hue;
        let s = saturation as f32 / 100.0;
        let v = brightness as f32 / 100.0;
        let mut k = (2.0 - s) * v;

        if k > 1.0 {
            k = 2.0 - k;
        }

        [
            h as u32,
            ((s * v / k * 10000.0) / 100.0) as u32,
            (k / 2.0 * 100.0) as u32,
        ]
    }
}

#[cfg(test)]
mod tests {
    use RandomColor;
    use color_dictionary::ColorDictionary;

    use Color;
    use Luminosity;

    #[test]
    fn accept_values() {
        let cd = ColorDictionary::new();
        let test_case = RandomColor {
            hue: Some(cd.blue),
            luminosity: Some(Luminosity::Light),
            seed: Some(42),
            alpha: Some(1.0),
        }.to_hsl_string();

        let rc = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsl_string();

        assert_eq!(test_case, rc);
    }
    #[test]
    fn generates_color_as_hsv_array() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsv_array();

        assert_eq!(test_case, [179, 20, 100]);
    }
    #[test]
    fn generates_color_as_rgb_string() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgb_string();

        assert_eq!(test_case, "rgb(204, 255, 254)");
    }
    #[test]
    fn generates_color_as_rgba_string() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgba_string();

        assert_eq!(test_case, "rgba(204, 255, 254, 1)");
    }
    #[test]
    fn generates_color_as_rgb_array() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgb_array();

        assert_eq!(test_case, [204, 255, 254]);
    }
    #[test]
    fn generates_color_as_hsl_string() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsl_string();

        assert_eq!(test_case, "hsl(179, 99%, 10%)");
    }
    #[test]
    fn generates_color_as_hsla_string() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsla_string();

        assert_eq!(test_case, "hsl(179, 99%, 10%, 1)");
    }
    #[test]
    fn generates_color_as_hsl_array() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsl_array();

        assert_eq!(test_case, [179, 99, 10]);
    }
    #[test]
    fn generates_color_as_hex() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hex();

        assert_eq!(test_case, "#b31464");
    }
}
