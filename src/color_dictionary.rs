struct Color {
    range: [u32;2],
    lower_bounds: Vec<[u32;2]>,
    saturation_range: [u32;2],
    value_range: [u32;2],
}
impl Color {
    pub fn new(range: [u32;2], lower_bounds: Vec<[u32;2]>) -> Color {
        
        let saturation_range_min = lower_bounds[0][0];
        let saturation_range_max = lower_bounds[lower_bounds.len() - 1][0];
        
        let saturation_range = [saturation_range_min, saturation_range_max];
        // Min = lowerBounds[lowerBounds.length - 1][1],
        // bMax = lowerBounds[0][1];
        
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
}
pub struct ColorDictionary {
    monochrome: Color,
    red: Color,
    orange: Color,
    yellow: Color,
    green: Color,
    blue: Color,
    purple: Color,
    pink: Color,
}
impl ColorDictionary {
    pub fn get_saturation_range(self, hue: &u32) -> [u32;2] {
        let color = self.get_color_info(hue);
        color.saturation_range
    }
    fn get_color_info(self, hue: &u32) -> Color {
        let min = 334;
        let max = 360;
        if hue >= min && hue <= max {
            let hue = hue - 360;
        }
        
        self.yellow
    }
}
