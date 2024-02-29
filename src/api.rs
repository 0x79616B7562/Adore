pub fn abort() {
    crate::window::abort();
}

pub fn input_mut() -> &'static mut crate::window::Input {
    crate::window::input_mut()
}

pub fn input() -> &'static crate::window::Input {
    crate::window::input()
}
