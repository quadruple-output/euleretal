use std::marker::PhantomData;

pub type ChangeCount = u32;

pub trait TrackedChange {
    /// Checks whether the given remembered change count is older than the
    /// current change count. Sets the given change count to the current change
    /// count, such that the method returns `true` only once when called
    /// repeatedly
    ///
    /// Note that a given change count of `0` will never match the current change
    /// count, such that `true` is returned in this case.
    fn has_changed(&self, remembered_change_count: &mut ChangeCount) -> bool {
        let current_change_count = self.change_count();
        let has_changed = self.change_count() != *remembered_change_count;
        *remembered_change_count = current_change_count;
        has_changed
    }

    /// Returns the current change count. Always > 0.
    fn change_count(&self) -> ChangeCount;
}

pub trait TRead: Sized {}
pub trait TReadWrite: TRead {}
pub struct Read;
pub struct ReadWrite;
impl TRead for Read {}
impl TRead for ReadWrite {}
impl TReadWrite for ReadWrite {}

pub struct ChangeTracker<T, RoRw = ReadWrite>
where
    T: PartialEq + Copy,
    RoRw: TRead,
{
    value: T,
    change_count: ChangeCount,
    dummy: PhantomData<RoRw>,
}

impl<T, RoRw> ChangeTracker<T, RoRw>
where
    T: PartialEq + Copy,
    RoRw: TRead,
{
    pub fn get(&self) -> T {
        self.value
    }

    pub fn copy_read_only(&self) -> ChangeTracker<T, Read> {
        ChangeTracker::<T, Read> {
            value: self.value,
            change_count: self.change_count,
            dummy: PhantomData,
        }
    }
}

impl<T> ChangeTracker<T, ReadWrite>
where
    T: PartialEq + Copy,
{
    pub fn with(value: T) -> ChangeTracker<T, ReadWrite> {
        ChangeTracker::<T, ReadWrite> {
            value,
            change_count: 1, // must be > 0, such that an initial value is always unequal
            dummy: PhantomData,
        }
    }

    pub fn set(&mut self, value: T) {
        if value != self.value {
            self.change_count += 1;
            self.value = value;
        }
    }
}

impl<T, RoRw> TrackedChange for ChangeTracker<T, RoRw>
where
    T: PartialEq + Copy,
    RoRw: TRead,
{
    fn change_count(&self) -> ChangeCount {
        self.change_count
    }
}

// #[test] -- does not compile (which is good)
// fn try_change_read_only_copy() {
//    let a = ChangeTracker::with(1_u32);
//    a.set(2);
//    let b = a.copy_read_only();
//    b.set(3);  <--- no method named `set` found for struct ChangeTracker<u32, Read>
//}
