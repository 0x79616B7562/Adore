pub fn abort() {
    crate::window::abort();
}

pub fn input_mut() -> &'static mut crate::window::Input {
    crate::window::input_mut()
}

pub fn input() -> &'static crate::window::Input {
    crate::window::input()
}

pub fn set_title(title: impl Into<String>) {
    crate::window::raw().set_title(title.into().as_str());
}
