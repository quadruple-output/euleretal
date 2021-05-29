use super::import::Vec3;
use ::std::{any::TypeId, collections::hash_map::DefaultHasher, hash::Hash};

pub trait AccelerationField: Send + Sync + 'static {
    fn value_at(&self, pos: Vec3) -> Vec3;

    fn label(&self) -> String;

    fn hash(&self, state: &mut DefaultHasher) {
        TypeId::of::<Self>().hash(state);
    }
}
