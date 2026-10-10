//! A library for generating attractive random colors with a variety of options.
//! Inspired by [randomColor](https://github.com/davidmerfield/randomColor).
//!
//! # Examples
//!
//! ```rust
//! use random_color::RandomColor;
//!
//! let random_color = RandomColor::new();
//!
//! let color = random_color.random_hex(&mut rand::rng());
//! println!("{}", color);
//! ```
//!
//! ```rust
//! use random_color::RandomColor;
//! use random_color::options::{Gamut, Luminosity};
//!
//! let random_color = RandomColor::new()
//!     .hue(Gamut::Blue)
//!     .luminosity(Luminosity::Dark);
//!
//! let color = random_color.random_hsl_string(&mut rand::rng());
//! println!("{}", color);
//! ```
//!
//! ```rust
//! use rand::rngs::SmallRng;
//! use rand::SeedableRng as _;
//! use random_color::RandomColor;
//!
//! let random_color = RandomColor::new();
//!
//! let mut rng = SmallRng::seed_from_u64(42);
//!
//! let color = random_color.random_rgb_string(&mut rng);
//! println!("{}", color);
//! ```

pub mod color_dictionary;
pub mod options;

use color_dictionary::ColorDictionary;
use options::{Gamut, Luminosity};

use rand::Rng;
use rand::RngExt as _;

/// A structure for generating random colors with a variety of options.
///
/// The available options are:
/// * `hue`: Specify a specific hue, or a range of hues. You can use the
///   `Gamut` enum to select a hue.
/// * `luminosity`: Specify a specific luminosity, or a range of luminosities.
///   You can use the `Luminosity` enum to select a luminosity.
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
    /// The alpha value of the color to generate.
    pub alpha: Option<f32>,
    /// The color dictionary to use.
    pub color_dictionary: ColorDictionary,
}

impl Default for RandomColor {
    fn default() -> Self {
        RandomColor {
            hue: None,
            luminosity: None,
            alpha: Some(1.0),
            color_dictionary: ColorDictionary::new(),
        }
    }
}

impl RandomColor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the hue setting.
    pub fn hue(mut self, hue: Gamut) -> Self {
        self.hue = Some(hue);

        self
    }

    /// Removes the luminosity setting.
    pub fn luminosity(mut self, luminosity: Luminosity) -> Self {
        self.luminosity = Some(luminosity);
        self
    }

    /// Sets the alpha setting.
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(alpha.clamp(0.0, 1.0));

        self
    }

    /// Removes the alpha setting.
    pub fn random_alpha(mut self) -> Self {
        self.alpha = None;

        self
    }

    /// Sets the ColorDictionary.
    pub fn dictionary(mut self, dictionary: ColorDictionary) -> Self {
        self.color_dictionary = dictionary;

        self
    }
}

impl RandomColor {
    /// Generates a random color and returns it as an HSV array.
    pub fn random_hsv_array<R: Rng + ?Sized>(&self, rng: &mut R) -> [u32; 3] {
        let (h, s, b) = self.generate_color(rng);

        [h as u32, s as u32, b as u32]
    }

    /// Generates a random color and returns it as an RGB string.
    pub fn random_rgb_string<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        let (h, s, b) = self.generate_color(rng);
        let rgb = self.hsv_to_rgb(h, s, b);

