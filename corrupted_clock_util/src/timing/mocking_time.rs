use std::{cell::RefCell, rc::Rc};

use super::{TimeImpl, UtcDateTime};

pub struct MockNowTimeAccessor(Rc<RefCell<UtcDateTime>>);

impl MockNowTimeAccessor {
    pub fn set_now(&mut self, new_now: UtcDateTime) {
        *self.0.borrow_mut() = new_now;
    }

    pub fn now(&self) -> UtcDateTime {
        self.0.borrow().clone()
    }
}

#[derive(Default)]
pub struct MockTimeImpl(Rc<RefCell<UtcDateTime>>);

impl TimeImpl for MockTimeImpl {
    fn now(&self) -> UtcDateTime {
        self.0.borrow().clone()
    }
}

impl MockTimeImpl {
    pub fn new(current_now: UtcDateTime) -> (Self, MockNowTimeAccessor) {
        let now = Rc::new(RefCell::new(current_now));
        let accessor = MockNowTimeAccessor(Rc::clone(&now));
        (Self(now), accessor)
    }
}
