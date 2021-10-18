use super::{core::Fraction, dt_fraction::DtFraction, step::Step, Abstraction, Variant};

#[derive(Default)]
pub struct Collection<const N: usize, const D: usize>(
    pub(in super::super) Vec<Variant<DtFraction<N, D>>>,
);

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
    pub(in crate::core::integration_step) const fn empty() -> Self {
        Self(Vec::new())
    }

    pub(in crate::core::integration_step) fn generalize(self) -> Generic {
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

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Generic {
    pub(in crate::core::integration_step) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub(in crate::core::integration_step) fn abstraction_iter_for<'a>(
        &'a self,
        step: &'a Step,
    ) -> impl Iterator<Item = Abstraction<'a>> {
        self.inner
            .iter()
            .map(move |variant| variant.abstraction_scaled_for(step, self.fraction))
    }
}