        format!("rgb({}, {}, {})", rgb[0], rgb[1], rgb[2])
    }

    /// Generates a random color and returns it as an RGBA string.
    pub fn random_rgba_string<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        let (h, s, b) = self.generate_color(rng);
        let rgb = self.hsv_to_rgb(h, s, b);
        let a: f32 = match self.alpha {
            Some(alpha) => alpha,
            None => rng.random_range(0.0..=1.0),
        };

        format!("rgba({}, {}, {}, {})", rgb[0], rgb[1], rgb[2], a)
    }

    /// Generates a random color and returns it as an RGB array.
    pub fn random_rgb_array<R: Rng + ?Sized>(&self, rng: &mut R) -> [u8; 3] {
        let (h, s, b) = self.generate_color(rng);

        self.hsv_to_rgb(h, s, b)
    }

    /// Generates a random color and returns it as an RGB array.
    pub fn random_rgba_array<R: Rng + ?Sized>(&self, rng: &mut R) -> [u8; 4] {
        let (h, s, b) = self.generate_color(rng);
        let rgb: [u8; 3] = self.hsv_to_rgb(h, s, b);

        let alpha = match self.alpha {
            Some(alpha) => (alpha * 255.0).round() as u8,
            None => rng.random_range(0..=255),
        };

        [rgb[0], rgb[1], rgb[2], alpha]
    }

    /// Generates a random color and returns it as a `f32` RGB array.
    pub fn random_f32_rgb_array<R: Rng + ?Sized>(&self, rng: &mut R) -> [f32; 3] {
        let (h, s, b) = self.generate_color(rng);
        let rgb: [u8; 3] = self.hsv_to_rgb(h, s, b);

        [
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
        ]
    }

    /// Generates a random color and returns it as an `f32` RGBA array.
    pub fn random_f32_rgba_array<R: Rng + ?Sized>(&self, rng: &mut R) -> [f32; 4] {
        let (h, s, b) = self.generate_color(rng);
        let rgb: [u8; 3] = self.hsv_to_rgb(h, s, b);

        let alpha: f32 = match self.alpha {
            Some(alpha) => alpha,
            None => rng.random_range(0.0..=1.0),
        };

        [
            rgb[0] as f32 / 255.0,
            rgb[1] as f32 / 255.0,
            rgb[2] as f32 / 255.0,
            alpha,
        ]
    }

    /// Generates a random color and returns it as an HSL string.
    pub fn random_hsl_string<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        let (h, s, b) = self.generate_color(rng);
        let hsv = self.hsv_to_hsl(h, s, b);

        format!("hsl({}, {}%, {}%)", hsv[0], hsv[1], hsv[2])
    }

    /// Generates a random color and returns it as an HSLA string.
    pub fn random_hsla_string<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        let (h, s, b) = self.generate_color(rng);
        let hsv = self.hsv_to_hsl(h, s, b);
        let a: f32 = match self.alpha {
            Some(alpha) => alpha,
            None => rng.random_range(0.0..=1.0),
        };

        format!("hsla({}, {}%, {}%, {})", hsv[0], hsv[1], hsv[2], a)
    }

    /// Generates a random color and returns it as an HSL array.
    pub fn random_hsl_array<R: Rng + ?Sized>(&self, rng: &mut R) -> [u32; 3] {
        let (h, s, b) = self.generate_color(rng);

        self.hsv_to_hsl(h, s, b)
    }

    /// Generates a random color and returns it as a hex string.
    pub fn random_hex<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        let (h, s, b) = self.generate_color(rng);
        let [r, g, b] = self.hsv_to_rgb(h, s, b);

        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    /// Generates a random color based on the settings.
    fn generate_color<R: Rng + ?Sized>(&self, rng: &mut R) -> (i64, i64, i64) {
        let h = self.pick_hue(rng);
        let s = self.pick_saturation(rng, &h);
        let b = self.pick_brightness(rng, &h, &s);

        (h, s, b)
    }

    /// Picks a random hue based on the hue setting.
    fn pick_hue<R: Rng + ?Sized>(&self, rng: &mut R) -> i64 {
        match self.hue {
            None => rng.random_range(0..=360),
            Some(ref gamut) => {
                let color = self.color_dictionary.get_color_from_gamut(gamut);
                self.random_within(rng, color.range[0], color.range[1])
                    .rem_euclid(360)
            }
        }
    }

    /// Picks a random saturation value based on the hue and luminosity setting.
    ///
    /// Parameters:
    /// * `hue`: The hue of the color.
    fn pick_saturation<R: Rng + ?Sized>(&self, rng: &mut R, hue: &i64) -> i64 {
        if self.hue == Some(Gamut::Monochrome) {
            return 0;
        }

        let s_range: (i64, i64) = self.color_dictionary.get_saturation_range(hue);

        let s_min = s_range.0;
        let s_max = s_range.1;

        match self.luminosity {
            Some(Luminosity::Random) => rng.random_range(0..100),
            Some(Luminosity::Bright) => self.random_within(rng, 55, s_max),
            Some(Luminosity::Dark) => rng.random_range(s_max - 10..s_max),
            Some(Luminosity::Light) => self.random_within(rng, s_min, 55),
            _ => self.random_within(rng, s_min, s_max),
        }
    }

    /// Picks a random brightness value based on the hue and saturation, as well
    /// as the luminosity setting.
    ///
    /// Parameters:
    /// * `hue`: The hue of the color.
    /// * `saturation`: The saturation of the color.
    fn pick_brightness<R: Rng + ?Sized>(&self, rng: &mut R, hue: &i64, saturation: &i64) -> i64 {
        let b_min = self.color_dictionary.get_minimum_value(hue, saturation);
        let b_max = 100;

        match self.luminosity {
            Some(Luminosity::Random) => rng.random_range(0..100),
            Some(Luminosity::Light) => self.random_within(rng, (b_max + b_min) / 2, b_max),
            Some(Luminosity::Dark) => rng.random_range(b_min..b_min + 20),
            _ => self.random_within(rng, b_min, b_max),
        }
    }

    /// Generates a random i64 within the given range.
    ///
    /// This function first ensures that `min` is less than or equal to `max`.
    /// If `min` is equal to `max`, it increments `max` by 1 to ensure that the
    /// range is not empty.
    ///
    /// Parameters:
    /// * `min`: The minimum value of the range.
    /// * `max`: The maximum value of the range.
    fn random_within<R: Rng + ?Sized>(&self, rng: &mut R, mut min: i64, mut max: i64) -> i64 {
        if min > max {
            std::mem::swap(&mut min, &mut max);
        }

        if min == max {
            max += 1;
        }

        rng.random_range(min..max)
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
        let k = (2.0 - s) * v;
        let d = if k > 1.0 { 2.0 - k } else { k };
        let s_l = if d > 0.0 { s * v / d } else { 0.0 };

        [
            h as u32,
            (s_l * 100.0).round() as u32,
            (k / 2.0 * 100.0).round() as u32,
        ]
    }
}

