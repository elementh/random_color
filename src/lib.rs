//! A library for generating attractive random colors with a variety of options.
//! Inspired by [randomColor](https://github.com/davidmerfield/randomColor).
//!
//! # Examples
//!
//! ```rust
//! use random_color::RandomColor;
//!
//! let mut random_color = RandomColor::new();
//!
//! let color = random_color.to_hex();
//! println!("{}", color);
//! ```
//!
//! ```rust
//! use random_color::RandomColor;
//! use random_color::options::{Gamut, Luminosity};
//!
//! let mut random_color = RandomColor{
//!     hue: Some(Gamut::Blue),
//!     luminosity: Some(Luminosity::Dark),
//!     ..Default::default()
//! };
//!
//! let color = random_color.to_hsl_string();
//! println!("{}", color);
//! ```
//!
//! ```rust
//! use random_color::RandomColor;
//!
//! let mut random_color = RandomColor::new();
//!
//! random_color.seed("A random seed");
//!
//! let color = random_color.to_rgb_string();
//! println!("{}", color);
//! ```
#[cfg(feature = "ecolor_support")]
extern crate ecolor;
#[cfg(feature = "palette_support")]
extern crate palette;
extern crate rand;
#[cfg(feature = "rgb_support")]
extern crate rgb;

pub mod color_dictionary;
pub mod options;

use color_dictionary::ColorDictionary;
use ecolor::{Color32, Rgba};
use options::{Gamut, Luminosity, Seed};
#[cfg(feature = "palette_support")]
use palette::{Srgb, Srgba};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
#[cfg(feature = "rgb_support")]
use rgb::Rgb;

/// A structure for generating random colors with a variety of options.
///
/// The available options are:
/// * `hue`: Specify a specific hue, or a range of hues. You can use the
///   `Gamut` enum to select a hue.
/// * `luminosity`: Specify a specific luminosity, or a range of luminosities.
///   You can use the `Luminosity` enum to select a luminosity.
/// * `seed`: Specify a seed for the random number generator. If you don't
///   specify a seed, one will be generated randomly.
/// * `alpha`: Specify an alpha value for the generated color. If you don't
///   specify an alpha value, 1.0 will be used.
/// * `color_dictionary`: Specify a custom color dictionary. If you don't
///   specify a color dictionary, the default one will be used.
#[derive(Debug, PartialEq, Clone)]
pub struct RandomColor {
    /// The hue of the color to generate.
    pub hue: Option<Gamut>,
    /// The luminosity of the color to generate.
    pub luminosity: Option<Luminosity>,
    /// The seed for the random number generator.
    pub seed: SmallRng,
    /// The alpha value of the color to generate.
    pub alpha: Option<f32>,
    /// The color dictionary to use.
    pub color_dictionary: ColorDictionary,
}

impl RandomColor {
    /// Creates a new `RandomColor` instance.
    pub fn new() -> Self {
        RandomColor {
            hue: None,
            luminosity: None,
            seed: SmallRng::from_entropy(),
            alpha: Some(1.0),
            color_dictionary: ColorDictionary::new(),
        }
    }

    /// Sets the hue setting.
    pub fn hue(&mut self, hue: Gamut) -> &mut RandomColor {
        self.hue = Some(hue);

        self
    }

    /// Removes the luminosity setting.
    pub fn luminosity(&mut self, luminosity: Luminosity) -> &mut RandomColor {
        self.luminosity = Some(luminosity);
        self
    }

    /// Sets the seed.
    pub fn seed<T: Seed>(&mut self, seed: T) -> &mut RandomColor {
        self.seed = SmallRng::seed_from_u64(seed.to_value());

        self
    }

    /// Sets the alpha setting.
    pub fn alpha(&mut self, alpha: f32) -> &mut RandomColor {
        if alpha < 1.0 {
            self.alpha = Some(alpha);
        }

        self
    }

    /// Removes the alpha setting.
    pub fn random_alpha(&mut self) -> &mut RandomColor {
        self.alpha = None;

        self
    }

