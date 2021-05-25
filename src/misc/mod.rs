#[macro_use]
pub mod fraction;
pub mod obj;

pub mod prelude {
    pub use super::fraction;
    pub use super::fraction::Fraction;
    pub use super::obj::Obj;
}
