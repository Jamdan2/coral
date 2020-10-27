
extern crate coral;
extern crate topo;

use std::cell::RefCell;
use coral::state::store::Store;

thread_local! {
    static STORE: RefCell<Store> = RefCell::new(Store::new());
}

pub fn main() {

}

pub fn a() {
    let x = STORE.with(|store| store.borrow_mut().hook(0));

    println!("Got {}", x);
}
