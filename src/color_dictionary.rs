use crate::options::Gamut;

/// Color information for a given hue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorInformation {
    /// The range of the hue.
    pub range: [i64; 2],
    /// The lower bounds of the hue.
    pub lower_bounds: Vec<[i64; 2]>,
    /// The saturation range of the hue.
    pub saturation_range: [i64; 2],
    /// The value range of the hue.
    pub value_range: [i64; 2],
}

impl ColorInformation {
    /// Create a new `ColorInformation` instance.
    pub fn new(range: [i64; 2], lower_bounds: Vec<[i64; 2]>) -> Self {
        let saturation_range_min = lower_bounds[0][0];
        let saturation_range_max = lower_bounds[lower_bounds.len() - 1][0];

        let saturation_range = [saturation_range_min, saturation_range_max];

        let value_range_min = lower_bounds[lower_bounds.len() - 1][1];
        let value_range_max = lower_bounds[0][1];

        let value_range = [value_range_min, value_range_max];

        Self {
            range,
            lower_bounds,
            saturation_range,
            value_range,
        }
    }

    /// Check if the given hue is within the range.
    pub fn has_between_range(&self, hue: &i64) -> bool {
        hue >= &self.range[0] && hue <= &self.range[1]
    }
}

/// The color dictionary contains all the color information for the different gamuts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorDictionary {
    /// The color information for the monochrome gamut.
    pub monochrome: ColorInformation,
    /// The color information for the red gamut.
    pub red: ColorInformation,
    /// The color information for the orange gamut.
    pub orange: ColorInformation,
    /// The color information for the yellow gamut.
    pub yellow: ColorInformation,
    /// The color information for the green gamut.
    pub green: ColorInformation,
    /// The color information for the blue gamut.
    pub blue: ColorInformation,
    /// The color information for the purple gamut.
    pub purple: ColorInformation,
    /// The color information for the pink gamut.
    pub pink: ColorInformation,
}

impl ColorDictionary {
    /// Creates a new `ColorDictionary` instance.
    pub fn new() -> ColorDictionary {
        ColorDictionary {
            monochrome: ColorInformation::new([0, 0], vec![[0, 0], [100, 0]]),
            red: ColorInformation::new(
                [-26, 18],
                vec![
                    [20, 100],
                    [30, 92],
                    [40, 89],
                    [50, 85],
                    [60, 78],
                    [70, 70],
                    [80, 60],
                    [90, 55],
                    [100, 50],
                ],
            ),
            orange: ColorInformation::new(
                [19, 46],
                vec![
                    [20, 100],
                    [30, 93],
                    [40, 88],
                    [50, 86],
                    [60, 85],
                    [70, 70],
                    [100, 70],
                ],
            ),
            yellow: ColorInformation::new(
                [47, 62],
                vec![
                    [25, 100],
                    [40, 94],
                    [50, 89],
                    [60, 86],
                    [70, 84],
                    [80, 82],
                    [90, 80],
                    [100, 75],
                ],
            ),
            green: ColorInformation::new(
                [63, 178],
                vec![
                    [30, 100],
                    [40, 90],
                    [50, 85],
                    [60, 81],
                    [70, 74],
                    [80, 64],
                    [90, 50],
                    [100, 40],
                ],
            ),
            blue: ColorInformation::new(
                [179, 257],
                vec![
                    [20, 100],
                    [30, 86],
                    [40, 80],
                    [50, 74],
                    [60, 60],
                    [70, 52],
                    [80, 44],
                    [90, 39],
                    [100, 35],
                ],
            ),
            purple: ColorInformation::new(
                [258, 282],
                vec![
                    [20, 100],
                    [30, 87],
                    [40, 79],
                    [50, 70],
                    [60, 65],
                    [70, 59],
                    [80, 52],
                    [90, 45],
                    [100, 42],
                ],
            ),
            pink: ColorInformation::new(
                [283, 334],
                vec![
                    [20, 100],
                    [30, 90],
                    [40, 86],
                    [60, 84],
                    [80, 80],
                    [90, 75],
                    [100, 73],
                ],
            ),
        }
    }

    /// Get the saturation range for the given hue.
    ///
    /// Parameters:
    /// * `hue`: The hue to get the saturation range for.
    pub fn get_saturation_range(&self, hue: &i64) -> (i64, i64) {
        let color = &self.get_color_from_hue(hue);
        (color.saturation_range[0], color.saturation_range[1])
    }

    /// Get the minimum value for the given hue and saturation.
    ///
    /// Parameters:
    /// * `hue`: The hue to get the minimum value for.
    /// * `saturation`: The saturation to get the minimum value for.
    pub fn get_minimum_value(&self, hue: &i64, saturation: &i64) -> i64 {
        let mut minimum_value = 0;
        let lower_bounds = &self.get_color_from_hue(hue).lower_bounds;
        for i in 0..lower_bounds.len() - 1 {
            let s1 = lower_bounds[i][0];
            let v1 = lower_bounds[i][1];

            let s2 = lower_bounds[i + 1][0];
            let v2 = lower_bounds[i + 1][1];

            if saturation >= &s1 && saturation <= &s2 {
                let m = (v2 - v1) / (s2 - s1);
                let b = v1 - m * s1;

                minimum_value = m * saturation + b;
            }
        }

        minimum_value
    }

    /// Get the color information for the given gamut.
    ///
    /// Parameters:
    /// * `gamut`: The gamut to get the color information for.
    pub fn get_color_from_gamut(&self, gamut: &Gamut) -> &ColorInformation {
        match gamut {
            Gamut::Monochrome => &self.monochrome,
            Gamut::Red => &self.red,
            Gamut::Orange => &self.orange,
            Gamut::Yellow => &self.yellow,
            Gamut::Green => &self.green,
            Gamut::Blue => &self.blue,
            Gamut::Purple => &self.purple,
            Gamut::Pink => &self.pink,
        }
    }

    /// Get the color information for the given hue.
    ///
    /// Parameters:
    /// * `hue`: The hue to get the color information for.
    fn get_color_from_hue(&self, hue: &i64) -> &ColorInformation {
        if self.monochrome.has_between_range(hue) {
            &self.monochrome
        } else if self.red.has_between_range(hue) {
            &self.red
        } else if self.orange.has_between_range(hue) {
            &self.orange
        } else if self.yellow.has_between_range(hue) {
            &self.yellow
        } else if self.green.has_between_range(hue) {
            &self.green
        } else if self.blue.has_between_range(hue) {
            &self.blue
        } else if self.purple.has_between_range(hue) {
            &self.purple
        } else {
            &self.pink
        }
    }
}

impl Default for ColorDictionary {
    fn default() -> Self {
        ColorDictionary::new()
    }
}
