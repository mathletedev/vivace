pub mod chip_node;

use godot::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub enum Gate {
    And,
    Not,
}

#[derive(Default)]
pub struct Chip {
    inputs: Vec<Rc<RefCell<bool>>>,
    outputs: Vec<Rc<RefCell<bool>>>,

    gate: Option<Gate>,
    children: Vec<Chip>,
}

impl Chip {
    fn new_bus(size: usize) -> Vec<Rc<RefCell<bool>>> {
        (0..size).map(|_| Rc::new(RefCell::new(false))).collect()
    }

    pub fn from_gate(gate: Gate) -> Self {
        let inputs = match gate {
            Gate::And => Self::new_bus(2),
            Gate::Not => Self::new_bus(1),
        };

        Self {
            inputs,
            outputs: Self::new_bus(1),

            gate: Some(gate),
            children: vec![],
        }
    }

    pub fn update(&self) {
        for child in &self.children {
            child.update();
        }

        if let Some(gate) = &self.gate {
            match gate {
                Gate::And => {
                    *self.outputs[0].borrow_mut() =
                        *self.inputs[0].borrow() && *self.inputs[1].borrow();
                }
                Gate::Not => {
                    *self.outputs[0].borrow_mut() = !*self.inputs[0].borrow();
                }
            }
        }
    }
}
