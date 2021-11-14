use std::cell::RefCell;
use std::rc::Rc;

pub type Obj<T> = Rc<RefCell<T>>;
