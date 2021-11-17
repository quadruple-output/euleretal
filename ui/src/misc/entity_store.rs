use ::std::{cell::RefCell, collections::BTreeMap, marker::PhantomData};

/// All references to contents of this list are based on Indexes (not `::std::rc::Rc`), because
/// otherwise deserialization would duplicate instances which where just one before serialization.
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize)]
#[serde(transparent)]
pub struct List<T> {
    // I use a BTreeMap instead of a Vec because I can add and remove new entries without
    // invalidating any Indexes.
    inner: BTreeMap<usize, RefCell<T>>,
    #[serde(skip)]
    next_key: usize,
}

#[derive(Debug, ::serde::Deserialize, ::serde::Serialize)]
#[serde(transparent)]
pub struct Index<T> {
    inner: usize,
    #[serde(skip)]
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
        self.provide_next_unused_key();
        debug_assert!(self
            .inner
            .insert(self.next_key, RefCell::new(item))
            .is_none());
        let index_for_pushed = Index::new(self.next_key);
        self.next_key = self.next_key.wrapping_add(1);
        index_for_pushed
    }

    fn provide_next_unused_key(&mut self) {
        let search_start = self.next_key;
        while self.inner.contains_key(&self.next_key) {
            self.next_key = self.next_key.wrapping_add(1);
            assert!(search_start != self.next_key, "List is full. (seriously?)");
        }
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
