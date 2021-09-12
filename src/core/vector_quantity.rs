use super::{
    import::{OrderedF32, Vec3},
    Duration, Fraction,
};
use ::std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VectorQuantity<Unit, VQMulDuration> {
    vector: Vec3,
    _unit: PhantomData<Unit>, // TODO: do we really need `Unit` as type parameter?
    _unit_mul_time: PhantomData<VQMulDuration>,
}

impl<U: Copy + Sized, VQT: Copy + Sized> Copy for VectorQuantity<U, VQT> {} // this is a work-around for rust issue #26925

impl<U, VQT> From<Vec3> for VectorQuantity<U, VQT> {
    fn from(vector: Vec3) -> Self {
        Self {
            vector,
            _unit: PhantomData,
            _unit_mul_time: PhantomData,
        }
    }
}

impl<U, VQT> From<VectorQuantity<U, VQT>> for Vec3 {
    fn from(vq: VectorQuantity<U, VQT>) -> Self {
        vq.vector
    }
}

impl<U, VQT> VectorQuantity<U, VQT> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self::from(Vec3::new(x, y, z))
    }

    pub fn zeros() -> Self {
        Self::from(Vec3::zeros())
    }

    pub fn as_vector(&self) -> &Vec3 {
        &self.vector
    }
}

impl<U, VQT> Hash for VectorQuantity<U, VQT> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        OrderedF32::from(self.vector.x).hash(state);
        OrderedF32::from(self.vector.y).hash(state);
        OrderedF32::from(self.vector.z).hash(state);
    }
}

impl<U, VQT> Add<VectorQuantity<U, VQT>> for VectorQuantity<U, VQT> {
    type Output = Self;

    fn add(self, rhs: VectorQuantity<U, VQT>) -> Self::Output {
        (self.vector + rhs.vector).into()
    }
}

impl<U, VQT> Sub<VectorQuantity<U, VQT>> for VectorQuantity<U, VQT> {
    type Output = Self;

    fn sub(self, rhs: VectorQuantity<U, VQT>) -> Self::Output {
        (self.vector - rhs.vector).into()
    }
}

impl<U, VQT> AddAssign for VectorQuantity<U, VQT> {
    fn add_assign(&mut self, rhs: Self) {
        self.vector += rhs.vector;
    }
}

impl<U, VQT> SubAssign for VectorQuantity<U, VQT> {
    fn sub_assign(&mut self, rhs: Self) {
        self.vector -= rhs.vector;
    }
}

impl<U, VQT> Mul<VectorQuantity<U, VQT>> for f32 {
    type Output = VectorQuantity<U, VQT>;

    fn mul(self, rhs: VectorQuantity<U, VQT>) -> Self::Output {
        (self * rhs.vector).into()
    }
}

impl<U, VQT> Mul<f32> for VectorQuantity<U, VQT> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        (self.vector * rhs).into()
    }
}

impl<U, VQT> Mul<Fraction> for VectorQuantity<U, VQT> {
    type Output = Self;

    fn mul(self, rhs: Fraction) -> Self::Output {
        self * f32::from(rhs)
    }
}

impl<U, VQT> Mul<Duration> for VectorQuantity<U, VQT>
where
    VQT: From<Vec3>,
{
    type Output = VQT;

    fn mul(self, rhs: Duration) -> Self::Output {
        (self.vector * rhs).into()
    }
}

impl<U, VQT> Div<f32> for VectorQuantity<U, VQT> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        (self.vector / rhs).into()
    }
}
