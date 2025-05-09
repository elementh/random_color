/// The luminosity of the color.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Luminosity {
    #[default]
    Random,
    Bright,
    Light,
    Dark,
}
