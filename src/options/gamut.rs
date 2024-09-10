/// The gamut (hue) of the color.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gamut {
    #[default]
    Monochrome,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}
