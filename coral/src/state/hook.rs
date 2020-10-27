
use slotmap::DefaultKey;
use std::marker::PhantomData;
use std::fmt::{self, Display, Formatter};

pub struct StateHook<T> {
    key: DefaultKey,
    _phantom: PhantomData<T>,
}

impl<T> Display for StateHook<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", )
    }
}

impl<T> Clone for StateHook<T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            _phantom: PhantomData::<T>,
        }
    }
}

impl<T> Copy for StateHook<T> {}

impl<T> StateHook<T> {
    pub fn new(key: DefaultKey) -> Self {
        Self {
            key,
            _phantom: PhantomData::<T>
        }
    }
}