    /// Sets the ColorDictionary.
    pub fn dictionary(&mut self, dictionary: ColorDictionary) -> &mut RandomColor {
        self.color_dictionary = dictionary;

        self
    }

    /// Generates a random color and returns it as an HSV array.
    pub fn to_hsv_array(&mut self) -> [u32; 3] {
        let (h, s, b) = self.generate_color();

        [h as u32, s as u32, b as u32]
    }

    /// Generates a random color and returns it as an RGB string.
    pub fn to_rgb_string(&mut self) -> String {
        let (h, s, b) = self.generate_color();
        let rgb = self.hsv_to_rgb(h, s, b);

        format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2])
    }

    /// Generates a random color and returns it as an RGBA string.
    pub fn to_rgba_string(&mut self) -> String {
        let (h, s, b) = self.generate_color();
        let rgb = self.hsv_to_rgb(h, s, b);
        let a: f32 = match self.alpha {
            Some(alpha) => alpha,
            None => rand::random(),
        };

        format!("rgba({}, {}, {}, {})", rgb[0], rgb[1], rgb[2], a)
    }

    /// Generates a random color and returns it as an RGB array.
    pub fn to_rgb_array(&mut self) -> [u8; 3] {
        let (h, s, b) = self.generate_color();

        self.hsv_to_rgb(h, s, b)
    }

    /// Generates a random color and returns it as an RGB array.
    pub fn to_rgba_array(&mut self) -> [u8; 4] {
        let (h, s, b) = self.generate_color();
        let rgb: [u8; 3] = self.hsv_to_rgb(h, s, b);

        [
            rgb[0],
            rgb[1],
            rgb[2],
            (self.alpha.unwrap_or(1.0) * 255.0).round() as u8,
        ]
    }

    /// Generates a random color and returns it as a `f32` RGB array.
    pub fn to_f32_rgb_array(&mut self) -> [f32; 3] {
        let (h, s, b) = self.generate_color();
        let rgb: [u8; 3] = self.hsv_to_rgb(h, s, b);

        [
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
        ]
    }

    /// Generates a random color and returns it as an `f32` RGBA array.
    pub fn to_f32_rgba_array(&mut self) -> [f32; 4] {
        let (h, s, b) = self.generate_color();
        let rgb: [u8; 3] = self.hsv_to_rgb(h, s, b);

        [
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
            self.alpha.unwrap_or(1.0),
        ]
    }

    /// Generates a random color and returns it as an HSL string.
    pub fn to_hsl_string(&mut self) -> String {
        let (h, s, b) = self.generate_color();
        let hsv = self.hsv_to_hsl(h, s, b);

        format!("hsl({}, {}%, {}%)", hsv[0], hsv[1], hsv[2])
    }

    /// Generates a random color and returns it as an HSLA string.
    pub fn to_hsla_string(&mut self) -> String {
        let (h, s, b) = self.generate_color();
        let hsv = self.hsv_to_hsl(h, s, b);
        let a: f32 = match self.alpha {
            Some(alpha) => alpha,
            None => rand::random(),
        };

        format!("hsl({}, {}%, {}%, {})", hsv[0], hsv[1], hsv[2], a)
    }

    /// Generates a random color and returns it as an HSL array.
    pub fn to_hsl_array(&mut self) -> [u32; 3] {
        let (h, s, b) = self.generate_color();

        self.hsv_to_hsl(h, s, b)
    }

    /// Generates a random color and returns it as a hex string.
    pub fn to_hex(&mut self) -> String {
        let (h, s, b) = self.generate_color();
        let [r, g, b] = self.hsv_to_rgb(h, s, b);

        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Transforms the `RandomColor` into a `f32` array with the color's RGB values.
    pub fn into_f32_rgb_array(self) -> [f32; 3] {
        self.clone().to_f32_rgb_array()
    }

    /// Transforms the `RandomColor` into a `f32` array with the color's RGBA values.
    pub fn into_f32_rgba_array(self) -> [f32; 4] {
        self.clone().to_f32_rgba_array()
    }

    /// Transforms the `RandomColor` into a `u8` array with the color's RGB values.
    pub fn into_rgb_array(self) -> [u8; 3] {
        self.clone().to_rgb_array()
    }

    /// Transforms the `RandomColor` into a `u8` array with the color's RGBA values.
    pub fn into_rgba_array(self) -> [u8; 4] {
        self.clone().to_rgba_array()
    }

    /// Generates a random color based on the settings.
    fn generate_color(&mut self) -> (i64, i64, i64) {
        let h = self.pick_hue();
        let s = self.pick_saturation(&h);
        let b = self.pick_brightness(&h, &s);

        (h, s, b)
    }

    /// Picks a random hue based on the hue setting.
    fn pick_hue(&mut self) -> i64 {
        match self.hue {
            None => self.random_within(0, 361),
            Some(ref gamut) => {
                let color = self.color_dictionary.get_color_from_gamut(gamut);
                self.random_within(color.range[0], color.range[1])
            }
        }
    }

    /// Picks a random saturation value based on the hue and luminosity setting.
    ///
    /// Parameters:
    /// * `hue`: The hue of the color.
    fn pick_saturation(&mut self, hue: &i64) -> i64 {
        let s_range: (i64, i64) = self.color_dictionary.get_saturation_range(hue);

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

    /// Picks a random brightness value based on the hue and saturation, as well
    /// as the luminosity setting.
    ///
    /// Parameters:
    /// * `hue`: The hue of the color.
    /// * `saturation`: The saturation of the color.
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

    /// Generates a random i64 within the given range.
    ///
    /// This function first ensures that `min` is less than or equal to `max`.
    /// If `min` is equal to `max`, it increments `max` by 1 to ensure that the
    /// range is not empty. It uses the `SmallRng` in the seed property to
    /// generate the random number.
    ///
    /// Parameters:
    /// * `min`: The minimum value of the range.
    /// * `max`: The maximum value of the range.
    fn random_within(&mut self, mut min: i64, mut max: i64) -> i64 {
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        if min == max {
            max += 1;
        }

        self.seed.gen_range(min..max)
    }

    /// Convert a color from HSV to RGB.
    ///
    /// Parameters:
    /// * `hue`: The hue of the color in the range [0, 360).
    /// * `saturation`: The saturation of the color in the range [0, 100].
    /// * `brightness`: The brightness of the color in the range [0, 100].
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

    /// Convert a color from HSV to HSL.
    ///
    /// Parameters:
    /// * `hue`: The hue of the color in the range [0, 360).
    /// * `saturation`: The saturation of the color in the range [0, 100].
    /// * `brightness`: The brightness of the color in the range [0, 100].
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

    /* Optional Features */

    /* `rgb` crate support */

    /// Generates a random color and returns it as an `Rgb` struct from the `rgb` crate.
    #[cfg(feature = "rgb_support")]
    pub fn to_rgb(&mut self) -> Rgb<u8> {
        let rgb = self.to_rgb_array();

        Rgb {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }

    /// Generates a random color and returns it as an `Rgba` struct from the `rgb` crate.
    #[cfg(feature = "rgb_support")]
    pub fn to_rgba(&mut self) -> rgb::Rgba<u8> {
        let rgb = self.to_rgb_array();

        let alpha = match self.alpha {
            Some(alpha) => (alpha * 255.0) as u8,
            None => self.random_within(0, 255) as u8,
        };

        rgb::Rgba {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
            a: alpha,
        }
    }
}

impl Default for RandomColor {
    fn default() -> Self {
        RandomColor::new()
    }
}

#[cfg(feature = "palette_support")]
impl From<RandomColor> for Srgba {
    fn from(value: RandomColor) -> Self {
        let rgba = value.into_f32_rgba_array();

        Srgba::new(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

#[cfg(feature = "palette_support")]
impl From<&mut RandomColor> for Srgba {
    fn from(value: &mut RandomColor) -> Self {
        let rgba = value.to_f32_rgba_array();

        Srgba::new(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

#[cfg(feature = "palette_support")]
impl From<RandomColor> for Srgb {
    fn from(value: RandomColor) -> Self {
        let rgb = value.into_f32_rgb_array();

        Srgb::new(rgb[0], rgb[1], rgb[2])
    }
}

#[cfg(feature = "palette_support")]
impl From<&mut RandomColor> for Srgb {
    fn from(value: &mut RandomColor) -> Self {
        let rgb = value.to_f32_rgb_array();

        Srgb::new(rgb[0], rgb[1], rgb[2])
    }
}

#[cfg(feature = "ecolor_support")]
impl From<RandomColor> for Color32 {
    fn from(value: RandomColor) -> Self {
        let rgba = value.into_rgba_array();
        Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

#[cfg(feature = "ecolor_support")]
impl From<&mut RandomColor> for Color32 {
    fn from(value: &mut RandomColor) -> Self {
        let rgba = value.to_rgba_array();
        Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

#[cfg(feature = "ecolor_support")]
impl From<RandomColor> for Rgba {
    fn from(value: RandomColor) -> Self {
        let rgba = value.into_f32_rgba_array();
        Rgba::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

#[cfg(feature = "ecolor_support")]
impl From<&mut RandomColor> for Rgba {
    fn from(value: &mut RandomColor) -> Self {
        let rgba = value.to_f32_rgba_array();
        Rgba::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3])
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
    fn generates_color_as_f32_rgb_array() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_f32_rgb_array();

        assert_eq!(test_case, [0.68235296, 0.9254902, 0.9764706]);
    }

    #[test]
    fn generates_color_as_f32_rgba_array() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_f32_rgba_array();

        assert_eq!(test_case, [0.68235296, 0.9254902, 0.9764706, 1.0]);
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

    /* Optional Feature Tests */

    #[test]
    #[cfg(feature = "rgb_support")]
    fn generates_color_as_rgb_from_rgb_crate() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0)
            .to_rgb();

        assert_eq!(test_case, Rgb::new(174, 236, 249));
    }

    #[test]
    #[cfg(feature = "rgb_support")]
    fn generates_color_as_rgba_from_rgb_crate() {
        let test_case = RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(0.69)
            .to_rgba();

        assert_eq!(test_case, rgb::Rgba::new(174, 236, 249, 175));
    }

    #[test]
    #[cfg(feature = "palette_support")]
    fn can_be_transformed_into_srgba_from_palette_crate() {
        let mut rc = RandomColor::new();

        let test_case = rc
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0);

        let converted = Srgba::from(test_case);

        assert_eq!(
            converted.into_components(),
            (0.68235296, 0.9254902, 0.9764706, 1.0)
        );
    }

    #[test]
    #[cfg(feature = "palette_support")]
    fn can_be_transformed_into_srgb_from_palette_crate() {
        let mut rc = RandomColor::new();

        let test_case = rc
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0);

        let converted = Srgb::from(test_case);

        assert_eq!(
            converted.into_components(),
            (0.68235296, 0.9254902, 0.9764706)
        );
    }

    #[test]
    #[cfg(feature = "ecolor_support")]
    fn can_be_transformed_into_color32_from_ecolor_crate() {
        let mut rc = RandomColor::new();

        let test_case = rc
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0);

        let converted = Color32::from(test_case);

        assert_eq!(converted.to_array(), [174, 236, 249, 255]);
    }

    #[test]
    #[cfg(feature = "ecolor_support")]
    fn can_be_transformed_into_rgba_from_ecolor_crate() {
        let mut rc = RandomColor::new();

        let test_case = rc
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .seed(42)
            .alpha(1.0);

        let converted = Rgba::from(test_case);

        assert_eq!(converted.to_array(), [0.68235296, 0.9254902, 0.9764706, 1.0]);
    }
}
