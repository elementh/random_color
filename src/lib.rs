//! Rust crate for generating random attractive colors.
//! Inspired by [RandomColor](https://github.com/davidmerfield/randomColor).
//! ### Usage
//! ```rust
//! use random_color::color_dictionary::{ColorDictionary, ColorInformation};
//! use random_color::{Color, Luminosity, RandomColor};
//!
//! let color = RandomColor::new()
//!   .hue(Color::Blue) // Optional
//!   .luminosity(Luminosity::Light) // Optional
//!   .seed(42) // Optional
//!   .alpha(1.0) // Optional
//!   .dictionary(ColorDictionary::new())
//!   .to_hsl_string(); //
//!
//! // color => "hsl(179, 99%, 10%)"
//! ```

extern crate rand;

pub mod color_dictionary;

use color_dictionary::{ColorDictionary, ColorInformation};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Monochrome,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Luminosity {
    Random,
    Bright,
    Light,
    Dark,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RandomColor {
    /// Can take values of `Color` enum.
    pub hue: Option<ColorInformation>,
    /// Can take values of `Luminosity` enum.
    pub luminosity: Option<Luminosity>,
    /// Can take any value of `i64`.
    pub seed: Option<u64>,
    /// Can take values `f32` from 0 to 1.
    pub alpha: Option<f32>,
    /// Optional, bring your own dictionary
    pub color_dictionary: Option<ColorDictionary>,
}

impl Default for RandomColor {
    fn default() -> Self {
        RandomColor {
            hue: None,
            luminosity: None,
            seed: None,
            alpha: Some(1.0),
            color_dictionary: Some(ColorDictionary::new()),
        }
    }
}

impl RandomColor {
    /// Generates a new RandomColor
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `RandomColor.hue` colorspace.
    pub fn hue(&mut self, hue: Color) -> &mut RandomColor {
        let cd = match &self.color_dictionary {
            Some(color_dict) => color_dict.clone(),
            None => ColorDictionary::new(),
        };

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
    pub fn seed<T: Seed>(&mut self, seed: T) -> &mut RandomColor {
        self.seed = Some(seed.to_value());
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

    pub fn dictionary(&mut self, dictionary: ColorDictionary) -> &mut RandomColor {
        self.color_dictionary = Some(dictionary);
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
        let (h, s, b) = self.generate_color();
        let rgb = self.hsv_to_rgb(h, s, b);
        let a: f32 = match self.alpha {
            Some(alpha) => alpha,
            None => rand::random(),
        };

        format!("rgba({}, {}, {}, {})", rgb[0], rgb[1], rgb[2], a)
    }

    pub fn to_rgb_array(&self) -> [u8; 3] {
        let (h, s, b) = self.generate_color();
        self.hsv_to_rgb(h, s, b)
    }

    pub fn to_hsl_string(&self) -> String {
        let (h, s, b) = self.generate_color();
        let hsv = self.hsv_to_hsl(h, s, b);

        format!("hsl({}, {}%, {}%)", hsv[0], hsv[1], hsv[2])
    }

    pub fn to_hsla_string(&self) -> String {
        let (h, s, b) = self.generate_color();
        let hsv = self.hsv_to_hsl(h, s, b);
        let a: f32 = match self.alpha {
            Some(alpha) => alpha,
            None => rand::random(),
        };
        format!("hsl({}, {}%, {}%, {})", hsv[0], hsv[1], hsv[2], a)
    }

    pub fn to_hsl_array(&self) -> [u32; 3] {
        let (h, s, b) = self.generate_color();

        self.hsv_to_hsl(h, s, b)
    }
    pub fn to_hex(&self) -> String {
        let (h, s, b) = self.generate_color();
        let [r, g, b] = self.hsv_to_rgb(h, s, b);
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    fn generate_color(&self) -> (i64, i64, i64) {
        let h = self.pick_hue();
        let s = self.pick_saturation(&h);
        let b = self.pick_brightness(&h, &s);

        (h, s, b)
    }

    fn pick_hue(&self) -> i64 {
        match self.hue {
            None => self.random_within(0, 361),
            Some(ref color) => self.random_within(color.range[0], color.range[1]),
        }
    }

    fn pick_saturation(&self, hue: &i64) -> i64 {
        let cd = match &self.color_dictionary {
            Some(color_dict) => color_dict.clone(),
            None => ColorDictionary::new(),
        };

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

    fn pick_brightness(&self, hue: &i64, saturation: &i64) -> i64 {
        let cd = match &self.color_dictionary {
            Some(color_dict) => color_dict.clone(),
            None => ColorDictionary::new(),
        };

        let b_min = cd.get_minimum_value(hue, saturation);
        let b_max = 100;

        match self.luminosity {
            Some(Luminosity::Random) => self.random_within(0, 100),
            Some(Luminosity::Light) => self.random_within((b_max + b_min) / 2, b_max),
            Some(Luminosity::Dark) => self.random_within(b_min, b_min + 20),
            _ => self.random_within(b_min, b_max),
        }
    }

    fn random_within(&self, mut min: i64, mut max: i64) -> i64 {
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        if min == max {
            max += 1;
        }

        match self.seed {
            None => SmallRng::from_entropy().gen_range(min..max),
            Some(seed) => SmallRng::seed_from_u64(seed).gen_range(min..max),
        }
    }

    fn hsv_to_rgb(&self, mut hue: i64, saturation: i64, brightness: i64) -> [u8; 3] {
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

        let (r, g, b) = match h_i as i64 {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        };

        [
            (r * 255.0).floor() as u8,
            (g * 255.0).floor() as u8,
            (b * 255.0).floor() as u8,
        ]
    }

    fn hsv_to_hsl(&self, hue: i64, saturation: i64, brightness: i64) -> [u32; 3] {
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

pub trait Seed {
    fn to_value(self) -> u64;
}

impl Seed for i64 {
    fn to_value(self) -> u64 {
        self as u64
    }
}

impl Seed for i32 {
    fn to_value(self) -> u64 {
        self as u64
    }
}

impl Seed for u64 {
    fn to_value(self) -> u64 {
        self
    }
}

impl Seed for u32 {
    fn to_value(self) -> u64 {
        self as u64
    }
}

impl Seed for String {
    fn to_value(self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Seed for &String {
    fn to_value(self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Seed for &str {
    fn to_value(self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use crate::color_dictionary::ColorDictionary;
    use crate::Color;
    use crate::Luminosity;
    use crate::RandomColor;

    #[test]
    fn accept_values() {
        let cd = ColorDictionary::new();
        let test_case = RandomColor {
            hue: Some(cd.blue),
            luminosity: Some(Luminosity::Light),
            seed: Some(42),
            alpha: Some(1.0),
            color_dictionary: Some(ColorDictionary::new()),
        }
        .to_hsl_string();

        let rc = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsl_string();

        assert_eq!(test_case, rc);
    }

    #[test]
    fn seed_by_string() {
        let cd = ColorDictionary::new();

        let mut hasher = DefaultHasher::new();
        "New Seed".to_string().hash(&mut hasher);
        let hash = hasher.finish();

        let test_case = RandomColor {
            hue: Some(cd.blue),
            luminosity: Some(Luminosity::Light),
            seed: Some(hash),
            alpha: Some(1.0),
            color_dictionary: Some(ColorDictionary::new()),
        }
        .to_hsl_string();

        let rc = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed("New Seed".to_string())
            .alpha(1.0)
            .to_hsl_string();

        assert_eq!(test_case, rc);
    }

    #[test]
    fn seed_by_u64() {
        let cd = ColorDictionary::new();

        let test_case = RandomColor {
            hue: Some(cd.blue),
            luminosity: Some(Luminosity::Light),
            seed: Some(12345u64),
            alpha: Some(1.0),
            color_dictionary: Some(ColorDictionary::new()),
        }
        .to_hsl_string();

        let rc = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(12345u64)
            .alpha(1.0)
            .to_hsl_string();

        assert_eq!(test_case, rc);
    }

    #[test]
    fn seed_by_i64() {
        let cd = ColorDictionary::new();

        let test_case = RandomColor {
            hue: Some(cd.blue),
            luminosity: Some(Luminosity::Light),
            seed: Some(12345u64),
            alpha: Some(1.0),
            color_dictionary: Some(ColorDictionary::new()),
        }
        .to_hsl_string();

        let rc = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(12345i64)
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

        assert_eq!(test_case, [191, 30, 94]);
    }
    #[test]
    fn generates_color_as_rgb_string() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgb_string();

        assert_eq!(test_case, "rgb(167, 226, 239)");
    }
    #[test]
    fn generates_color_as_rgba_string() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgba_string();

        assert_eq!(test_case, "rgba(167, 226, 239, 1)");
    }
    #[test]
    fn generates_color_as_rgb_array() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgb_array();

        assert_eq!(test_case, [167, 226, 239]);
    }
    #[test]
    fn generates_color_as_hsl_string() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsl_string();

        assert_eq!(test_case, "hsl(191, 70%, 20%)");
    }

    #[test]
    fn generates_color_as_hsla_string() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsla_string();

        assert_eq!(test_case, "hsl(191, 70%, 20%, 1)");
    }
    #[test]
    fn generates_color_as_hsl_array() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsl_array();

        assert_eq!(test_case, [191, 70, 20]);
    }
    #[test]
    fn generates_color_as_hex() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hex();

        assert_eq!(test_case, "#a7e2ef");
    }

    #[test]
    fn to_hex_is_rrggbb() {
        let test_case = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hex();
        let [r, g, b] = RandomColor::new()
            .hue(Color::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgb_array();

        assert_eq!(test_case, format!("#{:02x}{:02x}{:02x}", r, g, b).as_str());
    }

    #[test]
    fn single_digit_hex_are_padded_by_to_two_chars() {
        let test_case = RandomColor::new()
            .luminosity(Luminosity::Dark)
            .seed(5)
            .to_hex();

        assert_eq!(test_case, "#3e0496");
    }
}
