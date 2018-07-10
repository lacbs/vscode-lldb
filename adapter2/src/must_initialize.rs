use std::ops;

pub enum MustInitialize<T> {
    Initialized(T),
    NotInitialized,
}

pub use self::MustInitialize::Initialized;
pub use self::MustInitialize::NotInitialized;

impl<T> ops::Deref for MustInitialize<T> {
    type Target = T;

    fn deref(&self) -> &T {
        match (self) {
            Initialized(ref r) => r,
            NotInitialized => {
                panic!("Whoops! Something that was supposed to have been initialized at this point, wasn't.")
            }
        }
    }
}

impl<T> ops::DerefMut for MustInitialize<T> {
    fn deref_mut(&mut self) -> &mut T {
        match (self) {
            Initialized(ref mut r) => r,
            NotInitialized => {
                panic!("Whoops! Something that was supposed to have been initialized at this point, wasn't.")
            }
        }
    }
}