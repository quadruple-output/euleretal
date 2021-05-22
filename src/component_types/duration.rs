use crate::prelude::*;
use std::{hash::Hash, ops::Deref};

#[derive(Clone, Copy)]
pub struct Duration(pub R32);

impl Deref for Duration {
    type Target = R32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for Duration {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
