use crate::prelude::*;
use std::ops::Deref;

pub struct Duration(pub ChangeTracker<R32>);

impl Deref for Duration {
    type Target = ChangeTracker<R32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
