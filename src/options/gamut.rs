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

impl Gamut {
    pub const ALL: [Self; 8] = [
        Self::Monochrome,
        Self::Red,
        Self::Orange,
        Self::Yellow,
        Self::Green,
        Self::Blue,
        Self::Purple,
        Self::Pink,
    ];
}
