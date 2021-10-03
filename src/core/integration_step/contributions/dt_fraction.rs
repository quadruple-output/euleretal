use super::core::Fraction;

#[derive(Clone, Copy, Debug)]
pub struct DtFraction(Fraction);

impl From<Fraction> for DtFraction {
    fn from(fraction: Fraction) -> Self {
        Self(fraction)
    }
}

impl From<DtFraction> for Fraction {
    fn from(dt_fraction: DtFraction) -> Self {
        dt_fraction.0
    }
}
