use super::import::R32;

pub type Position = ::parry3d::math::Point<f32>;
pub type Translation = ::parry3d::math::Vector<f32>;

pub trait Hash {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H);
}

impl Hash for Position {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        use ::std::hash::Hash;
        R32::new(self.x).unwrap().hash(state);
        R32::new(self.y).unwrap().hash(state);
        R32::new(self.z).unwrap().hash(state);
    }
}
