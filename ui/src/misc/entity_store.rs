use ::std::{cell::RefCell, collections::BTreeMap, marker::PhantomData};

#[derive(Debug, ::serde::Deserialize, ::serde::Serialize)]
pub struct List<T> {
    inner: BTreeMap<usize, RefCell<T>>,
    next_key: usize,
}

#[derive(Debug, ::serde::Deserialize, ::serde::Serialize)]
pub struct Index<T> {
    inner: usize,
    type_bound: PhantomData<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
            next_key: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Index<T> {
        let result = Index::new(self.next_key);
        self.inner.insert(self.next_key, RefCell::new(item));
        self.next_key += 1;
        result
    }

    pub fn delete(&mut self, idx: Index<T>) {
        self.inner.remove(&idx.inner);
    }

    pub fn iter(&self) -> impl Iterator<Item = &RefCell<T>> {
        self.inner.values()
    }

    pub(crate) fn enumerate(&self) -> impl Iterator<Item = (Index<T>, &RefCell<T>)> {
        self.inner.iter().map(|(&i, t)| (Index::new(i), t))
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
        &self.inner[&index.inner]
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
