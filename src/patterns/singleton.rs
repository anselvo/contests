use std::mem::MaybeUninit;
use std::sync::{Mutex, Once};
use crate::structures::vector::Vec;

/// Implementation of singleton is not that easy in rust and usually require unsafe block
///
/// Avoid global state in general. Instead, construct the object somewhere early (perhaps in main),
/// then pass mutable references to that object into the places that need it. This will usually
/// make your code easier to reason about and doesn't require as much bending over backwards.
/// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
///
/// Official implementation https://docs.rust-embedded.org/book/peripherals/singletons.html
pub struct Singleton {
    instance: Mutex<Vec>,
}

impl Singleton {
    pub fn get_instance() -> &'static Singleton {
        static mut SINGLETON: MaybeUninit<Singleton> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                // Make it
                let singleton = Singleton {
                    instance: Mutex::new(Vec::new()),
                };
                // Store it to the static var, i.e. initialize it
                SINGLETON.write(singleton);
            });

            // Now we give out a shared reference to the data, which is safe to use
            // concurrently.
            SINGLETON.assume_init_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[should_panic]
    fn should_create_only_one_instance() {
        let len = add_elements_to_singleton(5);
        assert_eq!(len, 5);

        let len = add_elements_to_singleton(3);
        assert_eq!(len, 8);
    }

    fn add_elements_to_singleton(number: usize) -> usize {
        // when the guard goes out of scope (function), the mutex will be unlocked.
        let mut one = Singleton::get_instance().instance.lock().unwrap();
        for i in 0..number {
            one.push(i);
        }
        return one.len().clone();
    }
}
