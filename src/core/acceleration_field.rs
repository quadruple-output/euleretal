use super::{Acceleration, Position};
use ::std::{any::TypeId, collections::hash_map::DefaultHasher, hash::Hash};

pub trait AccelerationField: Send + Sync + 'static {
    fn value_at(&self, pos: Position) -> Acceleration;

    fn label(&self) -> String;

    fn hash(&self, state: &mut DefaultHasher) {
        TypeId::of::<Self>().hash(state);
    }
}
