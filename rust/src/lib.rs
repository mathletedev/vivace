pub mod bus_singleton;
pub mod chip;
pub mod common;

use bus_singleton::BusSingleton;
use godot::{classes::Engine, prelude::*};

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton()
                .register_singleton(StringName::from("BusSingleton"), BusSingleton::new_alloc());
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let bus_name = StringName::from("BusSingleton");

            let bus = engine.get_singleton(bus_name.clone()).unwrap();

            engine.unregister_singleton(bus_name);
            bus.free();
        }
    }
}
