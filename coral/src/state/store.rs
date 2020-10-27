
use topo::CallId;
use slotmap::{SlotMap, SecondaryMap, DefaultKey,};
use anymap::AnyMap;
use crate::state::hook::StateHook;

pub struct Store {
    state_to_component_map: SlotMap<DefaultKey, CallId>,
    states: AnyMap,
}

impl Store {
    pub fn new() -> Self {
        Self {
            state_to_component_map: SlotMap::new(),
            states: AnyMap::new(),
        }
    }

    pub fn hook<T: 'static>(&mut self, value: T) -> StateHook<T> {
        let key = self.insert(value);
        StateHook::new(key)
    }

    pub fn update<T: 'static>(&mut self, key: DefaultKey, value: T) {
        let x = self.insert(value);
    }

    pub fn insert<T: 'static>(&mut self, value: T) -> DefaultKey {
        let key = self.state_to_component_map.insert(CallId::current());

        if let Some(map) = self.states.get_mut::<SecondaryMap<DefaultKey, T>>() {
            map.insert(key, value);
        } else {
            let mut map = SecondaryMap::new();
            map.insert(key, value);
            self.states.insert(map);
        };

        key
    }

    pub fn get<T: 'static>(&self, key: DefaultKey) -> Option<&T> {
        self.states.get::<SecondaryMap<DefaultKey, T>>()?.get(key)
    }
}
