use godot::{
    classes::{Area2D, InputEvent, InputEventMouse, Material, Sprite2D},
    prelude::*,
};

use super::logic::Logic;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct ChipNode {
    #[export]
    outline: Option<Gd<Material>>,

    hovered: bool,
    drag_offset: Option<Vector2>,

    sprite2d: Option<Gd<Sprite2D>>,
    base: Base<Node2D>,

    chip: Logic,
}

#[godot_api]
impl ChipNode {
    fn set_outlined(&mut self, outlined: bool) {
        assert!(self.sprite2d.is_some());
        self.sprite2d.as_mut().unwrap().set_material(if outlined {
            self.outline.clone()
        } else {
            None
        });
    }

    #[func]
    fn on_mouse_entered(&mut self) {
        self.hovered = true;
        self.set_outlined(true);
    }

    #[func]
    fn on_mouse_exited(&mut self) {
        self.hovered = false;
        self.set_outlined(false);
    }
}

#[godot_api]
impl INode2D for ChipNode {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            outline: None,

            hovered: false,
            drag_offset: None,

            sprite2d: None,
            base,

            chip: Logic::default(),
        }
    }

    fn ready(&mut self) {
        self.sprite2d = Some(self.base().get_node_as::<Sprite2D>("Sprite2D"));

        let mut collider = self.base().get_node_as::<Area2D>("Area2D");

        collider.connect(
            "mouse_entered".into(),
            self.base().callable("on_mouse_entered"),
        );
        collider.connect(
            "mouse_exited".into(),
            self.base().callable("on_mouse_exited"),
        );
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouse>() {
            if event.is_action_pressed("mouse_left".into()) {
                self.drag_offset = Some(event.get_position() - self.base().get_position());
                self.set_outlined(false);
            }
            if event.is_action_released("mouse_left".into()) {
                self.drag_offset = None;
                self.set_outlined(true);
            }

            if let Some(drag_offset) = self.drag_offset {
                if self.hovered {
                    self.base_mut()
                        .set_position(event.get_position() - drag_offset);
                }
            }
        }
    }
}
