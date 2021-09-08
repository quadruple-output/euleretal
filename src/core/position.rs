use super::import::R32;

pub type Position = ::parry3d::math::Point<f32>;

/// `Position` cannot implement `std::hash::Hash` since we neither own `Hash`, nor `Position`.
/// Therefore, we define our own Trait which serves our Purpose of calculating a hash value.
pub trait AuxHash {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H);
}

impl AuxHash for Position {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        use ::std::hash::Hash;
        R32::new(self.x).unwrap().hash(state);
        R32::new(self.y).unwrap().hash(state);
        R32::new(self.z).unwrap().hash(state);
    }
}
