mod pick;
mod color_dictionary;

pub enum RandomColor {
    HSV([u32; 3]),
}
impl RandomColor {
    pub fn new() -> RandomColor{
        let hue = pick::hue();

        let saturation = pick::saturation(&hue);

        let value = pick::value(&hue, &saturation);
        
        let hsv = [hue, saturation, value];
        RandomColor::HSV(hsv)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
