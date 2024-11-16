use std::collections::HashMap;

use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct BusSingleton {
    base: Base<Object>,

    wires: HashMap<String, bool>,
}

#[godot_api]
impl BusSingleton {
    #[func]
    fn set_wire(&mut self, id: String, value: bool) {
        self.wires.insert(id, value);
    }
}
