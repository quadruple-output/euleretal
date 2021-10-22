use crate::{PhysicalQuantityKind, Position, Vec3};

pub trait Contribution<'a> {
    fn sampling_position(&self) -> Position;

    fn kind(&self) -> PhysicalQuantityKind;

    fn vector(&self) -> Option<Vec3>;

    fn contributions_iter(&'a self) -> Box<dyn Iterator<Item = Box<dyn Contribution + 'a>> + 'a>;
}

/*pub struct ContributionIter<'a> {}

impl<'a> Iterator for ContributionIter<'a> {
    type Item = &'a dyn Contribution;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
*/