#[cfg(feature = "rgb_support")]
mod rgb_support {
    use rand::Rng;
    use rand::distr::Distribution;
    use rgb::{Rgb, Rgba};

    use crate::RandomColor;

    impl Distribution<Rgb<u8>> for RandomColor {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rgb<u8> {
            let [r, g, b] = self.random_rgb_array(rng);

            Rgb { r, g, b }
        }
    }
    impl Distribution<Rgba<u8>> for RandomColor {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rgba<u8> {
            let [r, g, b, a] = self.random_rgba_array(rng);

            Rgba { r, g, b, a }
        }
    }
}
#[cfg(feature = "palette_support")]
mod palette_support {
    use palette::{Srgb, Srgba};
    use rand::Rng;
    use rand::distr::Distribution;

    use crate::RandomColor;

    impl Distribution<Srgb> for RandomColor {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Srgb {
            let [red, green, blue] = self.random_f32_rgb_array(rng);

            Srgb::new(red, green, blue)
        }
    }
    impl Distribution<Srgba> for RandomColor {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Srgba {
            let [red, green, blue, alpha] = self.random_f32_rgba_array(rng);

            Srgba::new(red, green, blue, alpha)
        }
    }
}

#[cfg(feature = "ecolor_support")]
mod ecolor_support {
    use ecolor::{Color32, Rgba};
    use rand::Rng;
    use rand::distr::Distribution;

    use crate::RandomColor;

    impl Distribution<Color32> for RandomColor {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color32 {
            let [r, g, b, a] = self.random_rgba_array(rng);

            Color32::from_rgba_unmultiplied(r, g, b, a)
        }
    }
    impl Distribution<Rgba> for RandomColor {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rgba {
            let [r, g, b, a] = self.random_f32_rgba_array(rng);

            Rgba::from_rgba_unmultiplied(r, g, b, a)
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng as _;
    use rand::rngs::SmallRng;

    use super::*;

    fn blue_light() -> RandomColor {
        RandomColor::new()
            .hue(Gamut::Blue)
            .luminosity(Luminosity::Light)
            .alpha(1.0)
    }
    fn rng() -> SmallRng {
        SmallRng::seed_from_u64(42)
    }

    #[test]
    fn generates_different_colors_using_the_same_instance() {
        let rc = RandomColor::new();
        let mut rng = rng();

        assert_ne!(
            rc.random_rgb_string(&mut rng),
            rc.random_rgb_string(&mut rng)
        );
    }

    #[test]
    fn generates_color_as_hsv_array() {
        assert_eq!(blue_light().random_hsv_array(&mut rng()), [242, 31, 99]);
    }

    #[test]
    fn generates_color_as_rgb_string() {
        assert_eq!(
            blue_light().random_rgb_string(&mut rng()),
            "rgb(176, 174, 252)"
        );
    }

    #[test]
    fn generates_color_as_rgba_string() {
        assert_eq!(
            blue_light().random_rgba_string(&mut rng()),
            "rgba(176, 174, 252, 1)"
        );
    }

    #[test]
    fn generates_color_as_rgb_array() {
        assert_eq!(blue_light().random_rgb_array(&mut rng()), [176, 174, 252]);
    }

    #[test]
    fn generates_color_as_f32_rgb_array() {
        assert_eq!(
            blue_light().random_f32_rgb_array(&mut rng()),
            [0.6901961, 0.68235296, 0.9882353]
        );
    }

    #[test]
    fn generates_color_as_f32_rgba_array() {
        assert_eq!(
            blue_light().random_f32_rgba_array(&mut rng()),
            [0.6901961, 0.68235296, 0.9882353, 1.0]
        );
    }

    #[test]
    fn generates_color_as_hsl_string() {
        assert_eq!(
            blue_light().random_hsl_string(&mut rng()),
            "hsl(242, 94%, 84%)"
        );
    }

    #[test]
    fn generates_color_as_hsla_string() {
        assert_eq!(
            blue_light().random_hsla_string(&mut rng()),
            "hsla(242, 94%, 84%, 1)"
        );
    }

