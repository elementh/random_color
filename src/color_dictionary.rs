#[derive(Debug, PartialEq)]
pub struct Color {
    pub range: [i32; 2],
    pub lower_bounds: Vec<[i32; 2]>,
    pub saturation_range: [i32; 2],
    pub value_range: [i32; 2],
}

impl Color {
    pub fn new(range: [i32; 2], lower_bounds: Vec<[i32; 2]>) -> Color {

        let saturation_range_min = lower_bounds[0][0];
        let saturation_range_max = lower_bounds[lower_bounds.len() - 1][0];

        let saturation_range = [saturation_range_min, saturation_range_max];

        let value_range_min = lower_bounds[lower_bounds.len() - 1][1];
        let value_range_max = lower_bounds[0][1];

        let value_range = [value_range_min, value_range_max];

        Color {
            range: range,
            lower_bounds: lower_bounds,
            saturation_range: saturation_range,
            value_range: value_range,
        }
    }
    pub fn has_between_range(&self, hue: &i32) -> bool {
        if hue >= &self.range[0] && hue <= &self.range[1] {
            (true)
        } else {
            (false)
        }
    }
}

pub struct ColorDictionary {
    pub monochrome: Color,
    pub red: Color,
    pub orange: Color,
    pub yellow: Color,
    pub green: Color,
    pub blue: Color,
    pub purple: Color,
    pub pink: Color,
}
impl ColorDictionary {
    pub fn new() -> ColorDictionary {
        ColorDictionary {
            monochrome: Color::new([0, 0], vec![[0, 0], [100, 0]]),
            red: Color::new(
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
            orange: Color::new(
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
            yellow: Color::new(
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
            green: Color::new(
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
            blue: Color::new(
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
            purple: Color::new(
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
            pink: Color::new(
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

    pub fn get_saturation_range(self, hue: &i32) -> (i32, i32) {
        let color = self.get_color(hue);
        (color.saturation_range[0], color.saturation_range[1])
    }

    pub fn get_minimum_value(self, hue: &i32, saturation: &i32) -> i32 {
        let mut minimum_value = 0;
        let lower_bounds = self.get_color(hue).lower_bounds;
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

        (minimum_value)
    }

    pub fn get_color(self, hue: &i32) -> Color {
        if self.monochrome.has_between_range(&hue) {
            self.monochrome
        } else if self.red.has_between_range(&hue) {
            self.red
        } else if self.orange.has_between_range(&hue) {
            self.orange
        } else if self.yellow.has_between_range(&hue) {
            self.yellow
        } else if self.green.has_between_range(&hue) {
            self.green
        } else if self.blue.has_between_range(&hue) {
            self.blue
        } else if self.purple.has_between_range(&hue) {
            self.purple
        } else {
            self.pink
        }
    }
}
