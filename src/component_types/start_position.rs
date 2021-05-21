use std::hash::Hash;

use crate::prelude::*;

pub struct StartPosition(pub ChangeTracker<Vec3>);

impl Hash for StartPosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let vec = self.0.get();
        // todo: it would be nice to have an R32-based Vec3, so we do not have to do this
        // conversion here:
        R32::from(vec.x).hash(state);
        R32::from(vec.y).hash(state);
        R32::from(vec.z).hash(state);
    }
}
// todo: impl Deref
