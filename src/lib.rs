extern crate rand;

pub mod color_dictionary;
pub mod options;

use color_dictionary::{ColorDictionary, ColorInformation};
use options::{Gamut, Luminosity, Seed};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[derive(Debug, PartialEq, Clone)]
pub struct RandomColor {
    pub hue: Option<ColorInformation>,
    pub luminosity: Option<Luminosity>,
    pub seed: SmallRng,
    pub alpha: Option<f32>,
    pub color_dictionary: ColorDictionary,
}

impl RandomColor {
    pub fn new() -> Self {
        RandomColor {
            hue: None,
            luminosity: None,
            seed: SmallRng::from_entropy(),
            alpha: Some(1.0),
            color_dictionary: ColorDictionary::new(),
        }
    }

    pub fn hue(&mut self, hue: Gamut) -> &mut RandomColor {
        let cd = self.color_dictionary.clone();
        
        self.hue = match hue {
            Gamut::Monochrome => Some(cd.monochrome),
            Gamut::Red => Some(cd.red),
            Gamut::Orange => Some(cd.orange),
            Gamut::Yellow => Some(cd.yellow),
            Gamut::Green => Some(cd.green),
            Gamut::Blue => Some(cd.blue),
            Gamut::Purple => Some(cd.purple),
            Gamut::Pink => Some(cd.pink),
        };

        self
    }

    pub fn luminosity(&mut self, luminosity: Luminosity) -> &mut RandomColor {
        self.luminosity = Some(luminosity);
        self
    }

    pub fn seed<T: Seed>(&mut self, seed: T) -> &mut RandomColor {
        self.seed = SmallRng::seed_from_u64(seed.to_value());

        self
    }

    pub fn alpha(&mut self, alpha: f32) -> &mut RandomColor {
        if alpha < 1.0 {
            self.alpha = Some(alpha);
        }

        self
    }

    pub fn random_alpha(&mut self) -> &mut RandomColor {
        self.alpha = None;

        self
    }

    pub fn dictionary(&mut self, dictionary: ColorDictionary) -> &mut RandomColor {
        self.color_dictionary = dictionary;

        self
    }

    pub fn to_hsv_array(&mut self) -> [u32; 3] {
        let (h, s, b) = self.generate_color();
        [h as u32, s as u32, b as u32]
    }

    pub fn to_rgb_string(&mut self) -> String {
        let (h, s, b) = self.generate_color();
        let rgb = self.hsv_to_rgb(h, s, b);

        format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2])
    }

    pub fn to_rgba_string(&mut self) -> String {
        let (h, s, b) = self.generate_color();
        let rgb = self.hsv_to_rgb(h, s, b);
        let a: f32 = match self.alpha {
            Some(alpha) => alpha,
            None => rand::random(),
        };

        format!("rgba({}, {}, {}, {})", rgb[0], rgb[1], rgb[2], a)
    }

    pub fn to_rgb_array(&mut self) -> [u8; 3] {
        let (h, s, b) = self.generate_color();
        self.hsv_to_rgb(h, s, b)
    }

    pub fn to_hsl_string(&mut self) -> String {
        let (h, s, b) = self.generate_color();
        let hsv = self.hsv_to_hsl(h, s, b);

        format!("hsl({}, {}%, {}%)", hsv[0], hsv[1], hsv[2])
    }

    pub fn to_hsla_string(&mut self) -> String {
        let (h, s, b) = self.generate_color();
        let hsv = self.hsv_to_hsl(h, s, b);
        let a: f32 = match self.alpha {
            Some(alpha) => alpha,
            None => rand::random(),
        };
        format!("hsl({}, {}%, {}%, {})", hsv[0], hsv[1], hsv[2], a)
    }

    pub fn to_hsl_array(&mut self) -> [u32; 3] {
        let (h, s, b) = self.generate_color();

        self.hsv_to_hsl(h, s, b)
    }
    pub fn to_hex(&mut self) -> String {
        let (h, s, b) = self.generate_color();
        let [r, g, b] = self.hsv_to_rgb(h, s, b);
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    fn generate_color(&mut self) -> (i64, i64, i64) {
        let h = self.pick_hue();
        let s = self.pick_saturation(&h);
        let b = self.pick_brightness(&h, &s);

        (h, s, b)
    }

    fn pick_hue(&mut self) -> i64 {
        match self.hue {
            None => self.random_within(0, 361),
            Some(ref color) => self.random_within(color.range[0], color.range[1]),
        }
    }

    fn pick_saturation(&mut self, hue: &i64) -> i64 {
        let s_range = self.color_dictionary.get_saturation_range(hue);

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

    fn pick_brightness(&mut self, hue: &i64, saturation: &i64) -> i64 {
        let b_min = self.color_dictionary.get_minimum_value(hue, saturation);
        let b_max = 100;

        match self.luminosity {
            Some(Luminosity::Random) => self.random_within(0, 100),
            Some(Luminosity::Light) => self.random_within((b_max + b_min) / 2, b_max),
            Some(Luminosity::Dark) => self.random_within(b_min, b_min + 20),
            _ => self.random_within(b_min, b_max),
        }
    }

    fn random_within(&mut self, mut min: i64, mut max: i64) -> i64 {
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        if min == max {
            max += 1;
        }

        self.seed.gen_range(min..max)
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

impl Default for RandomColor {
    fn default() -> Self {
        RandomColor::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_different_colors_using_the_same_instance() {
        let mut rc = RandomColor::new();

        assert_ne!(rc.to_rgb_string(), rc.to_rgb_string());
    }

    #[test]
    fn generates_color_as_hsv_array() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsv_array();

        assert_eq!(test_case, [191, 30, 98]);
    }
    #[test]
    fn generates_color_as_rgb_string() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgb_string();

        assert_eq!(test_case, "rgb(174, 236, 249)");
    }
    #[test]
    fn generates_color_as_rgba_string() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgba_string();

        assert_eq!(test_case, "rgba(174, 236, 249, 1)");
    }
    #[test]
    fn generates_color_as_rgb_array() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgb_array();

        assert_eq!(test_case, [174, 236, 249]);
    }
    #[test]
    fn generates_color_as_hsl_string() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsl_string();

        assert_eq!(test_case, "hsl(191, 88%, 16%)");
    }

    #[test]
    fn generates_color_as_hsla_string() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsla_string();

        assert_eq!(test_case, "hsl(191, 88%, 16%, 1)");
    }
    #[test]
    fn generates_color_as_hsl_array() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hsl_array();

        assert_eq!(test_case, [191, 88, 16]);
    }
    #[test]
    fn generates_color_as_hex() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hex();

        assert_eq!(test_case, "#aeecf9");
    }

    #[test]
    fn to_hex_is_rrggbb() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_hex();
        let [r, g, b] = RandomColor::new()
            .hue(Gamut::Blue)
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
