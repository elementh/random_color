struct Color {
    range: [i32;2],
    lower_bounds: Vec<[i32;2]>,
    saturation_range: [i32;2],
    value_range: [i32;2],
}
impl Color {
    pub fn new(range: [i32;2], lower_bounds: Vec<[i32;2]>) -> Color {
        
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
    pub fn new() -> ColorDictionary {
        ColorDictionary {
            monochrome: Color::new([0, 0], vec![[0,0], [100,0]]),
            red: Color::new(
                [-26,18], vec![[20,100],[30,92],[40,89],[50,85],[60,78],[70,70],[80,60],[90,55],[100,50]]),
            orange: Color::new(
                [19,46],
                vec![[20,100],[30,93],[40,88],[50,86],[60,85],[70,70],[100,70]]),
            yellow: Color::new(
                [47,62],
                vec![[25,100],[40,94],[50,89],[60,86],[70,84],[80,82],[90,80],[100,75]]),
            green: Color::new(
                [63,178],
                vec![[30,100],[40,90],[50,85],[60,81],[70,74],[80,64],[90,50],[100,40]]),
            blue: Color::new(
                [179, 257],
                vec![[20,100],[30,86],[40,80],[50,74],[60,60],[70,52],[80,44],[90,39],[100,35]]),
            purple: Color::new(
                [258, 282],
                vec![[20,100],[30,87],[40,79],[50,70],[60,65],[70,59],[80,52],[90,45],[100,42]]),
            pink: Color::new(
                [283, 334],
                vec![[20,100],[30,90],[40,86],[60,84],[80,80],[90,75],[100,73]]),
        }
    }
    fn get_color(&self, hue: &i32) -> &Color {
        let min = &334; // I don't understand this
        let max = &360;
        if hue >= min && hue <= max {
            let hue = hue - 360;
        }
        &self.monochrome
    }
}
