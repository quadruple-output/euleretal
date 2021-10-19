use super::{Abstraction, DtFraction, Step, Variant};
use crate::Fraction;

#[derive(Default)]
pub struct Collection<const N: usize, const D: usize>(Vec<Variant<DtFraction<N, D>>>);

pub struct Generic {
    fraction: Fraction,
    inner: Vec<Variant<DtFraction<1, 1>>>,
}

impl<const N: usize, const D: usize> From<Vec<Variant<DtFraction<N, D>>>> for Collection<N, D> {
    fn from(vec: Vec<Variant<DtFraction<N, D>>>) -> Self {
        Self(vec)
    }
}

impl<const N: usize, const D: usize> Collection<N, D> {
    pub(in crate::integration_step) const fn empty() -> Self {
        Self(Vec::new())
    }

    pub(in crate::integration_step) fn generalize(self) -> Generic {
        Generic {
            fraction: Fraction::new(N, D),
            inner: unsafe {
                ::std::mem::transmute::<
                    Vec<Variant<DtFraction<N, D>>>,
                    Vec<Variant<DtFraction<1, 1>>>,
                >(self.0)
            },
        }
    }
}

impl<'a, const N: usize, const D: usize> IntoIterator for &'a Collection<N, D> {
    type Item = &'a Variant<DtFraction<N, D>>;

    type IntoIter = std::slice::Iter<'a, Variant<DtFraction<N, D>>>;
    //type IntoIter = impl IntoIterator<Variant<DtFraction<N, D>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<const N: usize, const D: usize> std::ops::Add<Variant<DtFraction<N, D>>> for Collection<N, D> {
    type Output = Self;

    fn add(self, rhs: Variant<DtFraction<N, D>>) -> Self::Output {
        Self(self.0.into_iter().chain(Some(rhs)).collect())
    }
}

impl Generic {
    pub(in crate::integration_step) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub(in crate::integration_step) fn dt_fraction(&self) -> Fraction {
        self.fraction
    }

    pub(in crate::integration_step) fn abstraction_iter_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> impl Iterator<Item = Abstraction<'a>> {
        self.inner
            .iter()
            .map(move |variant| variant.abstraction_scaled_for(step, self.fraction))
    }
}
