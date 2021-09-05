use super::import::{Vec3, R32};
use ::std::hash::Hash;

pub struct StartVelocity(pub Vec3); //todo: use Position; not Vec3

impl Hash for StartVelocity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let vec = self.0;
        // todo: it would be nice to have an R32-based Vec3, so we do not have to do this
        // conversion here:
        R32::new(vec.x).unwrap().hash(state);
        R32::new(vec.y).unwrap().hash(state);
        R32::new(vec.z).unwrap().hash(state);
    }
}
