
pub mod store;
pub mod hook;

use std::cell::RefCell;
use slotmap::DefaultKey;
use store::Store;

thread_local! {
    static STORE: RefCell<Store> = RefCell::new(Store::new());
}

pub fn get<T: 'static>(key: DefaultKey) -> Option<&'static T> {
    STORE.with(|s| {
        s.borrow().get(key)
    })
}
