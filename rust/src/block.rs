use godot::{
    classes::{Area2D, InputEvent, InputEventMouse},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Node2D)]
struct Block {
    hovered: bool,
    drag_offset: Option<Vector2>,
    base: Base<Node2D>,
}

#[godot_api]
impl Block {
    #[func]
    fn on_mouse_entered(&mut self) {
        self.hovered = true;
    }

    #[func]
    fn on_mouse_exited(&mut self) {
        self.hovered = false;
    }
}

#[godot_api]
impl INode2D for Block {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            hovered: false,
            drag_offset: None,
            base,
        }
    }

    fn ready(&mut self) {
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
            }
            if event.is_action_released("mouse_left".into()) {
                self.drag_offset = None;
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
