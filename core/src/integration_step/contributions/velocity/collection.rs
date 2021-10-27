use super::{dt_fraction::DtFraction, step::Step, Abstraction, Contribution, Variant};
use crate::Fraction;

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

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Generic {
    pub(in crate::integration_step) fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub(in crate::integration_step) fn abstraction_iter_for<'slf, 'stp>(
        &'slf self,
        step: &'stp Step,
    ) -> Box<dyn Iterator<Item = Box<dyn Contribution + 'stp>> + 'slf>
    where
        'stp: 'slf,
    {
        let f = self.fraction;
        let iter: std::iter::Cloned<std::slice::Iter<'slf, Variant<DtFraction<1, 1>>>> =
            self.inner.iter().cloned();
        Box::new(iter.map(move |variant| {
            /*
              Why does it have to be so complicated?

              see https://stackoverflow.com/questions/52288980/how-does-the-mechanism-behind-the-creation-of-boxed-traits-work

              and note:
              "Coercions are only applied in coercion site like the return value. [or
              else] no unsized coercion is performed by the compiler."
              [https://stackoverflow.com/questions/65916882/cant-box-a-struct-that-implements-a-trait-as-a-trait-object]
            */
            //todo: tidy up
            let abstraction: Abstraction<'stp> = variant.abstraction_scaled_for(step, f);
            let box_step: Box<Abstraction<'stp>> = Box::new(abstraction);
            let dyn_box: Box<dyn Contribution + 'stp> = box_step;
            dyn_box
        }))
    }
}
