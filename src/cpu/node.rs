use math::transform::Transform;

#[derive(Clone)]
pub struct Node {
    pub name: Option<String>,
    pub children: Vec<usize>,
    pub skin: Option<usize>,
    pub mesh: Option<usize>,
    pub transform: Transform,
}

impl Node {
    pub fn new() -> Self {
        Self {
            name: None,
            children: Vec::new(),
            skin: None,
            mesh: None,
            transform: Transform::DEFAULT,
        }
    }

    pub fn with_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn with_mesh(&mut self, index: usize) {
        self.mesh = Some(index);
    }

    pub fn with_skin(&mut self, index: usize) {
        self.skin = Some(index);
    }

    pub fn with_children(&mut self, children: Vec<usize>) {
        self.children = children;
    }
}
