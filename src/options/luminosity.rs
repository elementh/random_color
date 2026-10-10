/// The luminosity of the color.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Luminosity {
    #[default]
    Random,
    Bright,
    Light,
    Dark,
}

impl Luminosity {
    pub const ALL: [Self; 4] = [Self::Random, Self::Bright, Self::Light, Self::Dark];
}
