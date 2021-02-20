pub type ChangeCount = u32;

pub trait TrackedChange {
    fn has_changed(&self, remembered_change_count: &mut ChangeCount) -> bool {
        let current_change_count = self.change_count();
        let has_changed = self.change_count() != *remembered_change_count;
        *remembered_change_count = current_change_count;
        has_changed
    }

    fn change_count(&self) -> ChangeCount;
}

pub struct ChangeTracker<T: PartialEq + Copy> {
    value: T,
    change_count: ChangeCount,
}

impl<T: PartialEq + Copy> ChangeTracker<T> {
    pub fn with(value: T) -> Self {
        Self {
            value,
            change_count: 1,
        }
    }

    pub fn get(&self) -> T {
        self.value
    }

    pub fn set(&mut self, value: T) {
        if value != self.value {
            self.change_count += 1;
            self.value = value;
        }
    }
}

impl<T: PartialEq + Copy> TrackedChange for ChangeTracker<T> {
    fn change_count(&self) -> ChangeCount {
        self.change_count
    }
}
