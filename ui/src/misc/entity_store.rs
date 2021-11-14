use ::std::{cell::RefCell, marker::PhantomData};

/// A List that can only grow, but never shrink. This makes it possible to store indexes into the
/// list without worrying about them becoming invalid.
#[derive(Debug)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Deserialize, ::serde::Serialize)
)]
pub struct List<T> {
    inner: Vec<RefCell<T>>,
}

/// an index into `AppendOnlyList<T>`
#[derive(Debug)]
#[cfg_attr(
    feature = "persistence",
    derive(::serde::Deserialize, ::serde::Serialize)
)]
pub struct Index<T> {
    inner: usize,
    type_bound: PhantomData<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn push(&mut self, item: T) -> Index<T> {
        let result = Index::new(self.inner.len());
        self.inner.push(RefCell::new(item));
        result
    }

    pub fn iter(&self) -> impl Iterator<Item = &RefCell<T>> {
        self.inner.iter()
    }

    pub(crate) fn enumerate(&self) -> impl Iterator<Item = (Index<T>, &RefCell<T>)> {
        self.inner
            .iter()
            .enumerate()
            .map(|(i, t)| (Index::new(i), t))
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ::std::ops::Index<self::Index<T>> for List<T> {
    type Output = RefCell<T>;

    fn index(&self, index: self::Index<T>) -> &Self::Output {
        &self.inner[index.inner]
    }
}

impl<T> Index<T> {
    fn new(n: usize) -> Self {
        Self {
            inner: n,
            type_bound: PhantomData::default(),
        }
    }
}

impl<T> Clone for Index<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            type_bound: PhantomData::default(),
        }
    }
}

impl<T> Copy for Index<T> {}

impl<T> PartialEq for Index<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
