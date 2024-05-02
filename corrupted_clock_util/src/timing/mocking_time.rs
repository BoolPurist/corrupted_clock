use std::{cell::RefCell, rc::Rc};

use chrono::TimeDelta;

use super::{TimeImpl, UtcDateTime};

#[derive(Default, Clone, Debug)]
pub struct MockTimeImpl(Rc<RefCell<UtcDateTime>>);

impl TimeImpl for MockTimeImpl {
    fn now(&self) -> UtcDateTime {
        self.0.borrow().clone()
    }
}

impl MockTimeImpl {
    pub fn new(current_now: UtcDateTime) -> Self {
        let now = Rc::new(RefCell::new(current_now));
        Self(now)
    }

    pub fn add_to_now(&self, to_add: TimeDelta) {
        *self.0.borrow_mut() += to_add;
    }
    pub fn set_now(&self, new_now: UtcDateTime) {
        *self.0.borrow_mut() = new_now;
    }
}
