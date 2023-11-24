#[macro_export]
macro_rules! into_mut_subscriber {
    ($sub:expr) => {
        $crate::Rc::new(RefCell::new($sub))
    };
}

#[macro_export]
macro_rules! into_subscriber {
    ($sub:expr) => {
        $crate::Rc::new($sub)
    };
}