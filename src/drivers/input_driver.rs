use std::{cell::Ref, rc::Rc};

use minifb::{Key, Window};

struct KeyboardHandler {
    pub window: Rc<Window>,
}
