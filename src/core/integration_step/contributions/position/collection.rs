use super::Variant;

#[derive(Default)]
pub struct Collection<const N: usize, const D: usize>(Vec<Variant<N, D>>);

impl<const N: usize, const D: usize> From<Vec<Variant<N, D>>> for Collection<N, D> {
    fn from(vec: Vec<Variant<N, D>>) -> Self {
        Self(vec)
    }
}

impl<const N: usize, const D: usize> Collection<N, D> {
    pub const fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = &Variant<N, D>> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn transmute<const A: usize, const B: usize>(self) -> Collection<A, B> {
        unsafe { ::std::mem::transmute::<Self, Collection<A, B>>(self) }
    }
}

impl<'a, const N: usize, const D: usize> IntoIterator for &'a Collection<N, D> {
    type Item = &'a Variant<N, D>;

    type IntoIter = std::slice::Iter<'a, Variant<N, D>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<const N: usize, const D: usize> std::ops::Add<Variant<N, D>> for Collection<N, D> {
    type Output = Self;

    fn add(self, rhs: Variant<N, D>) -> Self::Output {
        Self(self.0.into_iter().chain(Some(rhs)).collect())
    }
}
