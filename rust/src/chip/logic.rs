use uuid::Uuid;

pub enum Gate {
    And,
    Or,
    Not,
}

pub struct Logic {
    id: String,

    inputs: Vec<Uuid>,
    outputs: Vec<Uuid>,

    children: Vec<Logic>,
}

impl Default for Logic {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            inputs: vec![],
            outputs: vec![],
            children: vec![],
        }
    }
}

impl Logic {
    pub fn update(&self) {
        for child in &self.children {
            child.update();
        }
    }
}
