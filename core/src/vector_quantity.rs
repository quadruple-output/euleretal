use super::{
    import::{OrderedF32, Vec3},
    Duration, Fraction,
};
use ::std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

/// A 3D-Vector, representing a specific physical quantity (like Velocity, Acceleration, or
/// Position). `VQMulDuration` is the corresponding `VectorQuantity` that results from a
/// multiplication by `Duration`.
#[derive(Clone, Debug, PartialOrd, ::serde::Deserialize, ::serde::Serialize)]
pub struct VectorQuantity<Quantity, VQMulDuration> {
    vector: Vec3,
    // assuming `Quantity` is zero-sized, we would not even need `PhantomData` here, but with
    // `PhantomData` we are on the safe side:
    #[serde(skip)]
    _unit: PhantomData<Quantity>,
    #[serde(skip)]
    _unit_mul_time: PhantomData<VQMulDuration>,
}

// these are work-arounds for rust issue #26925:
// >>>
impl<U: Copy + Sized, VQD: Copy + Sized> Copy for VectorQuantity<U, VQD> {}

impl<U, VQD> PartialEq for VectorQuantity<U, VQD> {
    fn eq(&self, other: &Self) -> bool {
        self.vector == other.vector
    }
}
// <<<

impl<U, VQD> From<Vec3> for VectorQuantity<U, VQD> {
    fn from(vector: Vec3) -> Self {
        Self {
            vector,
            _unit: PhantomData,
            _unit_mul_time: PhantomData,
        }
    }
}

impl<U, VQD> From<VectorQuantity<U, VQD>> for Vec3 {
    fn from(vq: VectorQuantity<U, VQD>) -> Self {
        vq.vector
    }
}

impl<U, VQD> VectorQuantity<U, VQD> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3::new(x, y, z).into()
    }

    pub fn zeros() -> Self {
        Vec3::zeros().into()
    }

    pub fn as_vector(&self) -> &Vec3 {
        &self.vector
    }
}

impl<U, VQD> Hash for VectorQuantity<U, VQD> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        OrderedF32::from(self.vector.x).hash(state);
        OrderedF32::from(self.vector.y).hash(state);
        OrderedF32::from(self.vector.z).hash(state);
    }
}

impl<U, VQD> Add<VectorQuantity<U, VQD>> for VectorQuantity<U, VQD> {
    type Output = Self;

    fn add(self, rhs: VectorQuantity<U, VQD>) -> Self::Output {
        (self.vector + rhs.vector).into()
    }
}

impl<U, VQD> Sub<VectorQuantity<U, VQD>> for VectorQuantity<U, VQD> {
    type Output = Self;

    fn sub(self, rhs: VectorQuantity<U, VQD>) -> Self::Output {
        (self.vector - rhs.vector).into()
    }
}

impl<U, VQD> AddAssign for VectorQuantity<U, VQD> {
    fn add_assign(&mut self, rhs: Self) {
        self.vector += rhs.vector;
    }
}

impl<U, VQD> SubAssign for VectorQuantity<U, VQD> {
    fn sub_assign(&mut self, rhs: Self) {
        self.vector -= rhs.vector;
    }
}

impl<U, VQD> Mul<VectorQuantity<U, VQD>> for f32 {
    type Output = VectorQuantity<U, VQD>;

    fn mul(self, rhs: VectorQuantity<U, VQD>) -> Self::Output {
        (self * rhs.vector).into()
    }
}

impl<U, VQD> Mul<f32> for VectorQuantity<U, VQD> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        (self.vector * rhs).into()
    }
}

impl<U, VQD> Mul<Fraction> for VectorQuantity<U, VQD> {
    type Output = Self;

    fn mul(self, rhs: Fraction) -> Self::Output {
        self * f32::from(rhs)
    }
}

impl<U, VQD> Mul<Duration> for VectorQuantity<U, VQD>
where
    VQD: From<Vec3>,
{
    type Output = VQD;

    fn mul(self, rhs: Duration) -> Self::Output {
        (self.vector * f32::from(rhs)).into()
    }
}

impl<U, VQD> Div<f32> for VectorQuantity<U, VQD> {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        (self.vector / rhs).into()
    }
}