    #[test]
    fn generates_color_as_hsl_array() {
        assert_eq!(blue_light().random_hsl_array(&mut rng()), [242, 94, 84]);
    }

    #[test]
    fn generates_color_as_hex() {
        assert_eq!(blue_light().random_hex(&mut rng()), "#b0aefc");
    }

    #[test]
    fn to_hex_is_rrggbb() {
        let rc = blue_light();

        let hex = rc.random_hex(&mut rng());
        let [r, g, b] = rc.random_rgb_array(&mut rng());

        assert_eq!(hex, format!("#{:02x}{:02x}{:02x}", r, g, b));
    }

    #[test]
    fn single_digit_hex_are_padded_by_to_two_chars() {
        let rc = RandomColor::new().luminosity(Luminosity::Dark);

        let mut rng = SmallRng::seed_from_u64(5);

        assert_eq!(rc.random_hex(&mut rng), "#207204");
    }

    #[test]
    fn hsv_and_hsl_stay_in_range_for_every_option() {
        let mut rng = rng();

        for gamut in Gamut::ALL {
            for luminosity in Luminosity::ALL {
                let rc = RandomColor::new().hue(gamut).luminosity(luminosity);

                for _ in 0..1000 {
                    let [h, s, v] = rc.random_hsv_array(&mut rng);
                    assert!(
                        h < 360 && s <= 100 && v <= 100,
                        "{gamut:?} {luminosity:?} hsv {h} {s} {v}"
                    );

                    let [h, s, l] = rc.random_hsl_array(&mut rng);
                    assert!(
                        h < 360 && s <= 100 && l <= 100,
                        "{gamut:?} {luminosity:?} hsl {h} {s} {l}"
                    );
                }
            }
        }
    }

    #[test]
    fn monochrome_is_gray() {
        let mut rng = rng();

        for luminosity in Luminosity::ALL {
            let rc = RandomColor::new()
                .hue(Gamut::Monochrome)
                .luminosity(luminosity);

            for _ in 0..1000 {
                let [r, g, b] = rc.random_rgb_array(&mut rng);
                assert!(r == g && g == b, "{luminosity:?} rgb {r} {g} {b}");
            }
        }
    }

    #[test]
    fn converts_hsv_to_hsl() {
        let rc = RandomColor::new();

        assert_eq!(rc.hsv_to_hsl(0, 0, 0), [0, 0, 0]);
        assert_eq!(rc.hsv_to_hsl(0, 0, 100), [0, 0, 100]);
        assert_eq!(rc.hsv_to_hsl(120, 100, 100), [120, 100, 50]);
        assert_eq!(rc.hsv_to_hsl(0, 50, 100), [0, 100, 75]);
        assert_eq!(rc.hsv_to_hsl(242, 31, 99), [242, 94, 84]);
    }

    #[cfg(feature = "rgb_support")]
    mod rgb_support {
        use rand::distr::Distribution as _;

        use super::*;

        #[test]
        fn samples_rgb_from_rgb_crate() {
            let sampled: rgb::Rgb<u8> = blue_light().sample(&mut rng());

            assert_eq!(sampled, rgb::Rgb::new(176, 174, 252));
        }

        #[test]
        fn samples_rgba_from_rgb_crate() {
            let rc = blue_light().alpha(0.69);

            let sampled: rgb::Rgba<u8> = rc.sample(&mut rng());

            assert_eq!(sampled, rgb::Rgba::new(176, 174, 252, 176));
        }
    }

    #[cfg(feature = "palette_support")]
    mod palette_support {
        use rand::distr::Distribution as _;

        use super::*;

        #[test]
        fn samples_srgba_from_palette_crate() {
            let sampled: palette::Srgba = blue_light().sample(&mut rng());

            assert_eq!(
                sampled.into_components(),
                (0.6901961, 0.68235296, 0.9882353, 1.0)
            );
        }

        #[test]
        fn samples_srgb_from_palette_crate() {
            let sampled: palette::Srgb = blue_light().sample(&mut rng());

            assert_eq!(
                sampled.into_components(),
                (0.6901961, 0.68235296, 0.9882353)
            );
        }
    }

    #[cfg(feature = "ecolor_support")]
    mod ecolor_support {
        use ecolor::{Color32, Rgba};
        use rand::distr::Distribution as _;

        use super::*;

        #[test]
        fn samples_color32_from_ecolor_crate() {
            let sampled: Color32 = blue_light().sample(&mut rng());

            assert_eq!(sampled.to_array(), [176, 174, 252, 255]);
        }

        #[test]
        fn samples_rgba_from_ecolor_crate() {
            let sampled: Rgba = blue_light().sample(&mut rng());

            assert_eq!(sampled.to_array(), [0.6901961, 0.68235296, 0.9882353, 1.0]);
        }
    }
}
