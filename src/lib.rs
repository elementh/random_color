mod pick;
mod random_color_options;

use random_color_options::RandomColorOptions;
use random_color_options::ColorDictionary;

#[derive(Clone, Copy, Debug)]
pub struct RandomColor {
    h: i32,
    s: i32,
    v: i32,
}
impl RandomColor {
    pub fn new() -> RandomColorOptions {
        RandomColorOptions {
            hue: None,
            luminosity: None,
            seed: None,
            alpha: Some(1.0),
        }
    }
    pub fn with_hue(mut options: RandomColorOptions, hue: &'static str) -> RandomColorOptions {
        let cd = ColorDictionary::new();
        match hue {
            "monochrome" => options.hue = Some(cd.monochrome),
            "red" => options.hue = Some(cd.red),
            "orange" => options.hue = Some(cd.orange),
            "yellow" => options.hue = Some(cd.yellow),
            "green" => options.hue = Some(cd.green),
            "blue" => options.hue = Some(cd.blue),
            "purple" => options.hue = Some(cd.purple),
            "pink" => options.hue = Some(cd.pink),
            _ => options.hue = None
        }
        (options)
    }
    pub fn with_luminosity(mut options: RandomColorOptions,
                           luminosity: &'static str)
                           -> RandomColorOptions {
        options.luminosity = Some(luminosity);
        (options)
    }
    pub fn with_seed(mut options: RandomColorOptions, seed: i32) -> RandomColorOptions {
        options.seed = Some(seed);
        (options)
    }
    pub fn with_alpha(mut options: RandomColorOptions, alpha: f32) -> RandomColorOptions {
        options.alpha = Some(alpha);
        (options)
    }
    pub fn generate(options: RandomColorOptions) -> RandomColor {

        let hue = pick::hue(&options);

        let saturation = pick::saturation(&options, &hue);

        let value = pick::value(&options, &hue, &saturation);

        RandomColor {
            h: hue,
            s: saturation,
            v: value,
        }
    }
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
